use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::error::MangoResult;
use mango::matching::Book;
use mango::matching::Order;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use mango::state::PerpAccount;
use mango::state::PerpMarket;

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
#[instruction(redeemable_amount: u64)]
pub struct RedeemFromMangoDepository<'info> {
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
        constraint = user_collateral.mint == depository.collateral_mint @ErrorCode::InvalidUserCollateralATAMint
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_redeemable.mint == redeemable_mint.key() @ErrorCode::InvalidRedeemableMint,
        constraint = user_redeemable.amount >= redeemable_amount @ErrorCode::InsuficientRedeemableAmount
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @ErrorCode::InvalidRedeemableMint
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
    // XXX All these account should be properly constrained if possible
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

    // - [Calculates the quantity of short to close]
    let exposure_delta_in_quote_unit = I80F48::from_num(redeemable_amount);

    // - [Perp account state PRE perp position opening]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;

    // - [Base depository's position size in native units PRE perp opening (to calculate the % filled later on)]
    let initial_base_position = perp_base_position(&perp_account);

    // - [Find out how the best price and quantity for our order]
    let exposure_delta_in_quote_lot_unit = exposure_delta_in_quote_unit
        .checked_div(perp_info.quote_lot_size)
        .unwrap();
    let best_order = ctx
        .accounts
        .get_best_price_and_quantity_for_quote_amount_from_order_book(
            mango::matching::Side::Ask,
            exposure_delta_in_quote_lot_unit.to_num(),
        )?
        .unwrap();
    msg!(
        "best_order: [quantity {} - price {}]",
        best_order.quantity,
        best_order.price
    );

    // - [Checks that the best price found is withing slippage range]
    let market_price = perp_info.price;
    let market_price_slippage_adjusted = slippage_addition(market_price, slippage);
    if best_order.price
        > market_price_slippage_adjusted
            .checked_mul(perp_info.base_lot_size)
            .unwrap()
            .checked_div(perp_info.quote_lot_size)
            .unwrap()
    {
        msg!("Error- The best order price is beyond slippage");
        return Err(ErrorCode::InvalidSlippage.into());
    }

    // - [Place perp order CPI to Mango Market v3]
    let base_lot_quantity = best_order.quantity;
    let base_lot_price_in_quote_lot_unit = best_order
        .price
        .checked_mul(perp_info.quote_lot_size.to_num())
        .unwrap();
    mango_program::place_perp_order(
        ctx.accounts
            .into_close_mango_short_perp_context()
            .with_signer(depository_signer_seed),
        base_lot_price_in_quote_lot_unit,
        base_lot_quantity,
        0,
        mango::matching::Side::Bid,
        mango::matching::OrderType::ImmediateOrCancel,
        true,
    )?;

    // - [Perp account state POST perp position opening]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled]
    let post_position = perp_base_position(&perp_account);
    check_short_perp_close_order_fully_filled(
        best_order.quantity,
        initial_base_position,
        post_position,
    )?;

    // - 2 [BURN THE EQUIVALENT AMOUT OF UXD] ---------------------------------

    // Real execution amount of base and quote
    // Note : Assuming same decimals for USDC/Redeemable(UXD) - To fix
    let order_amount_quote_native_unit = I80F48::from_num(perp_account.taker_quote.abs())
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    let redeemable_delta = order_amount_quote_native_unit.to_num();
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_delta,
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

    // msg!("collateral_amount withdrawn then returned amount {}", collateral_amount);

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts
        .check_and_update_accounting(collateral_amount, redeemable_delta)?;

    // - 6 [ENSURE MINTING DOESN'T OVERFLOW THE MANGO DEPOSITORIES REDEEMABLE SOFT CAP]
    ctx.accounts
        .check_mango_depositories_redeemable_soft_cap_overflow(redeemable_delta)?;

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

    // Could use this to do a quick check - might end up using computing for no reason, let's keep that here.
    //
    // Walk up the book quantity units and return the price at that level. If quantity units not on book, return None
    // fn get_impact_price_from_perp_order_book(
    //     &self,
    //     side: mango::matching::Side,
    //     quantity: i64,
    // ) -> MangoResult<Option<i64>> {
    //     let perp_market = PerpMarket::load_checked(
    //         &self.mango_perp_market,
    //         self.mango_program.key,
    //         self.mango_group.key,
    //     )?;
    //     let bids_ai = self.mango_bids.to_account_info();
    //     let asks_ai = self.mango_asks.to_account_info();
    //     let book = Book::load_checked(self.mango_program.key, &bids_ai, &asks_ai, &perp_market)?;
    //     Ok(book.get_impact_price(side, quantity))
    // }

    fn get_best_price_and_quantity_for_quote_amount_from_order_book(
        &self,
        side: mango::matching::Side,
        quote_amount: i64,
    ) -> MangoResult<Option<Order>> {
        // Load book
        let perp_market = PerpMarket::load_checked(
            &self.mango_perp_market,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        let bids_ai = self.mango_bids.to_account_info();
        let asks_ai = self.mango_asks.to_account_info();
        let book = Book::load_checked(self.mango_program.key, &bids_ai, &asks_ai, &perp_market)?;
        Ok(book.get_best_order_for_quote_lot_amount(side, quote_amount))
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn check_and_update_accounting(
        &mut self,
        collateral_delta: u64,
        redeemable_delta: u64,
    ) -> ProgramResult {
        // Mango Depository
        self.depository
            .update_collateral_amount_deposited(AccountingEvent::Redeem, collateral_delta);
        self.depository
            .update_redeemable_amount_under_management(AccountingEvent::Redeem, redeemable_delta);
        // Controller
        self.controller
            .update_redeemable_circulating_supply(AccountingEvent::Redeem, redeemable_delta);

        // TODO catch errors above and make explicit error

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
        return Err(ErrorCode::PerpOrderPartiallyFilled.into());
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
