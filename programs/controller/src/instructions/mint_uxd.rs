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
    let collateral_amount = I80F48::from_num(collateral_amount);

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
    // base and quote details
    let base_decimals = mango_group.tokens[perp_market_index].decimals;
    let base_unit = I80F48::from_num(10u64.pow(base_decimals.into()));
    let base_lot_size = I80F48::from_num(mango_group.perp_markets[perp_market_index].base_lot_size);
    let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
    let quote_unit = I80F48::from_num(10u64.pow(quote_decimals.into()));
    let quote_lot_size =
        I80F48::from_num(mango_group.perp_markets[perp_market_index].quote_lot_size);
    msg!(
        "base decimals: {} - base unit: {} - base lot size: {}",
        base_decimals,
        base_unit,
        base_lot_size
    );
    msg!(
        "quote decimals: {} - quote unit: {} - quote lot size: {}",
        quote_decimals,
        quote_unit,
        quote_lot_size
    );

    // Slippage calulation
    let perp_value = mango_cache.price_cache[perp_market_index].price;
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
    let slippage_amount = perp_value.checked_mul(slippage_ratio).unwrap();
    let price = perp_value.checked_sub(slippage_amount).unwrap();
    msg!("collateral_perp_value: {}", perp_value);
    msg!("price (after slippage calculation): {}", price);

    // Exposure delta calculation
    let deposited_value = collateral_amount
        .checked_div(base_unit)
        .unwrap()
        .checked_mul(perp_value)
        .unwrap();
    let exposure_delta = collateral_amount.checked_mul(perp_value).unwrap();
    msg!("collateral_deposited_value: {}", deposited_value); // Is this valus good with decimals? To check
    msg!("exposure_delta: {}", exposure_delta);

    let exposure_delta_qlu = exposure_delta.checked_div(quote_lot_size).unwrap();
    msg!(
        "exposure_delta_qlu (in quote lot unit): {}",
        exposure_delta_qlu
    );

    // price in quote lot unit
    let order_price_qlu = price
        .checked_mul(quote_unit)
        .unwrap()
        .checked_mul(base_lot_size)
        .unwrap()
        .checked_div(quote_lot_size)
        .unwrap()
        .checked_div(base_unit)
        .unwrap();
    msg!("price_qlu (in quote lot unit): {}", order_price_qlu);

    // Execution quantity
    let order_quantity_blu = exposure_delta_qlu
        .checked_div(order_price_qlu)
        .unwrap()
        .abs();
    msg!("exec_qty_blu (base lot unit): {}", order_quantity_blu);

    // We now calculate the amount pre perp opening, in order to define after if it got 100% filled or not
    let pre_position = {
        let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
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
    let order_price = order_price_qlu.to_num::<i64>();
    let order_quantity = order_quantity_blu.to_num::<i64>();
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
    if !(order_quantity == filled) {
        return Err(ControllerError::PerpPartiallyFilled.into());
    }
    // // XXX Here we taking the worse price, but it might have found a better exec price, and we should get that else
    // // the diff will fill the insurance fund each time, and people will always pay max slippage.
    // //
    // // Determine the filled price to define how much UXD we need to mint
    // let execution_price_qlu = order_price_qlu; //
    // // msg!("controller: mint uxd [Mint UXD for redeemables]");
    // let uxd_decimals = ctx.accounts.uxd_mint.decimals as u32;
    // let uxd_unit = I80F48::from_num(10u64.pow(uxd_decimals));
    // // USD value of delta position
    // let perp_order_uxd_value = order_quantity_blu.checked_mul(execution_price_qlu).unwrap();
    // msg!("perp_order_uxd_value : {}", perp_order_uxd_value);
    // Converted to UXD amount (we multiply by uxd decimals and divide by coin decimals and get a uxd amount)
    // let position_uxd_amount = perp_order_uxd_value
    //     .checked_mul(uxd_unit)
    //     .unwrap()
    //     .checked_div(base_unit)
    //     .unwrap();
    // msg!("position_uxd_amount : {}", position_uxd_amount);

    // For now give him for the worth exec price (full slippage), see above
    let uxd_amount = collateral_amount.checked_mul(price).unwrap();
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
