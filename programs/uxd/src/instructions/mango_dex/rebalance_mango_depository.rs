use std::num::NonZeroU64;

use crate::check_assert;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::mango_program;
use crate::mango_utils::check_effective_order_price_versus_limit_price;
use crate::mango_utils::check_perp_order_fully_filled;
use crate::mango_utils::get_best_order_for_quote_lot_amount;
use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::Order;
use crate::mango_utils::PerpInfo;
use crate::serum_dex_program;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdError;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use fixed::types::I80F48;
use mango::matching::Book;
use mango::state::MangoAccount;
use mango::state::PerpAccount;
use mango::state::PerpMarket;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexRebalanceMangoDepository);

#[derive(Accounts)]
pub struct RebalanceMangoDepository<'info> {
    pub user: Signer<'info>,
    #[account(mut)] // The fee payer
    pub payer: Signer<'info>,
    // HERE NEED TO Pass a fee passthrough account to pay for the fees not to destabilize the system
    // #[account(
    //     mut,
    //     constraint = depository.fee_passthrough == depository_collateral_passthrough_account.key() @ErrorCode::InvalidCollateralPassthroughAccount,
    //     constraint = order_fee_payer.mint == @ErrorCode::InvalidCollateralPassthroughATAMint
    // )] // The fee payer for the orders fees to keep the system closed
    // pub order_fee_payer: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdIdlErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,
    // Mango related accounts -------------------------------------------------
    pub mango_group: UncheckedAccount<'info>,
    pub mango_cache: UncheckedAccount<'info>,
    pub mango_signer: UncheckedAccount<'info>,
    pub mango_root_bank: UncheckedAccount<'info>,
    #[account(mut)]
    pub mango_node_bank: UncheckedAccount<'info>,
    #[account(mut)]
    pub mango_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,
    #[account(mut)]
    pub mango_bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub mango_asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub mango_event_queue: UncheckedAccount<'info>,
    // Serum related accounts --------------------------------------------------
    #[account(mut)]
    pub spot_market: UncheckedAccount<'info>, 
    #[account(mut)]
    pub bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_request_queue: UncheckedAccount<'info>, 
    #[account(mut)]
    pub dex_event_queue: UncheckedAccount<'info>, 
    #[account(mut)]
    pub dex_base: UncheckedAccount<'info>, 
    #[account(mut)]
    pub dex_quote: UncheckedAccount<'info>, 
    pub base_root_bank: UncheckedAccount<'info>, 
    #[account(mut)]
    pub base_node_bank: UncheckedAccount<'info>, 
    #[account(mut)]
    pub base_vault: UncheckedAccount<'info>, 
    pub quote_root_bank: UncheckedAccount<'info>,
    #[account(mut)]
    pub quote_node_bank: UncheckedAccount<'info>, 
    #[account(mut)]
    pub quote_vault: UncheckedAccount<'info>, 
    pub signer: UncheckedAccount<'info>,      
    pub dex_signer: UncheckedAccount<'info>,  
    pub msrm_or_srm_vault: UncheckedAccount<'info>, 
    // ------------------------------------------------------------------------
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    pub dex_program: Program<'info, serum_dex_program::SerumDex>,
    // sysvars
    pub rent: Sysvar<'info, Rent>,
}

// Check what is the current PNL of the delta neutral position,
//   if it's positive it increase the overall delta neutral pos size
//   if it's negative it decreases it
pub fn handler(
    ctx: Context<RebalanceMangoDepository>,
    max_rebalancing_amount: u64, // In Quote
    slippage: u32,
) -> ProgramResult {
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - [Get perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // - 1 [SETTLE PNL (AND UNSETTLED FUNDING IF ANY)] ----------------------------------

    // Note : prior to this instruction, the UXD-client should call settlePnl from Mango Client,
    // IF the PNL is positive.
    // ELSE wait it out as it's a 0% interest loan (until we get settled by third parties possibly)

    // - [settle funding]
    // perp_account.settle_funding(cache)
    // todo!();
    // re load perp_account after else wont show update

    // - 2 [PREPARE REBALANCING] ----------------------------------------------

    // - [find out current perp PnL]
    let contract_size = perp_info.base_lot_size;
    let new_quote_position = I80F48::from_num(-pre_pa.base_position)
        .checked_mul(contract_size)
        .unwrap()
        .checked_mul(perp_info.price)
        .unwrap();

    let pnl = pre_pa
        .quote_position
        .checked_sub(new_quote_position)
        .unwrap();

    msg!(
        "quote_position {} - new_quote_position {} - pnl {}",
        pre_pa.quote_position,
        new_quote_position,
        pnl
    );

    if pnl.is_zero() {
        return Ok(());
    }

    // - [rebalancing limited to max_rebalancing_amount]
    let rebalancing_quote_amount = pnl.abs().min(I80F48::from_num(max_rebalancing_amount));

    // - 2 [REBALANCE] --------------------------------------------------------

    // - [Base depository's position size in native units PRE perp opening (to calculate the % filled later on)]
    let initial_base_position = total_perp_base_lot_position(&pre_pa)?;

    if pnl.is_negative() {
        // Find limit price and variables here.. THEN if this works, adjust quantity BELOW to match the result here (long == short)
        // let limit_price =

        // - 3 [Sell `long_spot_delta` amount + checks] -----------------------
        mango_program::place_spot_order_v2(
            ctx.accounts
                .into_sell_collateral_spot_context()
                .with_signer(depository_signer_seed),
            serum_dex::matching::Side::Ask,
            NonZeroU64::new(1).unwrap(),
            NonZeroU64::new(1).unwrap(),
            NonZeroU64::new(1).unwrap(),
            serum_dex::instruction::SelfTradeBehavior::AbortTransaction,
            serum_dex::matching::OrderType::ImmediateOrCancel,
            0,
            10, // Cycling through orders
        )?;
        // TODO

        // - 4 [Close `short_perp_delta` amount + checks] ---------------------

        // - [Find out how the best price and quantity for our order]
        let exposure_delta_in_quote_lot = rebalancing_quote_amount
            .checked_div(perp_info.quote_lot_size)
            .unwrap();
        let best_order = ctx
            .accounts
            .get_best_order_for_quote_lot_amount_from_order_book(
                mango::matching::Side::Ask,
                exposure_delta_in_quote_lot.to_num(),
            )?;

        // - [Checks that the best price found is within slippage range]
        check_effective_order_price_versus_limit_price(&perp_info, &best_order, slippage)?;

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
        let post_pa = ctx.accounts.perp_account(&perp_info)?;

        // - [Checks that the order was fully filled]
        let post_perp_order_base_lot_position = total_perp_base_lot_position(&post_pa)?;
        check_perp_order_fully_filled(
            best_order.quantity,
            initial_base_position,
            post_perp_order_base_lot_position,
        )?;
    }

    // - 3 [] ------
    // - [Update Accounting + verify global state of redeemable emitted / collateral size]
    //
    // TODO

    Ok(())

    // Place perp order SPOT https://github.com/blockworks-foundation/mango-v3/blob/main/program/src/processor.rs#L1462

    // Settle PnL (Only if positive, anyway the amount of Quote is already in the balance of the account) if needed https://github.com/blockworks-foundation/mango-v3/blob/7ff9e2c93e67cc467782048790f39c390e7aa280/program/src/processor.rs#L2284

    // Settle fees? wen? do we need? https://github.com/blockworks-foundation/mango-v3/blob/7ff9e2c93e67cc467782048790f39c390e7aa280/program/src/processor.rs#L2382
}

impl<'info> RebalanceMangoDepository<'info> {
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

    pub fn into_sell_collateral_spot_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::PlaceSpotOrderV2<'info>> {
        let cpi_accounts = mango_program::PlaceSpotOrderV2 {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            dex_prog: self.dex_program.to_account_info(),
            spot_market: self.spot_market.to_account_info(),
            bids: self.bids.to_account_info(),
            asks: self.asks.to_account_info(),
            dex_request_queue: self.dex_request_queue.to_account_info(),
            dex_event_queue: self.dex_event_queue.to_account_info(),
            dex_base: self.dex_base.to_account_info(),
            dex_quote: self.dex_quote.to_account_info(),
            base_root_bank: self.base_root_bank.to_account_info(),
            base_node_bank: self.base_node_bank.to_account_info(),
            base_vault: self.base_vault.to_account_info(),
            quote_root_bank: self.quote_root_bank.to_account_info(),
            quote_node_bank: self.quote_node_bank.to_account_info(),
            quote_vault: self.quote_vault.to_account_info(),
            token_prog: self.token_program.to_account_info(),
            signer: self.signer.to_account_info(),
            dex_signer: self.dex_signer.to_account_info(),
            msrm_or_srm_vault: self.msrm_or_srm_vault.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputed accounts
impl<'info> RebalanceMangoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> UxdResult<PerpInfo> {
        let perp_info = PerpInfo::new(
            &self.mango_group,
            &self.mango_cache,
            &self.mango_perp_market.key,
            self.mango_program.key,
        )?;
        // msg!("perp_info {:?}", perp_info);
        Ok(perp_info)
    }

    // Return the PerpAccount that represent the account balances (Quote and Taker, Taker is the part that is waiting settlement)
    fn perp_account(&self, perp_info: &PerpInfo) -> UxdResult<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.depository_mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    // fn resolve_unsettled_funding(&self, perp_account: &PerpAccount) -> UxdResult {
    //     let mango_group = match MangoGroup::load_checked(&self.mango_group, self.mango_program.key)
    //     {
    //         Ok(it) => it,
    //         Err(_err) => return Err(ErrorCode::MangoGroupLoading),
    //     };
    //     let mango_cache =
    //         match MangoCache::load_checked(&self.mango_cache, self.mango_program.key, &mango_group)
    //         {
    //             Ok(it) => it,
    //             Err(_err) => return Err(ErrorCode::MangoCacheLoading),
    //         };
    //     let perp_market_index = match mango_group.find_perp_market_index(self.mango_perp_market.key)
    //     {
    //         Some(it) => it,
    //         None => return Err(ErrorCode::MangoPerpMarketIndexNotFound),
    //     };
    //     let perp_market_cache = mango_cache.perp_market_cache[perp_market_index];
    //     perp_account.settle_funding(&perp_market_cache);

    //     Ok(())
    // }

    // fn get_best_order_for_base_lot_quantity_from_order_book(
    //     &self,
    //     side: mango::matching::Side,
    //     base_lot_amount: i64,
    // ) -> UxdResult<Order> {
    //     let perp_market = PerpMarket::load_checked(
    //         &self.mango_perp_market,
    //         self.mango_program.key,
    //         self.mango_group.key,
    //     )?;
    //     let bids_ai = self.mango_bids.to_account_info();
    //     let asks_ai = self.mango_asks.to_account_info();
    //     let book = Book::load_checked(self.mango_program.key, &bids_ai, &asks_ai, &perp_market)?;
    //     let best_order = get_best_order_for_base_lot_quantity(&book, side, base_lot_amount)?;

    //     Ok(best_order.ok_or(throw_err!(UxdErrorCode::InsufficientOrderBookDepth))?)
    // }

    fn get_best_order_for_quote_lot_amount_from_order_book(
        &self,
        side: mango::matching::Side,
        quote_lot_amount: i64,
    ) -> UxdResult<Order> {
        // Load book
        let perp_market = PerpMarket::load_checked(
            &self.mango_perp_market,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        let bids_ai = self.mango_bids.to_account_info();
        let asks_ai = self.mango_asks.to_account_info();
        let book = Book::load_checked(self.mango_program.key, &bids_ai, &asks_ai, &perp_market)?;
        let best_order = get_best_order_for_quote_lot_amount(&book, side, quote_lot_amount)?;

        Ok(best_order.ok_or(throw_err!(UxdErrorCode::InsufficientOrderBookDepth))?)
    }

    // fn get_best_price_and_quantity_for_quote_amount_from_order_book(
    //     &self,
    //     side: mango::matching::Side,
    //     quote_amount: i64,
    // ) -> UxdResult<Order> {
    //     // Load book
    //     let perp_market = match PerpMarket::load_checked(
    //         &self.mango_perp_market,
    //         self.mango_program.key,
    //         self.mango_group.key,
    //     ) {
    //         Ok(it) => it,
    //         Err(_err) => return Err(UxdErrorCode::MangoOrderBookLoading),
    //     };
    //     let bids_ai = self.mango_bids.to_account_info();
    //     let asks_ai = self.mango_asks.to_account_info();
    //     let book =
    //         match Book::load_checked(self.mango_program.key, &bids_ai, &asks_ai, &perp_market) {
    //             Ok(it) => it,
    //             Err(_err) => return Err(UxdErrorCode::MangoOrderBookLoading),
    //         };
    //     let best_order = get_best_order_for_quote_lot_amount(&book, side, quote_amount);

    //     return match best_order {
    //         Some(best_order) => {
    //             msg!(
    //                 "best_order: [quantity {} - price {} - size {}]",
    //                 best_order.quantity,
    //                 best_order.price,
    //                 best_order.size
    //             );
    //             Ok(best_order)
    //         }
    //         None => Err(UxdErrorCode::InsuficentOrderBookDepth),
    //     };
    // }
}

// Validate
impl<'info> RebalanceMangoDepository<'info> {
    pub fn validate(&self, max_coin_qty: u64, slippage: u32) -> ProgramResult {
        todo!();
        // check!(insurance_amount > 0, UxdErrorCode::InvalidInsuranceAmount)?;
        // // Mango withdraw will fail with proper error thanks to  `disabled borrow` set to true if the balance is not enough.
        // Ok(())
    }
}
