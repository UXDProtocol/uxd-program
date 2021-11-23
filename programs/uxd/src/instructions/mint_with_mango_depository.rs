use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::traits::ToFixed;
use fixed::types::I80F48;
use fixed::types::U64F0;
use mango::error::MangoResult;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use mango::state::PerpAccount;

use crate::mango_program;
use crate::utils::perp_base_position;
use crate::utils::PerpInfo;
use crate::AccountingEvent;
use crate::Controller;
use crate::ErrorCode;
use crate::MangoDepository;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;

#[derive(Accounts)]
pub struct MintWithMangoDepository<'info> {
    // XXX again we should use approvals so user doesnt need to sign
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump,
        has_one = controller @ErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @ErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @ErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @ErrorCode::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_collateral.mint == depository.collateral_mint @ErrorCode::InvalidUserCollateralATAMint
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_redeemable.mint == controller.redeemable_mint @ErrorCode::InvalidUserRedeemableATAMint
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
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
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

// HANDLER
pub fn handler(
    ctx: Context<MintWithMangoDepository>,
    collateral_amount: u64,
    slippage: u32,
) -> ProgramResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [TRANSFER COLLATERAL TO MANGO (LONG)] ------------------------------

    // msg!("Transfering user collateral to the passthrough account");
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_passthrough_context(),
        collateral_amount,
    )?;

    // msg!("uxd: mint uxd [Deposit Mango CPI]");
    mango_program::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_signer_seeds),
        collateral_amount,
    )?;

    // - 2 [OPEN SAME SIZE SHORT POSITION ] -----------------------------------

    // - [Get perp informations]
    let perp_info = ctx.accounts.perpetual_info();
    msg!("Perpetual informations: {:?}", perp_info);

    // - [Slippage calculation]
    // This is the price of one base lot in quote lot units : `perp_info.base_lot_price_in_quote_lot_unit()`
    let base_lot_price_in_quote_lot_unit =
        slippage_deduction(perp_info.base_lot_price_in_quote_lot_unit(), slippage);
    // msg!("base_lot_price_in_quote_lot_unit (after slippage deduction): {}", base_lot_price_in_quote_lot_unit);

    // - [Calculates the quantity of base lot to open short]
    // XXX assuming USDC and Redeemable (UXD) have same decimals, need to fix
    let collateral_amount_native_unit = I80F48::from_num(collateral_amount);
    let quantity_base_lot = collateral_amount_native_unit
        .checked_div(perp_info.base_lot_size)
        .unwrap();
    // msg!("quantity_base_lot: {}", quantity_base_lot);

    // - [Position PRE perp opening to calculate the % filled later on]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;
    let pre_position = perp_base_position(&perp_account);

    // - [Call mango CPI to open the perp short position]
    let order_price = base_lot_price_in_quote_lot_unit.to_num::<i64>();
    let order_quantity = quantity_base_lot.to_num::<i64>();
    // msg!("order_price {} - order_quantity {}", order_price, order_quantity);
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

    // - [Position POST perp opening to calculate the % filled later on]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;
    let post_position = perp_base_position(&perp_account);

    // - [Verify that the order has been 100% filled]
    let filled_amount = (post_position.checked_sub(pre_position).unwrap()).abs();
    check_short_perp_open_order_fully_filled(order_quantity, filled_amount)?;

    // - 3 [MINTS THE HEDGED AMOUNT OF REDEEMABLE] ----------------------------
    let redeemable_amount_fixed = derive_redeemable_amount(&perp_info, &perp_account);
    let redeemable_delta = redeemable_amount_fixed.to_num::<u64>();
    msg!("redeemable_amount to mint {}", redeemable_delta);

    let controller_signer_seed: &[&[&[u8]]] =
        &[&[CONTROLLER_NAMESPACE, &[ctx.accounts.controller.bump]]];
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_signer_seed),
        redeemable_delta,
    )?;

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------
    let collateral_delta = derive_collateral_delta(filled_amount, &perp_info);
    ctx.accounts
        .check_and_update_accounting(collateral_delta, redeemable_delta)?;

    // Note - 5 and 6 could be done before, but to save computing I'm doing it here for now as I've everything avalable

    // - 5 [ENSURE MINTING DOESN'T OVERFLOW THE GLOBAL REDEEMABLE SUPPLY CAP] -
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    // - 6 [ENSURE MINTING DOESN'T OVERFLOW THE MANGO DEPOSITORIES REDEEMABLE SOFT CAP]
    ctx.accounts
        .check_mango_depositories_redeemable_soft_cap_overflow(redeemable_delta)?;

    Ok(())
}

impl<'info> MintWithMangoDepository<'info> {
    pub fn into_transfer_user_collateral_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank.to_account_info(),
            mango_node_bank: self.mango_node_bank.to_account_info(),
            mango_vault: self.mango_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            owner_token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_open_mango_short_perp_context(
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

    pub fn into_mint_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputed accounts
impl<'info> MintWithMangoDepository<'info> {
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
        PerpInfo::init(&mango_group, &mango_cache, perp_market_index)
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

    // Ensure that the minted amount does not raise the Redeemable supply beyond the Global Redeemable Supply Cap
    fn check_redeemable_global_supply_cap_overflow(&self) -> ProgramResult {
        if !(self.controller.redeemable_circulating_supply
            <= self.controller.redeemable_global_supply_cap)
        {
            return Err(ErrorCode::RedeemableGlobalSupplyCapReached.into());
        }
        Ok(())
    }

    fn check_mango_depositories_redeemable_soft_cap_overflow(
        &self,
        redeemable_delta: u64,
    ) -> ProgramResult {
        if !(redeemable_delta <= self.controller.mango_depositories_redeemable_soft_cap) {
            return Err(ErrorCode::MangoDepositoriesSoftCapOverflow.into());
        }
        Ok(())
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn check_and_update_accounting(
        &mut self,
        collateral_delta: u64,
        redeemable_delta: u64,
    ) -> ProgramResult {
        // Mango Depository
        self.depository
            .update_collateral_amount_deposited(AccountingEvent::Mint, collateral_delta);
        self.depository
            .update_redeemable_amount_under_management(AccountingEvent::Mint, redeemable_delta);
        // Controller
        self.controller
            .update_redeemable_circulating_supply(AccountingEvent::Mint, redeemable_delta);

        // TODO catch errors above and make explicit error

        Ok(())
    }
}

// Returns price after slippage deduction
fn slippage_deduction(price: I80F48, slippage: u32) -> I80F48 {
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
    let slippage_amount = price.checked_mul(slippage_ratio).unwrap();
    price.checked_sub(slippage_amount).unwrap()
}

// Verify that the order quantity matches the base position delta
fn check_short_perp_open_order_fully_filled(
    order_quantity: i64,
    filled_amount: i64,
) -> ProgramResult {
    if !(order_quantity == filled_amount) {
        return Err(ErrorCode::PerpOrderPartiallyFilled.into());
    }
    Ok(())
}

// Find out the amount of redeemable the program mints for the user, derived from the outcome of the perp short opening
fn derive_redeemable_amount(perp_info: &PerpInfo, perp_account: &PerpAccount) -> I80F48 {
    // Need to add a check to make sure we don't mint more UXD than collateral value `collateral_amount_native_unit` (stupid?)
    // -
    // What is the valuation of the collateral? When we enter the instruction, do we value it from Base/ Perp price?
    // -
    // We Open a short position that tries to match that deposited collateral, but it might be smaller due to slippage.
    // We then mint on the value on this short position (To make sure everything that's minted is hedged)

    // - [Calculate the actual execution price (minus the mango fees)]
    let order_price_native_unit = I80F48::from_num(perp_account.taker_quote)
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    msg!(
        "  derive_redeemable_amount() - order_price_native_unit {}",
        order_price_native_unit
    );

    let fees = order_price_native_unit.abs() * perp_info.taker_fee;
    msg!("  derive_redeemable_amount() - fees {}", fees);

    // XXX here it's considering UXD and USDC have same decimals -- FIX LATER
    // THIS SHOULD BE THE SPOT MARKET VALUE MINTED AND NOT THE PERP VALUE CAUSE ELSE IT'S TOO MUCH
    order_price_native_unit.checked_sub(fees).unwrap()
}

fn derive_collateral_delta(base_lot_amount: i64, perp_info: &PerpInfo) -> u64 {
    base_lot_amount
        .checked_mul(perp_info.base_lot_size.to_num()) // Back from lot to native units
        .unwrap()
        .checked_abs()
        .unwrap()
        .checked_to_fixed::<U64F0>()
        .unwrap()
        .to_num()
    // msg!("collateral_delta {}", collateral_delta);
}
