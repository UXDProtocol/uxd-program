use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::error::MangoResult;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use mango::state::PerpAccount;

use crate::mango_program;
use crate::perp_base_position;
use crate::Depository;
use crate::PerpInfo;
use crate::State;
use crate::UXDError;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::SLIPPAGE_BASIS;
use crate::UXD_MINT_NAMESPACE;

#[derive(Accounts)]
#[instruction(uxd_amount: u64)]
pub struct RedeemUxd<'info> {
    // XXX again we should use approvals so user doesnt need to sign - wut, asking hana
    pub user: Signer<'info>,
    #[account(
        seeds = [&State::discriminator()[..]],
        bump = state.bump
    )]
    pub state: Box<Account<'info, State>>,
    #[account(
        seeds = [&Depository::discriminator()[..], collateral_mint.key().as_ref()],
        bump = depository.bump
    )]
    pub depository: Box<Account<'info, Depository>>,
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UXDError::MintMismatchCollateral
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.collateral_passthrough_bump,
    )]
    pub collateral_passthrough: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_collateral.mint == depository.collateral_mint @UXDError::MintMismatchCollateral
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_uxd.mint == uxd_mint.key() @UXDError::InvalidUxdMint,
        constraint = uxd_amount > 0 @UXDError::InvalidUxdRedeemAmount,
        constraint = user_uxd.amount >= uxd_amount @UXDError::InsuficientUxdAmount
    )]
    pub user_uxd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [UXD_MINT_NAMESPACE],
        bump = state.uxd_mint_bump,
    )]
    pub uxd_mint: Box<Account<'info, Mint>>,
    // XXX start mango --------------------------------------------------------
    // XXX All these account should be properly constrained
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
pub fn handler(ctx: Context<RedeemUxd>, uxd_amount: u64, slippage: u32) -> ProgramResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        &Depository::discriminator()[..],
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [CLOSE THE EQUIVALENT PERP SHORT ON MANGO] -------------------------

    // - [Get perp informations]
    let perp_info = ctx.accounts.perpetual_info();

    // - [Slippage calculation]
    let price_adjusted = slippage_addition(perp_info.price, slippage);

    let base_lot_price_in_quote_unit = price_adjusted.checked_mul(perp_info.base_lot_size).unwrap();
    // msg!("base_lot_price_in_quote_unit {}", base_lot_price_in_quote_unit);

    // - [Calculates the quantity of short to close]
    // XXX assuming USDC and UXD have same decimals, need to fix
    let exposure_delta_in_quote_unit = I80F48::from_num(uxd_amount);
    let quantity_base_lot_unit = exposure_delta_in_quote_unit
        .checked_div(base_lot_price_in_quote_unit)
        .unwrap();
    msg!("quantity_base_lot: {}", quantity_base_lot_unit);

    // - [Position PRE perp opening to calculate the % filled later on]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;
    let pre_position = perp_base_position(&perp_account);

    // - [Call Mango CPI to place the order that closes short position]
    let order_price = base_lot_price_in_quote_unit.to_num::<i64>();
    let order_quantity = quantity_base_lot_unit.to_num::<i64>();
    // msg!("order_price {} - order_quantity {}", order_price, order_quantity);
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

    // - [Position POST perp opening to calculate the % filled later on]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;
    let post_position = perp_base_position(&perp_account);

    // - [Verify that the order has been 100% filled]
    check_short_perp_close_order_fully_filled(order_quantity, pre_position, post_position)?;

    // - 2 [BURN THE EQUIVALENT AMOUT OF UXD] ---------------------------------

    // Real execution amount of base and quote
    // XXX Assuming same decimals for USDC/UXD - To fix
    let order_amount_quote_native_unit = I80F48::from_num(perp_account.taker_quote.abs())
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    token::burn(
        ctx.accounts.into_burn_uxd_context(),
        order_amount_quote_native_unit.to_num(),
    )?;
    msg!("UXD burnt amount {}", order_amount_quote_native_unit);

    // - 3 [WITHDRAW COLLATERAL FROM MANGO THEN RETURN TO USER] ---------------

    let collateral_amount = derive_collateral_amount(&perp_info, &perp_account).to_num();
    // - mango withdraw
    mango_program::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mango_context()
            .with_signer(depository_signer_seed),
        collateral_amount,
        false,
    )?;

    // - Return collateral back to user
    token::transfer(
        ctx.accounts
            .into_transfer_collateral_to_user_context()
            .with_signer(depository_signer_seed),
        collateral_amount,
    )?;

    msg!(
        "collateral withdrawn then returned amount {}",
        collateral_amount
    );

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

// Additional convenience methods related to the inputed accounts
impl<'info> RedeemUxd<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> PerpInfo {
        let mango_group =
            MangoGroup::load_checked(&self.mango_group, self.mango_program.key).unwrap();
        let mango_cache =
            MangoCache::load_checked(&self.mango_cache, self.mango_program.key, &mango_group)
                .unwrap();
        let perp_market_index = mango_group
            .find_perp_market_index(self.mango_perp_market.key)
            .unwrap();
        let perp_info = PerpInfo::init(&mango_group, &mango_cache, perp_market_index);
        msg!("Perpetual informations: {:?}", perp_info);
        return perp_info;
    }

    // Return the uncommited PerpAccount that represent the account balances
    fn perp_account(&self, perp_info: &PerpInfo) -> MangoResult<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }
}

// Returns price after slippage deduction
fn slippage_addition(price: I80F48, slippage: u32) -> I80F48 {
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
    let slippage_amount = price.checked_mul(slippage_ratio).unwrap();
    let price_adjusted = price.checked_add(slippage_amount).unwrap();
    msg!("price after slippage deduction: {}", price_adjusted);
    return price_adjusted;
}

// Verify that the order quantity matches the base position delta
fn check_short_perp_close_order_fully_filled(
    order_quantity: i64,
    pre_position: i64,
    post_position: i64,
) -> ProgramResult {
    let filled_amount = (post_position.checked_sub(pre_position).unwrap()).abs();
    if !(order_quantity == filled_amount) {
        return Err(UXDError::PerpOrderPartiallyFilled.into());
    }
    Ok(())
}

// Find out how much UXD the program mints for the user, derived from the outcome of the perp short opening
fn derive_collateral_amount(perp_info: &PerpInfo, perp_account: &PerpAccount) -> I80F48 {
    let order_amount_base_native_unit = I80F48::from_num(perp_account.taker_base.abs())
        .checked_mul(perp_info.base_lot_size)
        .unwrap();
    msg!(
        "order_amount_base_native_unit {}",
        order_amount_base_native_unit
    );
    let fees = I80F48::ONE
        .checked_sub(perp_info.taker_fee.checked_div(I80F48::ONE).unwrap())
        .unwrap();
    msg!("fees {}", fees);

    order_amount_base_native_unit
        .checked_mul(fees)
        .unwrap()
        .to_num()
}
