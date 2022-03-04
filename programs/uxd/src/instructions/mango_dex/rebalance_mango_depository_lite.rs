use crate::error::UxdError;
use crate::MANGO_PERP_MAX_FILL_EVENTS;
// use crate::events::RebalanceMangoDepositoryLiteEvent;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::price_to_lot_price;
use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::PerpInfo;
use crate::Controller;
use crate::MangoDepository;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::QUOTE_PASSTHROUGH_NAMESPACE;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use mango::matching::OrderType;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;

const SUPPORTED_DEPOSITORY_VERSION: u8 = 2;

/// Takes 29 accounts - 11 used locally - 13 for MangoMarkets CPI - 4 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct RebalanceMangoDepositoryLite<'info> {
    /// #1 Public call accessible to any user
    /// Note - Mut required for WSOL unwrapping
    // #[account(mut)]
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
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        constraint = depository.version >= SUPPORTED_DEPOSITORY_VERSION @UxdError::UnsupportedDepositoryVersion
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 The collateral mint used by the `depository` instance
    /// Required to create the user_collateral ATA if needed
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6 The quote mint used by the `depository` instance
    /// Required to create the user_quote ATA if needed
    #[account(
        constraint = quote_mint.key() == depository.quote_mint @UxdError::InvalidQuoteMint
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
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.collateral_passthrough_bump,
        constraint = depository.collateral_passthrough == depository_collateral_passthrough_account.key() @UxdError::InvalidCollateralPassthroughAccount,
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #10 The `depository`'s TA for its `quote_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        mut,
        seeds = [QUOTE_PASSTHROUGH_NAMESPACE, depository.key().as_ref()],
        bump= depository.quote_passthrough_bump,
        constraint = depository.quote_passthrough == depository_quote_passthrough_account.key() @UxdError::InvalidQuotePassthroughAccount,
    )]
    pub depository_quote_passthrough_account: Box<Account<'info, TokenAccount>>,

    /// #11 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdError::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// #12 [MangoMarkets CPI] Signer PDA
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_signer: UncheckedAccount<'info>,

    /// #13 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_group: UncheckedAccount<'info>,

    /// #14 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_cache: UncheckedAccount<'info>,

    /// #15 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_root_bank_quote: UncheckedAccount<'info>,

    /// #16 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank_quote: UncheckedAccount<'info>,

    /// #17 [MangoMarkets CPI] Vault `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault_quote: UncheckedAccount<'info>,

    /// #18 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_root_bank_collateral: UncheckedAccount<'info>,

    /// #19 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank_collateral: UncheckedAccount<'info>,

    /// #20 [MangoMarkets CPI] Vault for `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault_collateral: UncheckedAccount<'info>,

    /// #21 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,

    /// #22 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_bids: UncheckedAccount<'info>,

    /// #23 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_asks: UncheckedAccount<'info>,

    /// #24 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_event_queue: UncheckedAccount<'info>,

    /// #25 System Program
    pub system_program: Program<'info, System>,

    /// #26 Token Program
    pub token_program: Program<'info, Token>,

    /// #27 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #28 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,

    /// #29 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RebalanceMangoDepositoryLite>,
    max_rebalancing_amount: u64,
    polarity: &PnlPolarity,
    limit_price: f32,
) -> Result<()> {
    let depository = &ctx.accounts.depository;
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        depository.collateral_mint.as_ref(),
        &[depository.bump],
    ]];

    // - [Get perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // - 1 [FIND CURRENT UNREALIZED PNL AMOUNT]

    // - [find out current perp Unrealized PnL]
    let contract_size = perp_info.base_lot_size;
    // Note : Loose precision but an average value is fine here, we just want a value close to the current PnL
    let perp_position_notional_size: i128 =
        I80F48::from_num(total_perp_base_lot_position(&pre_pa)?)
            .checked_mul(contract_size)
            .ok_or_else(|| error!(UxdError::MathError))?
            .checked_mul(perp_info.price)
            .ok_or_else(|| error!(UxdError::MathError))?
            .abs()
            .checked_to_num()
            .ok_or_else(|| error!(UxdError::MathError))?;

    // The perp position unrealized PnL is equal to the outstanding amount of redeemable
    // minus the perp position notional size in quote.
    // Ideally they stay 1:1, to have the redeemable fully backed by the delta neutral
    // position and no paper profits.
    let redeemable_under_management = i128::try_from(depository.redeemable_amount_under_management)
        .map_err(|_e| error!(UxdError::MathError))?;

    // Will not overflow as `perp_position_notional_size` and `redeemable_under_management`
    // will vary together.
    let perp_unrealized_pnl = I80F48::checked_from_num(
        redeemable_under_management
            .checked_sub(perp_position_notional_size)
            .ok_or_else(|| error!(UxdError::MathError))?,
    )
    .ok_or_else(|| error!(UxdError::MathError))?;

    // Polarity parameter could be inferred, but is requested as input to prevent users
    // user rebalancing (swapping) in an undesired way, as the PnL could technically shift
    // between call and execution time.
    // This also filter out the case where `perp_unrealized_pnl` is 0
    match polarity {
        PnlPolarity::Positive => {
            if perp_unrealized_pnl.is_negative() {
                return Err(error!(UxdError::InvalidPnlPolarity));
            }
        }
        PnlPolarity::Negative => {
            if perp_unrealized_pnl.is_positive() {
                return Err(error!(UxdError::InvalidPnlPolarity));
            }
        }
    }
    // - [rebalancing limited to `max_rebalancing_amount`, up to `perp_unrealized_pnl`]
    let requested_rebalancing_amount = I80F48::from_num(max_rebalancing_amount);
    let rebalancing_quote_amount = perp_unrealized_pnl.abs().min(requested_rebalancing_amount);

    // - 2 [FIND BEST ORDER FOR SHORT PERP POSITION (depending of Polarity)] --

    // - [Plan the rebalancing amount]
    // Note : Depending of the side, the fees don't come from the same place.
    //        If the PnL is positive, it behaves like a redeem and the fees are taken
    //        on the inputted amount (also here they aren't burnt and living in the DN
    //        position as we don't process redeemables.)
    //        If the PnL is negative, it behaves like the mint and the fees are taken
    //        on the returned amount (here they aren't living in the delta neutral position
    //        but simply on the spot QUOTE balance)
    let rebalancing_amount = match polarity {
        PnlPolarity::Positive => {
            // - [Find the max fees]
            let max_fee_amount = rebalancing_quote_amount
                .checked_mul(perp_info.effective_fee)
                .ok_or_else(|| error!(UxdError::MathError))?
                .checked_ceil()
                .ok_or_else(|| error!(UxdError::MathError))?;

            // - [Get the amount of quote_lots for the perp order minus fees not to overflow max_rebalancing_amount]
            rebalancing_quote_amount
                .checked_sub(max_fee_amount)
                .ok_or_else(|| error!(UxdError::MathError))?
                .checked_div(perp_info.quote_lot_size)
                .ok_or_else(|| error!(UxdError::MathError))?
                .checked_floor()
                .ok_or_else(|| error!(UxdError::MathError))
        }
        PnlPolarity::Negative => {
            // - [Get the amount of quote_lots for the perp order]
            rebalancing_quote_amount
                .checked_div(perp_info.quote_lot_size)
                .ok_or_else(|| error!(UxdError::MathError))?
                .checked_floor()
                .ok_or_else(|| error!(UxdError::MathError))
        }
    }?;

    // - 3 [PlACE SHORT PERP] -------------------------------------------------

    // - [Place perp order CPI to Mango Market v3]
    // Note : The caller is the Taker, the side depend of the PnL Polarity.
    let taker_side = match polarity {
        // Note : Augment the delta neutral position, increasing short exposure, by selling perp.
        //        [BID: maker | ASK: taker (us, the caller)]
        PnlPolarity::Positive => Side::Ask,
        // Note : Reduce the delta neutral position, increasing long exposure, by buying perp.
        //        [BID: taker (us, the caller) | ASK: maker]
        PnlPolarity::Negative => Side::Bid,
    };
    let max_quote_quantity = rebalancing_amount
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))?;
    let limit_price =
        I80F48::checked_from_num(limit_price).ok_or_else(|| error!(UxdError::MathError))?;
    let limit_price_lot = price_to_lot_price(limit_price, &perp_info)?;
    let reduce_only = taker_side == Side::Bid;

    mango_markets_v3::place_perp_order2(
        ctx.accounts
            .into_place_perp_order_context()
            .with_signer(depository_signer_seed),
        taker_side,
        limit_price_lot.to_num(),
        i64::MAX,
        max_quote_quantity,
        0,
        OrderType::ImmediateOrCancel,
        reduce_only,
        None,
        MANGO_PERP_MAX_FILL_EVENTS,
    )?;

    // - [Perp account state POST perp order]
    let post_pa = ctx.accounts.perp_account(&perp_info)?;

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
        PnlPolarity::Positive => {
            if pre_pa.taker_quote > post_pa.taker_quote {
                return Err(error!(UxdError::InvalidOrderDirection));
            }
        }
        PnlPolarity::Negative => {
            if pre_pa.taker_quote < post_pa.taker_quote {
                return Err(error!(UxdError::InvalidOrderDirection));
            }
        }
    };
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
    msg!("order_delta {:?}", order_delta);
    match polarity {
        PnlPolarity::Positive => {
            // - 4 [TRANSFER COLLATERAL TO MANGO] -----------------------------
            let collateral_deposit_amount = order_delta
                .base
                .unsigned_abs()
                .checked_to_num()
                .ok_or_else(|| error!(UxdError::MathError))?;
            // - [Transferring user collateral to the passthrough account]
            token::transfer(
                ctx.accounts
                    .into_transfer_collateral_from_user_to_passthrough_context(),
                collateral_deposit_amount,
            )?;
            // - [Deposit collateral to MangoAccount]
            mango_markets_v3::deposit(
                ctx.accounts
                    .into_deposit_collateral_from_passthrough_to_mango_context()
                    .with_signer(depository_signer_seed),
                collateral_deposit_amount,
            )?;

            // - 5 [TRANSFER QUOTE TO USER (Minus Taker Fees)] ----------------
            let quote_withdraw_amount = order_delta
                .quote
                .checked_sub(order_delta.fee)
                .ok_or_else(|| error!(UxdError::MathError))?
                .unsigned_abs()
                .checked_to_num()
                .ok_or_else(|| error!(UxdError::MathError))?;
            // - [Withdraw mango quote to the passthrough account]
            mango_markets_v3::withdraw(
                ctx.accounts
                    .into_withdraw_quote_from_mango_to_passthrough_context()
                    .with_signer(depository_signer_seed),
                quote_withdraw_amount,
                false, // Settle PNL before calling this IX if this fails
            )?;
            token::transfer(
                ctx.accounts
                    .into_transfer_quote_from_passthrough_to_user_context()
                    .with_signer(depository_signer_seed),
                quote_withdraw_amount,
            )?;
            // - 6 [UPDATE ACCOUNTING] ------------------------------------------------
            ctx.accounts.update_onchain_accounting_positive_pnl(
                collateral_deposit_amount.into(),
                quote_withdraw_amount.into(),
                order_delta.fee.abs().to_num(),
            )?;
        }
        PnlPolarity::Negative => {
            // - 4 [TRANSFER QUOTE TO MANGO (Plus Taker Fees)] ----------------------------------
            let quote_deposit_amount = order_delta
                .quote
                .checked_add(order_delta.fee)
                .ok_or_else(|| error!(UxdError::MathError))?
                .unsigned_abs()
                .checked_to_num()
                .ok_or_else(|| error!(UxdError::MathError))?;
            // - [Transfers user quote to the passthrough account]
            token::transfer(
                ctx.accounts
                    .into_transfer_quote_from_user_to_passthrough_context(),
                quote_deposit_amount,
            )?;
            // - [Deposit quote to MangoAccount]
            mango_markets_v3::deposit(
                ctx.accounts
                    .into_deposit_quote_from_passthrough_to_mango_context()
                    .with_signer(depository_signer_seed),
                quote_deposit_amount,
            )?;

            // - 5 [TRANSFER COLLATERAL TO USER] ------------------------------
            let collateral_withdraw_amount = order_delta
                .base
                .unsigned_abs()
                .checked_to_num()
                .ok_or_else(|| error!(UxdError::MathError))?;
            // - [Mango withdraw CPI]
            mango_markets_v3::withdraw(
                ctx.accounts
                    .into_withdraw_collateral_from_mango_to_passthrough_context()
                    .with_signer(depository_signer_seed),
                collateral_withdraw_amount,
                false,
            )?;
            // - [Return collateral back to user ATA]
            token::transfer(
                ctx.accounts
                    .into_transfer_collateral_from_passthrough_to_user_context()
                    .with_signer(depository_signer_seed),
                collateral_withdraw_amount,
            )?;

            // - [If ATA mint is WSOL, unwrap]
            // Note - Computing too short for now
            // if depository.collateral_mint == spl_token::native_mint::id() {
            //     token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
            // }

            // - 6 [UPDATE ACCOUNTING] ------------------------------------------------
            ctx.accounts.update_onchain_accounting_negative_pnl(
                collateral_withdraw_amount.into(),
                quote_deposit_amount.into(),
                order_delta.fee.abs().to_num(),
            )?;
        }
    }

    // emit!(RebalanceMangoDepositoryLiteEvent {
    //     version: controller.version,
    //     depository_version: depository.version,
    //     controller: controller.key(),
    //     depository: depository.key(),
    //     user: ctx.accounts.user.key(),
    //     polarity: polarity.clone(),
    //     rebalancing_amount: max_rebalancing_amount,
    //     rebalanced_amount: rebalancing_quote_amount.to_num(),
    //     limit_price,
    //     base_delta: order_delta.base.to_num(),
    //     quote_delta: order_delta.quote.to_num(),
    //     fee_delta: order_delta.fee.to_num(),
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
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank_collateral.to_account_info(),
            node_bank: self.mango_node_bank_collateral.to_account_info(),
            vault: self.mango_vault_collateral.to_account_info(),
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
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank_quote.to_account_info(),
            node_bank: self.mango_node_bank_quote.to_account_info(),
            vault: self.mango_vault_quote.to_account_info(),
            owner_token_account: self.depository_quote_passthrough_account.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_quote_from_mango_to_passthrough_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank_quote.to_account_info(),
            node_bank: self.mango_node_bank_quote.to_account_info(),
            vault: self.mango_vault_quote.to_account_info(),
            token_account: self.depository_quote_passthrough_account.to_account_info(),
            signer: self.mango_signer.to_account_info(),
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
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank_collateral.to_account_info(),
            node_bank: self.mango_node_bank_collateral.to_account_info(),
            vault: self.mango_vault_collateral.to_account_info(),
            token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            signer: self.mango_signer.to_account_info(),
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
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::PlacePerpOrder2<'info>> {
        let cpi_accounts = mango_markets_v3::PlacePerpOrder2 {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.depository_mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            perp_market: self.mango_perp_market.to_account_info(),
            bids: self.mango_bids.to_account_info(),
            asks: self.mango_asks.to_account_info(),
            event_queue: self.mango_event_queue.to_account_info(),
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
    fn perpetual_info(&self) -> Result<PerpInfo> {
        let perp_info = PerpInfo::new(
            &self.mango_group,
            &self.mango_cache,
            &self.depository_mango_account,
            self.mango_perp_market.key,
            self.mango_group.key,
            self.mango_program.key,
        )?;
        msg!("perp_info {:?}", perp_info);
        Ok(perp_info)
    }

    // Return the PerpAccount that represent the account balances (Quote and Taker, Taker is the part that is waiting settlement)
    fn perp_account(&self, perp_info: &PerpInfo) -> Result<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.depository_mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )
        .map_err(|me| ProgramError::from(me))?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    fn update_onchain_accounting_negative_pnl(
        &mut self,
        collateral_withdrawn_amount: u128,
        rebalanced_amount: u128,
        fee_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository;
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_sub(collateral_withdrawn_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_rebalanced = depository
            .total_amount_rebalanced
            .wrapping_add(rebalanced_amount);
        depository.total_amount_paid_taker_fee = depository
            .total_amount_paid_taker_fee
            .wrapping_add(fee_amount);
        Ok(())
    }

    fn update_onchain_accounting_positive_pnl(
        &mut self,
        collateral_deposited_amount: u128,
        rebalanced_amount: u128,
        fee_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository;
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_add(collateral_deposited_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_rebalanced = depository
            .total_amount_rebalanced
            .wrapping_add(rebalanced_amount);
        depository.total_amount_paid_taker_fee = depository
            .total_amount_paid_taker_fee
            .wrapping_add(fee_amount);
        Ok(())
    }
}

// Validate input arguments
impl<'info> RebalanceMangoDepositoryLite<'info> {
    pub fn validate(
        &self,
        max_rebalancing_amount: u64,
        polarity: &PnlPolarity,
        limit_price: f32,
    ) -> Result<()> {
        if limit_price <= 0f32 {
            return Err(error!(UxdError::InvalidLimitPrice));
        }
        if max_rebalancing_amount == 0 {
            return Err(error!(UxdError::InvalidRebalancingAmount));
        }
        match polarity {
            PnlPolarity::Positive => (),
            PnlPolarity::Negative => {
                if self.user_quote.amount < max_rebalancing_amount {
                    return Err(error!(UxdError::InsufficientQuoteAmount));
                }
            }
        }
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
