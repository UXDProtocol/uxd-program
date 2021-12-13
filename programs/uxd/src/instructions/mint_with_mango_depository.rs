use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use mango::matching::Book;
use mango::state::MangoAccount;
use mango::state::PerpAccount;
use mango::state::PerpMarket;

use crate::mango_program;
use crate::utils::get_best_order_for_base_lot_quantity;
use crate::utils::uncommitted_perp_base_position;
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
pub struct MintWithMangoDepository<'info> {
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
    // Mango CPI accounts
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
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintWithMangoDepository>,
    collateral_amount: u64, // native units
    slippage: u32,
) -> ProgramResult {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    let controller_signer_seed: &[&[&[u8]]] =
        &[&[CONTROLLER_NAMESPACE, &[ctx.accounts.controller.bump]]];

    // - 1 [TRANSFER COLLATERAL TO MANGO (LONG)] ------------------------------

    // - [Transfering user collateral to the passthrough account]
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_passthrough_context(),
        collateral_amount,
    )?;

    // - [Deposit to Mango CPI]
    mango_program::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_signer_seed),
        collateral_amount,
    )?;

    // - 2 [OPEN SHORT PERP POSITION] -----------------------------------------

    // - [Get perp informations]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Perp account state PRE perp order]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;

    // - [Make sure that the PerpAccount crank has been ran previously to this instruction by the uxd-client so that pending changes are updated in mango]
    if !(perp_account.taker_base == 0 && perp_account.taker_quote == 0) {
        return Err(ErrorCode::InvalidPerpAccountState.into());
    }

    // - [Get the amount of Base Lots for the perp order]
    let base_lot_amount = I80F48::from_num(collateral_amount)
        .checked_div(perp_info.base_lot_size)
        .unwrap();

    // - [Base depository's position size in native units PRE perp opening (to calculate the % filled later on)]
    let initial_base_position = perp_account.base_position;

    // - [Find the best order]
    let best_order = ctx
        .accounts
        .get_best_price_and_amount_for_base_lot_quantity_from_order_book(
            mango::matching::Side::Bid,
            base_lot_amount.to_num(),
        )?;

    // - [Checks that the best price found is withing slippage range]
    check_short_perp_open_order_is_within_slippage_range(&perp_info, &best_order, slippage)?;

    // - [Place perp order CPI to Mango Market v3]
    mango_program::place_perp_order(
        ctx.accounts
            .into_open_mango_short_perp_context()
            .with_signer(depository_signer_seed),
        best_order.price,
        best_order.quantity,
        0,
        mango::matching::Side::Ask,
        mango::matching::OrderType::ImmediateOrCancel,
        false,
    )?;

    // - [Perp account state POST perp order]
    let perp_account = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled]
    let post_position = uncommitted_perp_base_position(&perp_account);
    check_short_perp_open_order_fully_filled(
        best_order.quantity,
        initial_base_position,
        post_position,
    )?;

    // - 3 [MINTS THE HEDGED AMOUNT OF REDEEMABLE (minus fees)] ---------------
    // Note : by removing the fees from the emitted UXD, the delta neutral position will hedge more than the circulating UXD,
    //   this difference is for the system to offset for the orders placement fees and will be "settled" during rebalancing operations
    let (order_delta, fee_delta) =
        derive_redeemable_order_and_fee_deltas(&perp_info, &perp_account);
    let redeemable_delta = order_delta.to_num();
    let redeemable_to_mint = order_delta.checked_sub(fee_delta).unwrap().to_num();
    msg!("redeemable_delta {}", redeemable_delta);
    msg!("redeemable_to_mint {}", redeemable_to_mint);
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_signer_seed),
        redeemable_to_mint,
    )?;

    // Seems that the display of the mango account doesn't display the fees in the perp pos... investigating

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------
    let collateral_delta = derive_collateral_delta(&perp_info, &perp_account).to_num();
    let redeemable_fee_delta = fee_delta.to_num();
    msg!("collateral_delta {}", collateral_delta);
    msg!("redeemable_fee_delta {}", redeemable_fee_delta);
    ctx.accounts.update_onchain_accounting(
        collateral_delta,
        redeemable_delta,
        redeemable_fee_delta,
    )?;

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
            Err(_err) => return Err(ErrorCode::MangoAccountLoading),
        };
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    fn get_best_price_and_amount_for_base_lot_quantity_from_order_book(
        &self,
        side: mango::matching::Side,
        base_amount: i64,
    ) -> UxdResult<Order> {
        // Load book
        let perp_market = match PerpMarket::load_checked(
            &self.mango_perp_market,
            self.mango_program.key,
            self.mango_group.key,
        ) {
            Ok(it) => it,
            Err(_) => return Err(ErrorCode::MangoLoadPerpMarket),
        };
        let bids_ai = self.mango_bids.to_account_info();
        let asks_ai = self.mango_asks.to_account_info();
        let book =
            match Book::load_checked(self.mango_program.key, &bids_ai, &asks_ai, &perp_market) {
                Ok(it) => it,
                Err(_) => return Err(ErrorCode::MangoOrderBookLoading),
            };

        let best_order = get_best_order_for_base_lot_quantity(&book, side, base_amount);

        return match best_order {
            Some(best_order) => Ok(best_order),
            None => Err(ErrorCode::InsuficentOrderBookDepth),
        };
    }

    // Ensure that the minted amount does not raise the Redeemable supply beyond the Global Redeemable Supply Cap
    fn check_redeemable_global_supply_cap_overflow(&self) -> UxdResult {
        if !(self.controller.redeemable_circulating_supply
            <= self.controller.redeemable_global_supply_cap)
        {
            return Err(ErrorCode::RedeemableGlobalSupplyCapReached);
        }
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
        let event = AccountingEvent::Deposit;
        self.depository
            .update_collateral_amount_deposited(&event, collateral_delta);
        self.depository
            .update_redeemable_amount_under_management(&event, circulating_supply_delta);
        self.depository
            .update_delta_neutral_quote_fee_offset(&event, fee_delta);
        self.depository
            .update_delta_neutral_quote_position(&event, redeemable_delta);

        // Controller
        self.controller
            .update_redeemable_circulating_supply(&event, circulating_supply_delta);

        self.depository.sanity_check()?;

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

fn check_short_perp_open_order_is_within_slippage_range(
    perp_info: &PerpInfo,
    order: &Order,
    slippage: u32,
) -> UxdResult {
    let market_price = perp_info.price;
    let market_price_slippage_adjusted = slippage_deduction(market_price, slippage);
    if order.price
        < market_price_slippage_adjusted
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
fn check_short_perp_open_order_fully_filled(
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

pub fn derive_redeemable_order_and_fee_deltas(
    perp_info: &PerpInfo,
    perp_account: &PerpAccount,
) -> (I80F48, I80F48) {
    let order_amount_quote_native_unit = I80F48::from_num(perp_account.taker_quote.abs())
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    let fee_amount = order_amount_quote_native_unit
        .checked_mul(perp_info.taker_fee)
        .unwrap()
        .ceil();
    (order_amount_quote_native_unit, fee_amount)
}

pub fn derive_collateral_delta(perp_info: &PerpInfo, perp_account: &PerpAccount) -> I80F48 {
    let order_amount_base_native_unit = I80F48::from_num(perp_account.taker_base.abs())
        .checked_mul(perp_info.base_lot_size)
        .unwrap();
    order_amount_base_native_unit
}
