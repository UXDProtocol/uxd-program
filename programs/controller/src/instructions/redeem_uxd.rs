use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
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

#[derive(Accounts)]
#[instruction(uxd_amount: u64)]
pub struct RedeemUxd<'info> {
    // XXX again we should use approvals so user doesnt need to sign
    pub user: Signer<'info>,
    #[account(seeds = [STATE_SEED], bump)]
    pub state: Box<Account<'info, State>>,
    #[account(seeds = [DEPOSITORY_SEED, collateral_mint.key().as_ref()], bump)]
    pub depository: Box<Account<'info, Depository>>,
    #[account(constraint = collateral_mint.key() == depository.collateral_mint_key)]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [PASSTHROUGH_SEED, collateral_mint.key().as_ref()], bump)]
    pub collateral_passthrough: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_collateral.mint == depository.collateral_mint_key,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_uxd.mint == uxd_mint.key(),
        constraint = uxd_amount > 0,
        constraint = user_uxd.amount >= uxd_amount, // THESE SHOULD USE the custom error to avoid ` custom program error: 0x8f ` -- OR the access_control
    )]
    pub user_uxd: Box<Account<'info, TokenAccount>>,
    #[account(mut, seeds = [UXD_SEED], bump)]
    pub uxd_mint: Box<Account<'info, Mint>>,
    // XXX start mango --------------------------------------------------------
    // MangoGroup that this mango account is for
    pub mango_group: AccountInfo<'info>,
    // Mango Account of the Depository Record
    #[account(mut)]
    pub mango_account: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    // This is some mango internal stuff - name is misleading
    pub mango_signer: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,
    // The perp market for `collateral_mint` on mango, and the associated required accounts
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,
    // XXX end mango ----------------------------------------------------------
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    // sysvars
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RedeemUxd>, uxd_amount: u64, slippage: u32) -> ProgramResult {
    // get current passthrough balance before withdrawing from mango
    // in theory this should always be zero but better safe
    // XXX cannot be updated and read through this program, only if we would be doing ledger operations.
    // let _initial_passthrough_balance = I80F48::from_num(ctx.accounts.collateral_passthrough.amount);

    // msg!("controller: redeem uxd [calculation for perp position closing]");
    let collateral_mint_key = ctx.accounts.collateral_mint.key();
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
    // base and quote details - Some unused for now but will need to update when using the right decimals
    // let base_decimals = mango_group.tokens[perp_market_index].decimals;
    // let base_unit = I80F48::from_num(10u64.pow(base_decimals.into()));
    let base_lot_size = I80F48::from_num(mango_group.perp_markets[perp_market_index].base_lot_size);
    // let quote_decimals = mango_group.tokens[mango::state::QUOTE_INDEX].decimals;
    // let quote_unit = I80F48::from_num(10u64.pow(quote_decimals.into()));
    let quote_lot_size =
        I80F48::from_num(mango_group.perp_markets[perp_market_index].quote_lot_size);
    // msg!("-----");
    // msg!("base_unit {}", base_unit);
    msg!("base_lot_size {}", base_lot_size);
    // msg!("quote_unit {}", quote_unit);
    msg!("quote_lot_size {}", quote_lot_size);
    msg!("-----");

    // About Mango (Serum) lots size, native units, life and the universe
    //
    // First some context:
    //
    // A part is defined as BASE/QUOTE, base being the asset valued using quote
    // BASE and QUOTE are both SPL tokens, and have varying decimals.
    //
    // `lot_size` are an abritrary amount, the minimum amount of `unit`, previously described, tradable
    // both QUOTE and BASE has a specific lot size, for BTC it's 10 and USDC it's 100.
    // `base_unit` and `quote_unit` are simply `10^respective_decimals`.
    // Meaning you cannot trade smaller chunks that 10 units.
    //
    // So let's take BTC/USDC perp for instance :
    //
    // BTC has 8 decimals
    // so 1BTC == to  100_000_000 BTC   native units (satoshis)
    //
    // USDC has 6 decimals
    // so 1USDC == to   1_000_000 USDC  native units (tinycents idk)
    //
    // Mango base lot size for BTC is 10 (arbitrary, probably from Serum)
    // That means that mango smallest amount for trades in BTC is 10 satoshis (0.00_000_010)
    // For USDC it's 100, meaning 0.00_0100
    //
    // If you want to trade BTC with mango, you need to think in lot size,
    //  hence take your native units, and divide them by base_lot_size for that perp
    //
    // I want to place a perp order long for 0.05 BTC :
    //
    // First we calculate the quantity, that will be in [Base Lot]
    //  - base_unit ==                  10 ** base_decimals         -> 100_000_000 (although it's 6 on solana iirc, for for the sake of this example doesn't matter)
    //  - btc_amount ==                 0.05_000_000
    //  - btc_amount_native_unit ==     btc_amount * base_unit      ->   5_000_000
    //  - btc_amount_base_lot_unit ==   5_000_000 / base_lot_size   ->     500_000
    //
    // Then we calculate the price, that will be in [Quote Lot]
    //
    //  What we get from mango is the price of one `base unit` expressed in `quote units`
    // so for btc is how much quote unit for a satoshi
    //
    //  - perp_quote_price ==           mango_cache.price_cache[perp_market_index].price;
    //
    //  Mango deal in lots (Serum actually), so you need to run some conversions
    //
    // let base_lot_price_in_quote_unit = perp_price.checked_mul(base_lot_size)
    //
    // let base_lot_order_quantity = order_amount_in_quote_unit.checked_div(base_lot_price_in_quote_unit)
    //
    // let base_lot_price_in_quote_lot = base_lot_price_in_quote_unit.checked_div(quote_lot_size)
    //
    //  === Now can call `place_perp_order(quantity: base_lot_order_quantity, price: base_lot_price_in_quote_lot);`
    //
    // Let's say the order is filled 100%, then you bought
    //
    // quantity_bought_in_btc_base_unit ==  perp_order.taker_base * base_lot_size
    // usdc spent                       ==  perp_order.taker_quote * quote_lot_size
    //
    // And to that you can also calculate the fees
    //     let taker_fee = mango_group.perp_markets[perp_market_index].taker_fee;
    // then you do the calculation

    // Slippage calulation
    let price = mango_cache.price_cache[perp_market_index].price;
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
    let slippage_amount = price.checked_mul(slippage_ratio).unwrap();
    let price_adjusted = price.checked_add(slippage_amount).unwrap();
    msg!("price (base unit value expressed in quote unit): {}", price);
    msg!(
        "price_adjusted (after slippage calculation): {}",
        price_adjusted
    );

    // XXX considering UXD and USDC same decimals, fix later
    let exposure_delta_in_quote_unit = I80F48::from_num(uxd_amount);
    msg!(
        "+++ exposure_delta_in_quote_unit {}",
        exposure_delta_in_quote_unit
    );

    let base_lot_price_in_quote_unit = price_adjusted.checked_mul(base_lot_size).unwrap();
    msg!(
        "+++ base_lot_price_in_quote_unit {}",
        base_lot_price_in_quote_unit
    );

    let base_lot_order_quantity = exposure_delta_in_quote_unit
        .checked_div(base_lot_price_in_quote_unit)
        .unwrap();
    msg!("+++ base_lot_order_quantity {}", base_lot_order_quantity);

    let quote_lot_order_price = base_lot_price_in_quote_unit
        .checked_div(quote_lot_size)
        .unwrap();
    msg!("+++ quote_lot_order_price {}", quote_lot_order_price);

    // We now calculate the amount pre perp closing, in order to define after if it got 100% filled or not
    let pre_position = {
        let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];
        perp_account.base_position + perp_account.taker_base
    };
    // Drop ref cause they are also used in the Mango CPI destination
    drop(mango_group);
    drop(mango_cache);
    drop(mango_account);
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        DEPOSITORY_SEED,
        collateral_mint_key.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];
    // Call Mango CPI to place the order that closes short position
    let order_price = quote_lot_order_price.to_num::<i64>();
    let order_quantity = base_lot_order_quantity.to_num::<i64>();
    msg!("order_price {}", order_price);
    msg!("order_quantity {}", order_quantity);
    mango_program::place_perp_order(
        ctx.accounts
            .into_close_mango_short_perp_context()
            .with_signer(depository_signer_seed),
        order_price,
        order_quantity,
        0,
        mango::matching::Side::Bid,
        mango::matching::OrderType::ImmediateOrCancel,
        true,
    )?;

    // msg!("verify that the order got 100% filled");
    let mango_account = MangoAccount::load_checked(
        &ctx.accounts.mango_account,
        ctx.accounts.mango_program.key,
        ctx.accounts.mango_group.key,
    )?;
    let perp_account: &PerpAccount = &mango_account.perp_accounts[perp_market_index];

    let post_position = perp_account.base_position + perp_account.taker_base;
    msg!("base_position {}", perp_account.base_position);
    msg!("quote_position {}", perp_account.quote_position);
    msg!("taker_base {}", perp_account.taker_base);
    msg!("taker_quote {}", perp_account.taker_quote);
    msg!("-----");
    msg!("post_position {}", post_position);
    let filled = (post_position - pre_position).abs();
    msg!("filled {}", filled);
    if !(order_quantity == filled) {
        return Err(ControllerError::PerpPartiallyFilled.into());
    }

    // Real execution amount of base and quote
    let order_amount_quote_native_unit = I80F48::from_num(perp_account.taker_quote.abs())
        .checked_mul(quote_lot_size)
        .unwrap();
    let order_amount_base_native_unit = I80F48::from_num(perp_account.taker_base.abs())
        .checked_mul(base_lot_size)
        .unwrap();

    // XXX SHOULD MAKE THE decimal conversions from USDC/UXD to be safe
    msg!("UXD burn amount {}", order_amount_quote_native_unit);
    // - Burn the uxd they'r giving up
    token::burn(
        ctx.accounts.into_burn_uxd_context(),
        order_amount_quote_native_unit.to_num(),
    )?;

    // - Call mango CPI to withdraw collateral
    // XXX like price * 0.98 == price minus fees
    let fees = I80F48::ONE
        .checked_sub(taker_fee.checked_div(I80F48::ONE).unwrap())
        .unwrap();
    msg!("fees {}", fees);

    // Now calculate the quote amount to withdraw
    let amount_to_withdraw_base_native_unit = order_amount_base_native_unit
        // Minus fees
        .checked_mul(fees)
        .unwrap()
        .to_num();
    msg!(
        "amount_to_withdraw_base_native_unit {}",
        amount_to_withdraw_base_native_unit
    );

    // Drop ref cause they are also used in the Mango CPI destination
    drop(mango_account);
    mango_program::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mango_context()
            .with_signer(depository_signer_seed),
        amount_to_withdraw_base_native_unit,
        false,
    )?;

    // - Return collateral back to user
    token::transfer(
        ctx.accounts
            .into_transfer_collateral_to_user_context()
            .with_signer(depository_signer_seed),
        amount_to_withdraw_base_native_unit,
    )?;

    Ok(())
}

// MARK: - Contexts -----

impl<'info> RedeemUxd<'info> {
    pub fn into_burn_uxd_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.uxd_mint.to_account_info(),
            to: self.user_uxd.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_close_mango_short_perp_context(
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

    pub fn into_withdraw_collateral_from_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Withdraw<'info>> {
        let cpi_accounts = mango_program::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_account: self.collateral_passthrough.to_account_info(),
            mango_signer: self.mango_signer.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_collateral_to_user_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: self.collateral_passthrough.to_account_info(),
            to: self.user_collateral.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
