use crate::error::UxdError;
use crate::events::RebalanceMangoDepositoryLiteEvent;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::price_to_lot_price;
use crate::mango_utils::total_perp_base_lot_position;
use crate::mango_utils::PerpInfo;
use crate::validate_perp_market_mints_matches_depository_mints;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MANGO_PERP_MAX_FILL_EVENTS;
use anchor_comp::mango_markets_v3;
use anchor_comp::mango_markets_v3::MangoMarketV3;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::matching::OrderType;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;

/// Takes 25 accounts
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
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = mango_account @UxdError::InvalidMangoAccount,
        has_one = quote_mint @UxdError::InvalidQuoteMint,
        has_one = collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 The collateral mint used by the `depository` instance
    /// Required to create the user_collateral ATA if needed
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6 The quote mint used by the `depository` instance
    /// Required to create the user_quote ATA if needed
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #7 The `user`'s TA for the `depository`'s `collateral_mint`
    /// Will be debited during this instruction when `Polarity` is positive
    /// Will be credited during this instruction when `Polarity` is negative
    #[account(
        mut,
        constraint = user_collateral.mint == depository.load()?.collateral_mint @UxdError::InvalidCollateralMint,
        constraint = &user_collateral.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s TA for the `depository`'s `quote_mint`
    /// Will be credited during this instruction when `Polarity` is positive
    /// Will be debited during this instruction when `Polarity` is negative
    #[account(
        mut,
        constraint = user_quote.mint == depository.load()?.quote_mint @UxdError::InvalidQuoteMint,
        constraint = &user_quote.owner == user.key @UxdError::InvalidOwner,
    )]
    pub user_quote: Box<Account<'info, TokenAccount>>,

    /// #9 The MangoMarkets Account (MangoAccount) managed by the `depository`
    /// CHECK : Seeds checked. Depository registered
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.load()?.mango_account_bump,
    )]
    pub mango_account: AccountInfo<'info>,

    /// #10 [MangoMarkets CPI] Signer PDA
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    pub mango_signer: UncheckedAccount<'info>,

    /// #11 [MangoMarkets CPI] Index grouping perp and spot markets
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_group: UncheckedAccount<'info>,

    /// #12 [MangoMarkets CPI] Cache
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_cache: UncheckedAccount<'info>,

    /// #13 [MangoMarkets CPI] Root Bank for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_root_bank_quote: UncheckedAccount<'info>,

    /// #14 [MangoMarkets CPI] Node Bank for the `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank_quote: UncheckedAccount<'info>,

    /// #15 [MangoMarkets CPI] Vault `depository`'s `quote_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault_quote: UncheckedAccount<'info>,

    /// #16 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_root_bank_collateral: UncheckedAccount<'info>,

    /// #17 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_node_bank_collateral: UncheckedAccount<'info>,

    /// #18 [MangoMarkets CPI] Vault for `depository`'s `collateral_mint`
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_vault_collateral: UncheckedAccount<'info>,

    /// #19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_perp_market: UncheckedAccount<'info>,

    /// #20 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_bids: UncheckedAccount<'info>,

    /// #21 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_asks: UncheckedAccount<'info>,

    /// #22 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    /// CHECK: Mango CPI - checked MangoMarketV3 side
    #[account(mut)]
    pub mango_event_queue: UncheckedAccount<'info>,

    /// #23 System Program
    pub system_program: Program<'info, System>,

    /// #24 Token Program
    pub token_program: Program<'info, Token>,

    /// #25 MangoMarketv3 Program
    pub mango_program: Program<'info, MangoMarketV3>,
}

pub fn handler(
    ctx: Context<RebalanceMangoDepositoryLite>,
    max_rebalancing_amount: u64,
    polarity: &PnlPolarity,
    limit_price: f32,
) -> Result<()> {
    let depository = ctx.accounts.depository.load()?;
    let collateral_mint = depository.collateral_mint;
    let depository_bump = depository.bump;
    let redeemable_amount_under_management = depository.redeemable_amount_under_management;
    drop(depository);

    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        collateral_mint.as_ref(),
        &[depository_bump],
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
    let redeemable_under_management = i128::try_from(redeemable_amount_under_management)
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
            require!(
                perp_unrealized_pnl.is_positive(),
                UxdError::InvalidPnlPolarity
            );
        }
        PnlPolarity::Negative => {
            require!(
                perp_unrealized_pnl.is_negative(),
                UxdError::InvalidPnlPolarity
            );
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
    let limit_price_fixed =
        I80F48::checked_from_num(limit_price).ok_or_else(|| error!(UxdError::MathError))?;
    let limit_price_lot = price_to_lot_price(limit_price_fixed, &perp_info)?;
    let reduce_only = taker_side == Side::Bid;

    require!(max_quote_quantity != 0, UxdError::QuantityBelowContractSize);

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
            require!(
                pre_pa.taker_quote <= post_pa.taker_quote,
                UxdError::InvalidOrderDirection
            );
        }
        PnlPolarity::Negative => {
            require!(
                pre_pa.taker_quote >= post_pa.taker_quote,
                UxdError::InvalidOrderDirection
            );
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

            // - [Deposit collateral to MangoAccount]
            mango_markets_v3::deposit(
                ctx.accounts
                    .into_deposit_user_collateral_to_mango_context()
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
                    .into_withdraw_quote_from_mango_to_user_context()
                    .with_signer(depository_signer_seed),
                quote_withdraw_amount,
                false, // Settle PNL before calling this IX if this fails
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

            // - [Deposit quote to MangoAccount]
            mango_markets_v3::deposit(
                ctx.accounts
                    .into_deposit_user_quote_to_mango_context()
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
                    .into_withdraw_collateral_from_mango_to_user_context()
                    .with_signer(depository_signer_seed),
                collateral_withdraw_amount,
                false,
            )?;

            // - [If ATA mint is WSOL, unwrap]
            if collateral_mint == spl_token::native_mint::id() {
                token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
            }

            // - 6 [UPDATE ACCOUNTING] ------------------------------------------------
            ctx.accounts.update_onchain_accounting_negative_pnl(
                collateral_withdraw_amount.into(),
                quote_deposit_amount.into(),
                order_delta.fee.abs().to_num(),
            )?;
        }
    }

    emit!(RebalanceMangoDepositoryLiteEvent {
        version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        polarity: polarity.clone(),
        rebalancing_amount: max_rebalancing_amount,
        rebalanced_amount: rebalancing_quote_amount.to_num(),
        limit_price,
        base_delta: order_delta.base.to_num(),
        quote_delta: order_delta.quote.to_num(),
        fee_delta: order_delta.fee.to_num(),
    });

    Ok(())
}

impl<'info> RebalanceMangoDepositoryLite<'info> {
    pub fn into_deposit_user_collateral_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.user.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank_collateral.to_account_info(),
            node_bank: self.mango_node_bank_collateral.to_account_info(),
            vault: self.mango_vault_collateral.to_account_info(),
            owner_token_account: self.user_collateral.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_user_quote_to_mango_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Deposit<'info>> {
        let cpi_accounts = mango_markets_v3::Deposit {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.user.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank_quote.to_account_info(),
            node_bank: self.mango_node_bank_quote.to_account_info(),
            vault: self.mango_vault_quote.to_account_info(),
            owner_token_account: self.user_quote.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_quote_from_mango_to_user_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank_quote.to_account_info(),
            node_bank: self.mango_node_bank_quote.to_account_info(),
            vault: self.mango_vault_quote.to_account_info(),
            token_account: self.user_quote.to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_collateral_from_mango_to_user_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::Withdraw<'info>> {
        let cpi_accounts = mango_markets_v3::Withdraw {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
            owner: self.depository.to_account_info(),
            mango_cache: self.mango_cache.to_account_info(),
            root_bank: self.mango_root_bank_collateral.to_account_info(),
            node_bank: self.mango_node_bank_collateral.to_account_info(),
            vault: self.mango_vault_collateral.to_account_info(),
            token_account: self.user_collateral.to_account_info(),
            signer: self.mango_signer.to_account_info(),
        };
        let cpi_program = self.mango_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_place_perp_order_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::PlacePerpOrder2<'info>> {
        let cpi_accounts = mango_markets_v3::PlacePerpOrder2 {
            mango_group: self.mango_group.to_account_info(),
            mango_account: self.mango_account.to_account_info(),
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
            &self.mango_account,
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
            &self.mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )
        .map_err(ProgramError::from)?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    fn update_onchain_accounting_negative_pnl(
        &mut self,
        collateral_withdrawn_amount: u128,
        rebalanced_amount: u128,
        fee_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository.load_mut()?;
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_sub(collateral_withdrawn_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_rebalanced = depository
            .total_amount_rebalanced
            .checked_add(rebalanced_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_paid_taker_fee = depository
            .total_amount_paid_taker_fee
            .checked_add(fee_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        Ok(())
    }

    fn update_onchain_accounting_positive_pnl(
        &mut self,
        collateral_deposited_amount: u128,
        rebalanced_amount: u128,
        fee_amount: u128,
    ) -> Result<()> {
        let depository = &mut self.depository.load_mut()?;
        depository.collateral_amount_deposited = depository
            .collateral_amount_deposited
            .checked_add(collateral_deposited_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_rebalanced = depository
            .total_amount_rebalanced
            .checked_add(rebalanced_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
        depository.total_amount_paid_taker_fee = depository
            .total_amount_paid_taker_fee
            .checked_add(fee_amount)
            .ok_or_else(|| error!(UxdError::MathError))?;
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
        require!(limit_price > 0f32, UxdError::InvalidLimitPrice);
        require!(
            max_rebalancing_amount != 0,
            UxdError::InvalidRebalancingAmount
        );

        match polarity {
            PnlPolarity::Positive => (),
            PnlPolarity::Negative => {
                require!(
                    self.user_quote.amount >= max_rebalancing_amount,
                    UxdError::InsufficientQuoteAmount
                );
            }
        }

        validate_perp_market_mints_matches_depository_mints(
            &self.mango_group,
            self.mango_program.key,
            self.mango_perp_market.key,
            &self.depository.load()?.collateral_mint,
            &self.depository.load()?.quote_mint,
        )?;

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
