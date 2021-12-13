use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::matching::Book;
use mango::state::MangoAccount;
use mango::state::PerpAccount;
use mango::state::PerpMarket;

use crate::mango_program;
use crate::utils::get_best_order_for_quote_lot_amount;
use crate::utils::perp_base_position;
use crate::utils::Order;
use crate::utils::PerpInfo;
use crate::AccountingEvent;
use crate::Controller;
use crate::ErrorCode;
use crate::MangoDepository;
use crate::UxdResult;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;

#[derive(Accounts)]
pub struct RedeemFromMangoDepository<'info> {
    #[account(mut)]
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
        constraint = user_redeemable.mint == redeemable_mint.key() @ErrorCode::InvalidRedeemableMint
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
        constraint = depository.collateral_passthrough == depository_collateral_passthrough_account.key() @ErrorCode::InvalidCollateralPassthroughAccount,
        constraint = depository_collateral_passthrough_account.mint == collateral_mint.key() @ErrorCode::InvalidCollateralPassthroughATAMint
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @ErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,
    // Mango related accounts -------------------------------------------------
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
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Perp account state PRE perp order]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;

    // - [Make sure that the PerpAccount crank has been run previously to this instruction by the uxd-client so that pending changes are updated in mango]
    if !(perp_account.taker_base == 0 && perp_account.taker_quote == 0) {
        return Err(ErrorCode::InvalidPerpAccountState.into());
    }

    // - [Calculates the quantity of short to close]
    let mut exposure_delta_in_quote_unit = I80F48::from_num(redeemable_amount);

    // - [Find the max taker fees mango will take on the perp order and remove it from the exposure delta to be sure the amount order + fees doesn't overflow the redeemed amount]
    let max_fee_amount = exposure_delta_in_quote_unit
        .checked_mul(perp_info.taker_fee)
        .unwrap();
    exposure_delta_in_quote_unit = exposure_delta_in_quote_unit
        .checked_sub(max_fee_amount)
        .unwrap();

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
        )?;

    // - [Checks that the best price found is withing slippage range]
    check_short_perp_close_order_is_within_slippage_range(&perp_info, &best_order, slippage)?;

    // - [Place perp order CPI to Mango Market v3]
    mango_program::place_perp_order(
        ctx.accounts
            .into_close_mango_short_perp_context()
            .with_signer(depository_signer_seed),
        best_order.price,
        best_order.quantity,
        0,
        mango::matching::Side::Bid,
        mango::matching::OrderType::ImmediateOrCancel,
        true,
    )?;

    // - [Perp account state POST perp order]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled]
    let post_position = perp_base_position(&perp_account);
    check_short_perp_close_order_fully_filled(
        best_order.quantity,
        initial_base_position,
        post_position,
    )?;

    // - 2 [BURN THE EQUIVALENT AMOUT OF UXD] ---------------------------------
    // Note : In order to account for the perp order fees, we burn extra redeemable for the perp order fees (total corresponding to the amount redeemed + fees).
    //  This will make the delta neutral position is bit bigger thant the redeemable in circulation it actually hedges, but will be settled during rebalancing.
    let (order_delta, fee_delta) =
        derive_redeemable_order_and_fee_deltas(&perp_info, &perp_account);
    let redeemable_to_burn = order_delta.checked_add(fee_delta).unwrap().to_num();
    msg!("redeemable_to_burn (Burn) {}", redeemable_to_burn);
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_to_burn,
    )?;

    // - 3 [WITHDRAW COLLATERAL FROM MANGO THEN RETURN TO USER] ---------------
    // Note : The amount of collateral returned to the user
    let collateral_delta = derive_collateral_delta(&perp_info, &perp_account).to_num();
    // - [Mango withdraw CPI]
    mango_program::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mango_context()
            .with_signer(depository_signer_seed),
        collateral_delta,
        false,
    )?;

    // - [Return collateral back to user]
    msg!("collateral_delta (Transfered to user) {}", collateral_delta);
    token::transfer(
        ctx.accounts
            .into_transfer_collateral_to_user_context()
            .with_signer(depository_signer_seed),
        collateral_delta,
    )?;

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------
    let redeemable_delta = order_delta.to_num();
    let redeemable_fee_delta = fee_delta.to_num();
    ctx.accounts.update_onchain_accounting(
        collateral_delta,
        redeemable_delta,
        redeemable_fee_delta,
    )?;

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
    fn perpetual_info(&self) -> UxdResult<PerpInfo> {
        let perp_info = PerpInfo::new(
            &self.mango_group,
            &self.mango_cache,
            &self.mango_perp_market.key,
            self.mango_program.key,
        )?;
        msg!("Perpetual informations: {:?}", perp_info);
        Ok(perp_info)
    }

    // Return the uncommited PerpAccount that represent the account balances
    fn perp_account(&self, perp_info: &PerpInfo) -> UxdResult<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = match MangoAccount::load_checked(
            &self.depository_mango_account,
            self.mango_program.key,
            self.mango_group.key,
        ) {
            Ok(it) => it,
            Err(_err) => return Err(ErrorCode::MangoOrderBookLoading),
        };
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    fn get_best_price_and_quantity_for_quote_amount_from_order_book(
        &self,
        side: mango::matching::Side,
        quote_amount: i64,
    ) -> UxdResult<Order> {
        // Load book
        let perp_market = match PerpMarket::load_checked(
            &self.mango_perp_market,
            self.mango_program.key,
            self.mango_group.key,
        ) {
            Ok(it) => it,
            Err(_err) => return Err(ErrorCode::MangoOrderBookLoading),
        };
        let bids_ai = self.mango_bids.to_account_info();
        let asks_ai = self.mango_asks.to_account_info();
        let book =
            match Book::load_checked(self.mango_program.key, &bids_ai, &asks_ai, &perp_market) {
                Ok(it) => it,
                Err(_err) => return Err(ErrorCode::MangoOrderBookLoading),
            };
        let best_order = get_best_order_for_quote_lot_amount(&book, side, quote_amount);

        return match best_order {
            Some(best_order) => {
                msg!(
                    "best_order: [quantity {} - price {} - size {}]",
                    best_order.quantity,
                    best_order.price,
                    best_order.size
                );
                Ok(best_order)
            }
            None => Err(ErrorCode::InsuficentOrderBookDepth),
        };
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_delta: u64,
        redeemable_delta: u64,
        redeemable_fee_delta: u64,
    ) -> UxdResult {
        let fee_delta = redeemable_fee_delta;
        let circulating_supply_delta = redeemable_delta.checked_sub(fee_delta).unwrap();
        // Mango Depository
        let event = AccountingEvent::Withdraw;
        self.depository
            .update_collateral_amount_deposited(&event, collateral_delta);
        self.depository
            .update_delta_neutral_quote_fee_offset(&event, fee_delta);
        self.depository
            .update_delta_neutral_quote_position(&event, redeemable_delta);
        self.depository
            .update_redeemable_amount_under_management(&event, circulating_supply_delta);
        // Controller
        self.controller
            .update_redeemable_circulating_supply(&event, circulating_supply_delta);

        self.depository.sanity_check()?;

        Ok(())
    }

    fn check_mango_depositories_redeemable_soft_cap_overflow(
        &self,
        redeemable_delta: u64,
    ) -> UxdResult {
        if !(redeemable_delta <= self.controller.mango_depositories_redeemable_soft_cap) {
            return Err(ErrorCode::MangoDepositoriesSoftCapOverflow);
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
    msg!("price after slippage addition: {}", price_adjusted);
    return price_adjusted;
}

pub fn check_short_perp_close_order_is_within_slippage_range(
    perp_info: &PerpInfo,
    order: &Order,
    slippage: u32,
) -> UxdResult {
    let market_price = perp_info.price;
    let market_price_slippage_adjusted = slippage_addition(market_price, slippage);
    if order.price
        > market_price_slippage_adjusted
            .checked_mul(perp_info.base_lot_size)
            .unwrap()
            .checked_div(perp_info.quote_lot_size)
            .unwrap()
    {
        return Err(ErrorCode::InvalidSlippage);
    }
    Ok(())
}

// Verify that the order quantity matches the base position delta
pub fn check_short_perp_close_order_fully_filled(
    order_quantity: i64,
    pre_position: i64,
    post_position: i64,
) -> UxdResult {
    let filled_amount = (post_position.checked_sub(pre_position).unwrap()).abs();
    if !(order_quantity == filled_amount) {
        return Err(ErrorCode::PerpOrderPartiallyFilled);
    }
    Ok(())
}

fn derive_redeemable_order_and_fee_deltas(
    perp_info: &PerpInfo,
    perp_account: &PerpAccount,
) -> (I80F48, I80F48) {
    let order_amount_quote_native_unit = I80F48::from_num(perp_account.taker_quote.abs())
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    let fee_amount = order_amount_quote_native_unit
        .checked_mul(perp_info.taker_fee)
        .unwrap();
    (order_amount_quote_native_unit, fee_amount)
}

fn derive_collateral_delta(perp_info: &PerpInfo, perp_account: &PerpAccount) -> I80F48 {
    let order_amount_base_native_unit = I80F48::from_num(perp_account.taker_base.abs())
        .checked_mul(perp_info.base_lot_size)
        .unwrap();
    order_amount_base_native_unit
}
