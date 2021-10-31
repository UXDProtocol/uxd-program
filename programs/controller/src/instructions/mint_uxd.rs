use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use mango::state::PerpAccount;

use crate::mango_program;
use crate::ControllerError;
use crate::Depository;
use crate::State;
use crate::DEPOSITORY_SEED;
use crate::PASSTHROUGH_SEED;
use crate::SLIPPAGE_BASIS;
use crate::STATE_SEED;
use crate::UXD_SEED;

// First iteration
// XXX oki this shit is complicated lets see what all is here...
// XXX gahh this means we need our own redeemable account too...
// this is troublesome... hmm we could theoretically uhh...
// * user gives mint 1 btc-redeemable
// * we call proxy transfer which *burns* the redeemable and sends *us* 1 btc
// * we deposit that 1 btc into the mago account and create a position
// * we mint the amount of uxd that corresponds to the position size
// and then in reverse is like
// * burn the amount of uxd
// * close out a corresponding position size and redeem for coin
// * proxy transfer coin to depository which *mints* redeemable to us
// * transfer redeemable to user
// and in fact we may very well just mint directly to user

// Second iteration
// Take Collateral from the user
// Deposit collateral on Mango (long half)
// Place immediate perp order on mango using our deposited collateral for borrowing (short half)
//   if it does not fill withing slippage, we abort
// Mint equivalent amount of UXD as the position is covering for
// basically what we do is take redeemables from the user, take coin from depository
// send coin to mango, open position, mint uxd to user

#[derive(Accounts)]
pub struct MintUxd<'info> {
    // XXX again we should use approvals so user doesnt need to sign
    pub user: Signer<'info>,
    #[account(seeds = [STATE_SEED], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(
        seeds = [DEPOSITORY_SEED, collateral_mint.key().as_ref()],
        bump
    )]
    pub depository: Box<Account<'info, Depository>>,
    // TODO use commented custom errors in 1.8.0 anchor and 1.8.0 solana mainnet
    #[account(constraint = collateral_mint.key() == depository.collateral_mint_key)] //@ ControllerError::MintMismatchCollateral)]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [PASSTHROUGH_SEED, collateral_mint.key().as_ref()],
        bump
    )]
    pub collateral_passthrough: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_collateral.mint == depository.collateral_mint_key //@ ControllerError::UnexpectedCollateralMint
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_uxd.mint == state.uxd_mint_key //@ ControllerError::InvalidUserUXDAssocTokenAccount
    )]
    pub user_uxd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [UXD_SEED], 
        bump,
        constraint = uxd_mint.key() == state.uxd_mint_key //@ ControllerError::MintMismatchUXD
    )]
    pub uxd_mint: Box<Account<'info, Mint>>,
    // XXX start mango --------------------------------------------------------
    // MangoGroup that this mango account is for
    pub mango_group: AccountInfo<'info>,
    // Mango Account of the Depository Record
    #[account(mut)]
    pub mango_account: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,
    // The spot/perp market for `coin_mint` on mango, and the associated required accounts
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,
    // XXX end mango ----------------------------------------------------------
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    //
    pub rent: Sysvar<'info, Rent>,
}

// HANDLER
pub fn handler(ctx: Context<MintUxd>, collateral_amount: u64, slippage: u32) -> ProgramResult {
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_collateral.to_account_info(),
            to: ctx.accounts.collateral_passthrough.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, collateral_amount)?;

    msg!("controller: mint uxd [Deposit Mango CPI]");
    let collateral_mint_key = ctx.accounts.collateral_mint.key();

    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        DEPOSITORY_SEED,
        collateral_mint_key.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];
    mango_program::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_signer_seeds),
        collateral_amount,
    )?;

    // msg!("controller: mint uxd [calculation for perp position opening]");
    let collateral_amount_native_unit = I80F48::from_num(collateral_amount);

    let mango_account = MangoAccount::load_checked(
        &ctx.accounts.mango_account,
        ctx.accounts.mango_program.key,
        &ctx.accounts.mango_group.key,
    )?;
    let mango_group =
        MangoGroup::load_checked(&ctx.accounts.mango_group, ctx.accounts.mango_program.key)?;
    let mango_cache = MangoCache::load_checked(
        &ctx.accounts.mango_cache,
        ctx.accounts.mango_program.key,
        &mango_group,
    )?;
    // PERP
    let perp_market_index = mango_group
        .find_perp_market_index(ctx.accounts.mango_perp_market.key)
        .unwrap();
    let taker_fee = mango_group.perp_markets[perp_market_index].taker_fee;
    // base and quote details
    let base_decimals = mango_group.tokens[perp_market_index].decimals;
    let base_unit = I80F48::from_num(10u64.pow(base_decimals.into()));
    let base_lot_size = I80F48::from_num(mango_group.perp_markets[perp_market_index].base_lot_size);
    let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
    let quote_unit = I80F48::from_num(10u64.pow(quote_decimals.into()));
    let quote_lot_size =
        I80F48::from_num(mango_group.perp_markets[perp_market_index].quote_lot_size);
    msg!("-----");
    msg!("base_unit {}", base_unit);
    msg!("base_lot_size {}", base_lot_size);
    msg!("quote_unit {}", quote_unit);
    msg!("quote_lot_size {}", quote_lot_size);
    msg!("-----");

    // Slippage calulation
    // price: I80F48 - native quote per native base - THIS IS IMPORTANT - Equivalent to price per lamport for sol, or price per satoshi
    let price = mango_cache.price_cache[perp_market_index].price;
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
    let slippage_amount = price.checked_mul(slippage_ratio).unwrap();
    let price_adjusted = price.checked_sub(slippage_amount).unwrap();
    msg!("price (native quote per native base): {}", price);
    msg!("price_adjusted (after slippage calculation): {}", price_adjusted); // This is the UI price, in UI amount
    
    // 0.191875 * 1000000 / 100 <====> perp_price * quote_unit / quote_lot_size == base_lot_native_unit

    // == 1918.75 USDC native units  <=> 0.001919 usdc par SOL lot
    // 1 sol lot == 10000000 lamports

    // This is the price of one base lot in quote native units
    let base_lot_price_in_quote_lot_unit = price_adjusted 
        .checked_mul(quote_unit).unwrap() // to quote native amount
        .checked_div(base_unit).unwrap() // price for 1 decimal unit (1 satoshi for btc for instance)
        .checked_mul(base_lot_size).unwrap() // price for a lot (100 sat for btc for instance)
        .checked_div(quote_lot_size).unwrap(); // price for a lot in quote_lot_unit
    // let base_lot_price_in_quote_native_units = price_adjusted // price for one native unit of BASE in native units of QUOTE
    //     .checked_mul(base_lot_size).unwrap(); // multiply by the number of native units of BASE in a lot
    // let base_lot_price_in_quote_lot_unit = base_lot_price_in_quote_native_units.checked_mul(quote_lot_size).unwrap();
    msg!("base_lot_price_in_quote_native_units: {}", base_lot_price_in_quote_lot_unit);

    //XXX assuming USDC and UXD have same decimals, need to fix
    let quantity_base_lot = collateral_amount_native_unit.checked_div(base_lot_size).unwrap();
    msg!("quantity_base_lot: {}", quantity_base_lot);

    // We now calculate the amount pre perp opening, in order to define after if it got 100% filled or not
    let pre_position = {
        let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];    
        msg!("-----");
        msg!("base_position {}", perp_account.base_position);
        msg!("quote_position {}", perp_account.quote_position);
        msg!("taker_base {}", perp_account.taker_base);
        msg!("taker_quote {}", perp_account.taker_quote);
        msg!("-----");
        perp_account.base_position + perp_account.taker_base
    };

    // Drop ref cause they are also used in the Mango CPI destination
    drop(mango_group);
    drop(mango_cache);
    drop(mango_account);

    let depository_record_bump = Pubkey::find_program_address(
        &[DEPOSITORY_SEED, collateral_mint_key.as_ref()],
        ctx.program_id,
    )
    .1;
    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        DEPOSITORY_SEED,
        collateral_mint_key.as_ref(),
        &[depository_record_bump],
    ]];
    // Call Mango CPI
    let order_price = base_lot_price_in_quote_lot_unit.to_num::<i64>();
    let order_quantity = quantity_base_lot.to_num::<i64>();
    msg!("order_price {}", order_price);
    msg!("order_quantity {}", order_quantity);
    mango_program::place_perp_order(
        ctx.accounts
            .into_open_mango_short_perp_context()
            .with_signer(depository_signer_seeds),
        order_price,
        order_quantity,
        0,
        mango::matching::Side::Ask,
        mango::matching::OrderType::ImmediateOrCancel,
        false,
    )?;

    msg!("verify that the order got 100% filled");
    let mango_account = MangoAccount::load_checked(
        &ctx.accounts.mango_account,
        ctx.accounts.mango_program.key,
        ctx.accounts.mango_group.key,
    )?;
    let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
    let post_position = perp_account.base_position + perp_account.taker_base;
    let filled = (post_position - pre_position).abs();
    msg!("-----");
    msg!("base_position {}", perp_account.base_position);
    msg!("quote_position {}", perp_account.quote_position);
    msg!("taker_base {}", perp_account.taker_base);
    msg!("taker_quote {}", perp_account.taker_quote);
    msg!("-----");
    msg!("post_position {}", post_position);
    if !(order_quantity == filled) {
        return Err(ControllerError::PerpPartiallyFilled.into());
    }

    // real execution price (minus the fees)
    let order_price_native_unit = I80F48::from_num(perp_account.taker_quote).checked_mul(quote_lot_size).unwrap();
    let fees = order_price_native_unit.abs() * taker_fee;
    // XXX here it's considering UXD and USDC have same decimals -- FIX LATER
    let uxd_amount = order_price_native_unit - fees;
    msg!("uxd_amount {}", uxd_amount);
    let state_signer_seed: &[&[&[u8]]] = &[&[STATE_SEED, &[ctx.accounts.state.bump]]];
    token::mint_to(
        ctx.accounts
            .into_mint_uxd_context()
            .with_signer(state_signer_seed),
        uxd_amount.to_num(), // deposited_value best vs uxd_amount worse
    )?;

    Ok(())
}

impl<'info> MintUxd<'info> {
    pub fn into_deposit_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            owner_token_account: self.collateral_passthrough.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_open_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::PlacePerpOrder<'info>> {
        let cpi_accounts = mango_program::PlacePerpOrder {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_perp_market: self.mango_perp_market.to_account_info(),
            mango_bids: self.mango_bids.to_account_info(),
            mango_asks: self.mango_asks.to_account_info(),
            mango_event_queue: self.mango_event_queue.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_mint_uxd_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.uxd_mint.to_account_info(),
            to: self.user_uxd.to_account_info(),
            authority: self.state.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
