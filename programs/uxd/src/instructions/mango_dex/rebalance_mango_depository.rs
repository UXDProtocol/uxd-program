use crate::check_assert;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::mango_program;
use crate::mango_utils::PerpInfo;
use crate::mango_utils::check_perp_order_fully_filled;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::limit_price;
use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::SpotInfo;
use crate::serum_dex_program;
use crate::state::AccountingEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdError;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::SLIPPAGE_BASIS;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use fixed::types::I80F48;
use mango::state::MangoAccount;
use mango::state::MangoGroup;
use mango::state::PerpAccount;
use std::num::NonZeroU64;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexRebalanceMangoDepository);

#[derive(Accounts)]
pub struct RebalanceMangoDepository<'info> {
    pub user: Signer<'info>,
    // #[account(mut)] // The fee payer
    // pub payer: Signer<'info>,
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
    pub serum_dex_signer: UncheckedAccount<'info>,
    pub msrm_or_srm_vault: UncheckedAccount<'info>,
    pub base_root_bank: UncheckedAccount<'info>,
    pub quote_root_bank: UncheckedAccount<'info>,
    #[account(mut)]
    pub base_node_bank: UncheckedAccount<'info>,
    #[account(mut)]
    pub quote_node_bank: UncheckedAccount<'info>,
    #[account(mut)]
    pub mango_base_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub mango_quote_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub serum_base_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub serum_quote_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub perp_market: UncheckedAccount<'info>,
    #[account(mut)]
    pub spot_market: UncheckedAccount<'info>,
    #[account(mut)]
    pub perp_bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub perp_asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub spot_bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub spot_asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub perp_event_queue: UncheckedAccount<'info>,
    #[account(mut)]
    pub spot_event_queue: UncheckedAccount<'info>,
    #[account(mut)]
    pub spot_request_queue: UncheckedAccount<'info>,
    // ------------------------------------------------------------------------
    // programs
    pub token_program: Program<'info, Token>,
    pub mango_program: Program<'info, mango_program::Mango>,
    pub serum_dex_program: Program<'info, serum_dex_program::SerumDex>,
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
    // - [Get spot information]
    let spot_info = ctx.accounts.spot_info()?;

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
    let perp_contract_size = perp_info.base_lot_size;
    let new_quote_position = I80F48::from_num(-pre_pa.base_position)
        .checked_mul(perp_contract_size)
        .ok_or(math_err!())?
        .checked_mul(perp_info.price)
        .ok_or(math_err!())?;

    let pnl = pre_pa
        .quote_position
        .checked_sub(new_quote_position)
        .ok_or(math_err!())?;

    msg!(
        "PERP quote_position {} - new_quote_position {} - pnl {}",
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
        let spot_price = spot_info.price;
        let perp_price = perp_info.price;
        let spot_side = mango::matching::Side::Ask;
        let perp_side = mango::matching::Side::Bid;

        let limit_price_spot = limit_price(spot_price, slippage, spot_side)?
            .checked_div(spot_info.quote_lot_size)
            .ok_or(math_err!())?;
        let limit_price_perp = limit_price(perp_price, slippage, perp_side)?
            .checked_div(perp_info.quote_lot_size)
            .ok_or(math_err!())?;

        // - 3 [Sell `long_spot_delta` amount + checks] -----------------------
        let max_coin_quantity = rebalancing_quote_amount
            .checked_div(spot_price)
            .ok_or(math_err!())?
            .checked_div(spot_info.base_lot_size) // In serum spot base_lots
            .ok_or(math_err!())?;
        let max_native_pc_qty_including_fees = rebalancing_quote_amount
            .checked_div(perp_info.quote_lot_size)
            .ok_or(math_err!())?;

        let collateral_balance_pre = ctx.accounts.get_collateral_native_deposit()?;

        mango_program::place_spot_order_v2(
            ctx.accounts
                .into_sell_collateral_spot_context()
                .with_signer(depository_signer_seed),
            serum_dex::matching::Side::Ask,
            NonZeroU64::new(limit_price_spot.checked_to_num().ok_or(math_err!())?)
                .ok_or(math_err!())?,
            NonZeroU64::new(max_coin_quantity.checked_to_num().ok_or(math_err!())?)
                .ok_or(math_err!())?,
            NonZeroU64::new(
                max_native_pc_qty_including_fees
                    .checked_to_num()
                    .ok_or(math_err!())?,
            )
            .ok_or(math_err!())?,
            serum_dex::instruction::SelfTradeBehavior::AbortTransaction,
            serum_dex::matching::OrderType::ImmediateOrCancel,
            0,
            10, // Cycling through orders
        )?;

        let collateral_balance_post = ctx.accounts.get_collateral_native_deposit()?;
        // - [Find out how much collateral we purchased spot]
        let spot_collateral_delta = collateral_balance_pre
            .checked_sub(collateral_balance_post)
            .ok_or(math_err!())?;
        msg!(
            "collateral_balance_pre {} - collateral_balance_post {} - spot_collateral_delta {}",
            collateral_balance_pre,
            collateral_balance_post,
            spot_collateral_delta
        );

        // - 4 [Close `spot_collateral_delta` amount + checks] ---------------------
        let perp_order_quantity = spot_collateral_delta
            .checked_div(perp_info.base_lot_size)
            .ok_or(math_err!())?
            .to_num();
        mango_program::place_perp_order_v2(
            ctx.accounts
                .into_close_mango_short_perp_context()
                .with_signer(depository_signer_seed),
            limit_price_perp.to_num(),
            perp_order_quantity,
            0,
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
            perp_order_quantity,
            initial_base_position,
            post_perp_order_base_lot_position,
        )?;

        // - [Update Accounting + verify global state of redeemable emitted / collateral size]
        let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
        ctx.accounts.update_onchain_accounting(
            order_delta.collateral,
            order_delta.fee,
            &perp_side,
        )?;
    }

    Ok(())

    // Place perp order SPOT https://github.com/blockworks-foundation/mango-v3/blob/main/program/src/processor.rs#L1462

    // Settle PnL (Only if positive, anyway the amount of Quote is already in the balance of the account) if needed https://github.com/blockworks-foundation/mango-v3/blob/7ff9e2c93e67cc467782048790f39c390e7aa280/program/src/processor.rs#L2284

    // Settle fees? wen? do we need? https://github.com/blockworks-foundation/mango-v3/blob/7ff9e2c93e67cc467782048790f39c390e7aa280/program/src/processor.rs#L2382
}

impl<'info> RebalanceMangoDepository<'info> {
    pub fn into_close_mango_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::PlacePerpOrderV2<'info>> {
        let cpi_accounts = mango_program::PlacePerpOrderV2 {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_perp_market: self.perp_market.to_account_info(),
            mango_bids: self.perp_bids.to_account_info(),
            mango_asks: self.perp_asks.to_account_info(),
            mango_event_queue: self.perp_event_queue.to_account_info(),
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
            dex_prog: self.serum_dex_program.to_account_info(),
            spot_market: self.spot_market.to_account_info(),
            bids: self.spot_bids.to_account_info(),
            asks: self.spot_asks.to_account_info(),
            dex_request_queue: self.spot_request_queue.to_account_info(),
            dex_event_queue: self.spot_event_queue.to_account_info(),
            dex_base: self.serum_base_vault.to_account_info(),
            dex_quote: self.serum_quote_vault.to_account_info(),
            base_root_bank: self.base_root_bank.to_account_info(),
            base_node_bank: self.base_node_bank.to_account_info(),
            base_vault: self.mango_base_vault.to_account_info(),
            quote_root_bank: self.quote_root_bank.to_account_info(),
            quote_node_bank: self.quote_node_bank.to_account_info(),
            quote_vault: self.mango_quote_vault.to_account_info(),
            token_prog: self.token_program.to_account_info(),
            signer: self.mango_signer.to_account_info(),
            dex_signer: self.serum_dex_signer.to_account_info(),
            msrm_or_srm_vault: self.msrm_or_srm_vault.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> RebalanceMangoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> UxdResult<PerpInfo> {
        let perp_info = PerpInfo::new(
            &self.mango_group,
            &self.mango_cache,
            &self.perp_market.key,
            self.mango_program.key,
        )?;
        // msg!("perp_info {:?}", perp_info);
        Ok(perp_info)
    }

    // Return general information about the underlying SerumDex spot market related to the collateral in use
    fn spot_info(&self) -> UxdResult<SpotInfo> {
        let spot_info = SpotInfo::new(
            &self.mango_group,
            &self.mango_cache,
            &self.spot_market,
            self.mango_program.key,
            self.serum_dex_program.key,
        )?;
        msg!("spot_info {:?}", spot_info);
        Ok(spot_info)
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

    // Return the collateral balance for the Depository Mango Account in native units
    fn get_collateral_native_deposit(&self) -> UxdResult<I80F48> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.depository_mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        let mango_group = MangoGroup::load_checked(&self.mango_group, &self.mango_program.key)?;
        let token_index = mango_group
            .find_root_bank_index(&self.base_root_bank.key)
            .ok_or(throw_err!(UxdErrorCode::InvalidRootBank))?;
        Ok(mango_account.deposits[token_index])
    }

    // Update the accounting in the Depository Account to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_delta: u64,
        fee_delta: u64,
        rebalancing_side: &mango::matching::Side,
    ) -> UxdResult {
        let event = match rebalancing_side {
            mango::matching::Side::Bid => AccountingEvent::Deposit,
            mango::matching::Side::Ask => AccountingEvent::Withdraw,
        };
        self.depository
            .update_collateral_amount_deposited(&event, collateral_delta)?;
        self.depository
            .update_total_amount_paid_taker_fee(fee_delta)?;
        Ok(())
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
}

impl<'info> RebalanceMangoDepository<'info> {
    pub fn validate(&self, max_rebalancing_amount: u64, slippage: u32) -> ProgramResult {
        check!(slippage <= SLIPPAGE_BASIS, UxdErrorCode::InvalidSlippage)?;
        check!(
            max_rebalancing_amount > 0,
            UxdErrorCode::InvalidRebalancingAmount
        )?;
        // If the amount is beyond what the depository hold it will error later.
        Ok(())
    }
}
