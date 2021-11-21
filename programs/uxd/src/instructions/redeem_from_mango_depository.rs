use anchor_lang::prelude::*;
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
use crate::utils::perp_base_position;
use crate::utils::PerpInfo;
use crate::Controller;
use crate::MangoDepository;
use crate::UXDError;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;

#[derive(Accounts)]
#[instruction(redeemable_amount: u64)]
pub struct RedeemFromMangoDepository<'info> {
    // XXX again we should use approvals so user doesnt need to sign - wut, asking hana
    pub user: Signer<'info>,
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump
    )]
    pub controller: Account<'info, Controller>,
    #[account(
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump
    )]
    pub depository: Account<'info, MangoDepository>,
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UXDError::MintMismatchCollateral
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_collateral.mint == depository.collateral_mint @UXDError::MintMismatchCollateral
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_redeemable.mint == redeemable_mint.key() @UXDError::InvalidRedeemableMint,
        constraint = redeemable_amount > 0 @UXDError::InvalidRedeemAmount,
        constraint = user_redeemable.amount >= redeemable_amount @UXDError::InsuficientRedeemableAmount
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.collateral_passthrough_bump,
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.mango_account_bump,
    )]
    pub depository_mango_account: AccountInfo<'info>,
    // Mango related accounts -------------------------------------------------
    // XXX All these account should be properly constrained
    pub mango_group: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_signer: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,
    // ------------------------------------------------------------------------
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    // sysvars
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RedeemFromMangoDepository>,
    redeemable_amount: u64,
    slippage: u32,
) -> ProgramResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
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
    // XXX assuming USDC and redeemable (UXD) have same decimals, need to fix
    let exposure_delta_in_quote_unit = I80F48::from_num(redeemable_amount);
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
    // XXX Assuming same decimals for USDC/Redeemable(UXD) - To fix
    let order_amount_quote_native_unit = I80F48::from_num(perp_account.taker_quote.abs())
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        order_amount_quote_native_unit.to_num(),
    )?;
    msg!("Redeemable burnt amount {}", order_amount_quote_native_unit);

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

impl<'info> RedeemFromMangoDepository<'info> {
    pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_close_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::PlacePerpOrder<'info>> {
        let cpi_accounts = mango_program::PlacePerpOrder {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
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
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
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
            from: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            to: self.user_collateral.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputed accounts
impl<'info> RedeemFromMangoDepository<'info> {
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
            &self.depository_mango_account,
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
