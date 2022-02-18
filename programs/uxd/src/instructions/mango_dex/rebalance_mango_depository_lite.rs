use crate::check_assert;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdIdlErrorCode;
use crate::mango_program;
use crate::mango_utils::check_effective_order_price_versus_limit_price;
use crate::mango_utils::check_perp_order_fully_filled;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::get_best_order_for_quote_lot_amount;
use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::Order;
use crate::mango_utils::PerpInfo;
use crate::AccountingEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdError;
use crate::UxdErrorCode;
use crate::UxdResult;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::QUOTE_PASSTHROUGH_NAMESPACE;
use crate::SLIPPAGE_BASIS;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use mango::matching::BookSide;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;
use mango::state::PerpMarket;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexRebalanceMangoDepositoryLite);

const SUPPORTED_DEPOSITORY_VERSION: u8 = 2;

/// Takes 29 accounts - 11 used locally - 13 for MangoMarkets CPI - 4 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct RebalanceMangoDepositoryLite<'info> {
    /// #1 Public call accessible to any user
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository,
        constraint = depository.version >= SUPPORTED_DEPOSITORY_VERSION @UxdIdlErrorCode::UnsupportedDepositoryVersion
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 The collateral mint used by the `depository` instance
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6 The quote mint used by the `depository` instance
    #[account(
        constraint = quote_mint.key() == depository.quote_mint @UxdIdlErrorCode::InvalidQuoteMint
    )]
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #7 The `user`'s ATA for the `depository`'s `collateral_mint`
    /// Will be debited during this instruction when `Polarity` is positive
    /// Will be credited during this instruction when `Polarity` is negative
    #[account(
        init_if_needed,
        associated_token::mint = collateral_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s ATA for the `depository`'s `quote_mint`
    /// Will be credited during this instruction when `Polarity` is positive
    /// Will be debited during this instruction when `Polarity` is negative
    #[account(
        init_if_needed,
        associated_token::mint = quote_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_quote: Box<Account<'info, TokenAccount>>,

    /// #9 The `depository`'s TA for its `collateral_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.collateral_passthrough_bump,
        constraint = depository.collateral_passthrough == depository_collateral_passthrough_account.key() @UxdIdlErrorCode::InvalidCollateralPassthroughAccount,
        constraint = depository_collateral_passthrough_account.mint == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralPassthroughATAMint
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #10 The `depository`'s TA for its `quote_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [QUOTE_PASSTHROUGH_NAMESPACE, depository.key().as_ref()],
        bump= depository.quote_passthrough_bump,
        constraint = depository.quote_passthrough == depository_quote_passthrough_account.key() @UxdIdlErrorCode::InvalidQuotePassthroughAccount,
        constraint = depository_quote_passthrough_account.mint == depository.quote_mint @UxdIdlErrorCode::InvalidQuotePassthroughATAMint
    )]
    pub depository_quote_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #11 The MangoMarkets Account (MangoAccount) managed by the `depository`
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdIdlErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// #12 [MangoMarkets CPI] Signer PDA
    pub mango_signer: AccountInfo<'info>,

    /// #13 [MangoMarkets CPI] Index grouping perp and spot markets
    pub mango_group: AccountInfo<'info>,

    /// #14 [MangoMarkets CPI] Cache
    pub mango_cache: AccountInfo<'info>,

    /// #15 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`
    pub mango_root_bank_quote: AccountInfo<'info>,

    /// #16 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`
    #[account(mut)]
    pub mango_node_bank_quote: AccountInfo<'info>,

    /// #17 [MangoMarkets CPI] Vault `depository`'s `quote_mint`
    #[account(mut)]
    pub mango_vault_quote: AccountInfo<'info>,

    /// #18 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    pub mango_root_bank_collateral: AccountInfo<'info>,

    /// #19 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    #[account(mut)]
    pub mango_node_bank_collateral: AccountInfo<'info>,

    /// #20 [MangoMarkets CPI] Vault for `depository`'s `collateral_mint`
    #[account(mut)]
    pub mango_vault_collateral: AccountInfo<'info>,

    /// #21 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,

    /// #22 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,

    /// #23 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,

    /// #24 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,

    /// #25 System Program
    pub system_program: Program<'info, System>,

    /// #26 Token Program
    pub token_program: Program<'info, Token>,

    /// #27 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #28 MangoMarketv3 Program
    pub mango_program: Program<'info, mango_program::Mango>,

    /// #29 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RebalanceMangoDepositoryLite>,
    max_rebalancing_amount: u64,
    polarity: &PnlPolarity,
    slippage: u32,
) -> UxdResult {
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - [Get perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // Note : This will be implemented when we have more computing.
    //      It will move the redeemable_pnl to the spot balance
    //      As it is currently, we might borrow or keep positive redeem balance until third party settlement.
    // - [settle funding] TODO

    // - 1 [FIND CURRENT UNREALIZED PNL AMOUNT]

    // - [find out current perp Unrealized PnL]
    let perp_contract_size = perp_info.base_lot_size;
    let perp_position_notional_size = I80F48::from_num(pre_pa.base_position)
        .checked_mul(perp_contract_size)
        .ok_or(math_err!())?
        .checked_mul(perp_info.price)
        .ok_or(math_err!())?
        .abs();

    // Here will be overflow some day (u128 -> I80F48)
    let redeemable_amount_under_management =
        I80F48::checked_from_num(ctx.accounts.depository.redeemable_amount_under_management)
            .ok_or(math_err!())?;

    let perp_unrealized_pnl = redeemable_amount_under_management
        .checked_sub(perp_position_notional_size)
        .ok_or(math_err!())?;

    if perp_unrealized_pnl.is_zero() {
        return Ok(());
    }
    // We could get rid of the polarity parameter, but better have the user specify
    match polarity {
        PnlPolarity::Positive => check!(
            perp_unrealized_pnl.is_positive(),
            UxdErrorCode::InvalidPnlPolarity
        )?,
        PnlPolarity::Negative => check!(
            perp_unrealized_pnl.is_negative(),
            UxdErrorCode::InvalidPnlPolarity
        )?,
    }
    // - [rebalancing limited to `max_rebalancing_amount`, up to `perp_unrealized_pnl`]
    let rebalancing_quote_amount = perp_unrealized_pnl
        .abs()
        .checked_to_num::<u64>()
        .ok_or(math_err!())?
        .min(max_rebalancing_amount);

    // - 2 [FIND BEST ORDER FOR SHORT PERP POSITION (depending of Polarity)] --

    // - [Get the amount of Quote Lots for the perp order]
    let rebalancing_amount = I80F48::from_num(rebalancing_quote_amount)
        .checked_div(perp_info.quote_lot_size)
        .ok_or(math_err!())?
        .floor();

    // - [Estimate the best perp order depending of polarity]
    // Note : The caller is the Taker, the side depend of the PnL Polarity.
    let taker_side = match polarity {
        // Note : Reduce the delta neutral position, increasing long exposure, by buying perp.
        //        [BID: taker (us, the caller) | ASK: maker]
        PnlPolarity::Positive => Side::Bid,
        // Note : Augment the delta neutral position, increasing short exposure, by selling perp.
        //        [BID: maker | ASK: taker (us, the caller)]
        PnlPolarity::Negative => Side::Ask,
    };
    let quote_lot_amount = rebalancing_amount.checked_to_num().ok_or(math_err!())?;
    let perp_order = ctx
        .accounts
        .get_best_order_for_quote_lot_amount_from_order_book(taker_side, quote_lot_amount)?;

    // - [Checks that the best price found is within slippage range]
    check_effective_order_price_versus_limit_price(&perp_info, &perp_order, slippage)?;

    // - 3 [PlACE SHORT PERP] -------------------------------------------------

    // - [Base depository's position size in native units PRE perp order (to calculate the % filled later on)]
    let initial_base_position = total_perp_base_lot_position(&pre_pa)?;

    // - [Place perp order CPI to Mango Market v3]
    let reduce_only = perp_order.taker_side == Side::Bid;
    mango_program::place_perp_order(
        ctx.accounts
            .into_place_perp_order_context()
            .with_signer(depository_signer_seed),
        perp_order.price,
        perp_order.quantity,
        0,
        perp_order.taker_side,
        mango::matching::OrderType::ImmediateOrCancel,
        reduce_only,
    )?;

    // - [Perp account state POST perp order]
    let post_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled]
    let post_perp_order_base_lot_position = total_perp_base_lot_position(&post_pa)?;
    check_perp_order_fully_filled(
        perp_order.quantity,
        initial_base_position,
        post_perp_order_base_lot_position,
    )?;

    // - 4 [TRANSFER COLLATERAL/QUOTE TO MANGO (depending of Polarity)] -------
    // - 5 [TRANSFER QUOTE/COLLATERAL TO USER (depending of Polarity)] --------
    // Note : This is a workaround due to being limited by the number of accounts per instruction (~34)
    //          and how MangoMarketv3 is designed.
    //        As we cannot process a Perp and Spot order in a single atomic transaction, we use this
    //          detour to offload the Spot order.
    //        [4] will deposit either COLLATERAL or QUOTE depending of the PnL Polarity
    //        [5] will return the equivalent value of QUOTE or COLLATERAL depending of the PnL Polarity
    //

    // - [Calculate order deltas to proceed to transfers]
    // ensures current context make sense as the derive_order_delta is generic
    match polarity {
        PnlPolarity::Positive => check!(
            pre_pa.taker_quote < post_pa.taker_quote,
            UxdErrorCode::InvalidOrderDirection
        )?,
        PnlPolarity::Negative => check!(
            pre_pa.taker_quote > post_pa.taker_quote,
            UxdErrorCode::InvalidOrderDirection
        )?,
    };
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;

    match polarity {
        PnlPolarity::Positive => {
            // - 4 [TRANSFER COLLATERAL TO MANGO] -----------------------------
            // - [Transferring user collateral to the passthrough account]
            token::transfer(
                ctx.accounts
                    .into_transfer_collateral_from_user_to_passthrough_context(),
                order_delta.collateral,
            )?;

            // - [Deposit collateral to MangoAccount]
            mango_program::deposit(
                ctx.accounts
                    .into_deposit_collateral_from_passthrough_to_mango_context()
                    .with_signer(depository_signer_seed),
                order_delta.collateral,
            )?;
            // - 5 [TRANSFER QUOTE TO USER (Minus Taker Fees)] ----------------
            let quote_delta = order_delta
                .quote
                .checked_sub(order_delta.fee)
                .ok_or(math_err!())?;
            // - [Withdraw mango quote to the passthrough account]
            mango_program::withdraw(
                ctx.accounts
                    .into_withdraw_quote_from_mango_to_passthrough_context()
                    .with_signer(depository_signer_seed),
                quote_delta,
                false,
            )?;

            // - Return insurance_amount back to authority
            token::transfer(
                ctx.accounts
                    .into_transfer_quote_from_passthrough_to_user_context()
                    .with_signer(depository_signer_seed),
                quote_delta,
            )?;
        }
        PnlPolarity::Negative => {
            // - 4 [TRANSFER QUOTE TO MANGO (Plus Taker Fees)] ----------------------------------
            let quote_delta = order_delta
                .quote
                .checked_add(order_delta.fee)
                .ok_or(math_err!())?;
            // - [Transfers user quote to the passthrough account]
            token::transfer(
                ctx.accounts
                    .into_transfer_quote_from_user_to_passthrough_context(),
                quote_delta,
            )?;

            // - [Deposit quote to MangoAccount]
            mango_program::deposit(
                ctx.accounts
                    .into_deposit_quote_from_passthrough_to_mango_context()
                    .with_signer(depository_signer_seed),
                quote_delta,
            )?;
            // - 5 [TRANSFER COLLATERAL TO USER] ------------------------------
            // - [Mango withdraw CPI]
            mango_program::withdraw(
                ctx.accounts
                    .into_withdraw_collateral_from_mango_to_passthrough_context()
                    .with_signer(depository_signer_seed),
                order_delta.collateral,
                false,
            )?;

            // - [Return collateral back to user ATA]
            token::transfer(
                ctx.accounts
                    .into_transfer_collateral_from_passthrough_to_user_context()
                    .with_signer(depository_signer_seed),
                order_delta.collateral,
            )?;

            // Note : Too short in computing for now. Add again later
            // - [If ATA mint is WSOL, unwrap]
            // if ctx.accounts.depository.collateral_mint == spl_token::native_mint::id() {
            //     token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
            // }
        }
    }

    // - 6 [UPDATE ACCOUNTING] ------------------------------------------------

    ctx.accounts.update_onchain_accounting(
        order_delta.collateral,
        order_delta.quote,
        order_delta.fee,
        polarity,
    )?;

    // Note : Add later when computing limit is not an issue anymore
    // emit!(RebalanceMangoDepositoryLiteEvent {
    //     version: ctx.accounts.controller.version,
    //     depository_version: ctx.accounts.depository.version,
    //     controller: ctx.accounts.controller.key(),
    //     depository: ctx.accounts.depository.key(),
    //     user: ctx.accounts.user.key(),
    //     polarity: polarity.clone(),
    //     rebalancing_amount: max_rebalancing_amount,
    //     rebalanced_amount: rebalancing_quote_amount,
    //     slippage,
    //     collateral_delta: order_delta.collateral,
    //     quote_delta: order_delta.quote,
    //     fee_delta: order_delta.fee,
    // });

    Ok(())
}

impl<'info> RebalanceMangoDepositoryLite<'info> {
    pub fn into_transfer_collateral_from_user_to_passthrough_context(
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

    pub fn into_deposit_collateral_from_passthrough_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank_collateral.to_account_info(),
            mango_node_bank: self.mango_node_bank_collateral.to_account_info(),
            mango_vault: self.mango_vault_collateral.to_account_info(),
            token_program: self.token_program.to_account_info(),
            owner_token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_quote_from_user_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_quote.to_account_info(),
            to: self.depository_quote_passthrough_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_quote_from_passthrough_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Deposit<'info>> {
        let cpi_accounts = mango_program::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank_quote.to_account_info(),
            mango_node_bank: self.mango_node_bank_quote.to_account_info(),
            mango_vault: self.mango_vault_quote.to_account_info(),
            token_program: self.token_program.to_account_info(),
            owner_token_account: self.depository_quote_passthrough_account.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_quote_from_mango_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Withdraw<'info>> {
        let cpi_accounts = mango_program::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank_quote.to_account_info(),
            mango_node_bank: self.mango_node_bank_quote.to_account_info(),
            mango_vault: self.mango_vault_quote.to_account_info(),
            token_account: self.depository_quote_passthrough_account.to_account_info(),
            mango_signer: self.mango_signer.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_quote_from_passthrough_to_user_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: self.depository_quote_passthrough_account.to_account_info(),
            to: self.user_quote.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_collateral_from_mango_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_program::Withdraw<'info>> {
        let cpi_accounts = mango_program::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            mango_root_bank: self.mango_root_bank_collateral.to_account_info(),
            mango_node_bank: self.mango_node_bank_collateral.to_account_info(),
            mango_vault: self.mango_vault_collateral.to_account_info(),
            token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            mango_signer: self.mango_signer.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_collateral_from_passthrough_to_user_context(
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

    pub fn into_place_perp_order_context(
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

    pub fn into_unwrap_wsol_by_closing_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::CloseAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::CloseAccount {
            account: self.user_collateral.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> RebalanceMangoDepositoryLite<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> UxdResult<PerpInfo> {
        let perp_info = PerpInfo::new(
            &self.mango_group,
            &self.mango_cache,
            self.mango_perp_market.key,
            self.mango_program.key,
        )?;
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

    fn get_best_order_for_quote_lot_amount_from_order_book(
        &self,
        taker_side: Side,
        quote_lot_amount: i64,
    ) -> UxdResult<Order> {
        let perp_market = PerpMarket::load_checked(
            &self.mango_perp_market,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        // Load the maker side of the book
        let book_maker_side = match taker_side {
            Side::Bid => {
                BookSide::load_mut_checked(&self.mango_asks, self.mango_program.key, &perp_market)
            }
            Side::Ask => {
                BookSide::load_mut_checked(&self.mango_bids, self.mango_program.key, &perp_market)
            }
        }?;
        // Search for the best order to spend the given amount of quote lot
        get_best_order_for_quote_lot_amount(book_maker_side, taker_side, quote_lot_amount)
    }

    fn update_onchain_accounting(
        &mut self,
        collateral_delta: u64,
        quote_delta: u64,
        fee_delta: u64,
        polarity: &PnlPolarity,
    ) -> UxdResult {
        // Mango Depository
        let event = match polarity {
            PnlPolarity::Positive => AccountingEvent::Deposit,
            PnlPolarity::Negative => AccountingEvent::Withdraw,
        };
        self.depository
            .update_collateral_amount_deposited(&event, collateral_delta)?;
        self.depository.update_rebalanced_amount(quote_delta)?;
        self.depository
            .update_total_amount_paid_taker_fee(fee_delta)?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> RebalanceMangoDepositoryLite<'info> {
    pub fn validate(
        &self,
        max_rebalancing_amount: u64,
        polarity: &PnlPolarity,
        slippage: u32,
    ) -> ProgramResult {
        // Valid slippage check
        check!(
            (slippage > 0) && (slippage <= SLIPPAGE_BASIS),
            UxdErrorCode::InvalidSlippage
        )?;

        // Rebalancing amount must be above 0
        check!(
            max_rebalancing_amount > 0,
            UxdErrorCode::InvalidRebalancingAmount
        )?;

        // Rebalancing amount must be above 0
        match polarity {
            PnlPolarity::Positive => (), // Checked later
            PnlPolarity::Negative => check!(
                self.user_quote.amount >= max_rebalancing_amount,
                UxdErrorCode::InsufficientQuoteAmount
            )?,
        };
        Ok(())
    }
}

// Represent the direction of the Delta Neutral position (short perp) PnL of a MangoDepository.
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PnlPolarity {
    Positive,
    Negative,
}

impl std::fmt::Display for PnlPolarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PnlPolarity::Positive => f.write_str("Positive"),
            PnlPolarity::Negative => f.write_str("Negative"),
        }
    }
}
