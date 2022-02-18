use crate::check_assert;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
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
use crate::UxdResult;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::matching::BookSide;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;
use mango::state::PerpMarket;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexRedeemFromMangoDepository);

#[derive(Accounts)]
pub struct RedeemFromMangoDepository<'info> {
    /// Public call accessible to any user
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manager a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// The collateral mint used by the `depository` instance
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// The redeemable mint managed by the `controller` instance
    /// Tokens will be burnt during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @UxdIdlErrorCode::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// The `user`'s ATA for the `depository`'s `collateral_mint`
    /// Will be credited during this instruction
    #[account(
        init_if_needed,
        associated_token::mint = collateral_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        associated_token::mint = redeemable_mint,
        associated_token::authority = user,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// The `depository`'s TA for its `collateral_mint`
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

    /// The MangoMarkets Account (MangoAccount) managed by the `depository`
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdIdlErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// [MangoMarkets CPI] Index grouping perp and spot markets
    pub mango_group: AccountInfo<'info>,

    /// [MangoMarkets CPI] Cache
    pub mango_cache: AccountInfo<'info>,

    /// [MangoMarkets CPI] Signer PDA
    pub mango_signer: AccountInfo<'info>,

    /// [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    pub mango_root_bank: AccountInfo<'info>,

    /// [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,

    /// [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`
    #[account(mut)]
    pub mango_vault: AccountInfo<'info>,

    /// [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,

    /// [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,

    /// [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,

    /// [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,

    // System Program
    pub system_program: Program<'info, System>,

    /// Token Program
    pub token_program: Program<'info, Token>,

    /// Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// MangoMarketv3 Program
    pub mango_program: Program<'info, mango_program::Mango>,

    /// Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RedeemFromMangoDepository>,
    redeemable_amount: u64,
    slippage: u32,
) -> UxdResult {
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [CLOSE THE EQUIVALENT PERP SHORT ON MANGO] -------------------------

    // - [Get perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Calculates the quantity of short to close]
    let mut exposure_delta_in_quote_unit = I80F48::from_num(redeemable_amount);

    // - [Find the max taker fees mango will take on the perp order and remove it from the exposure delta to be sure the amount order + fees doesn't overflow the redeemed amount]
    let max_fee_amount = exposure_delta_in_quote_unit
        .checked_mul(perp_info.taker_fee)
        .ok_or(math_err!())?
        .checked_ceil()
        .ok_or(math_err!())?;
    exposure_delta_in_quote_unit = exposure_delta_in_quote_unit
        .checked_sub(max_fee_amount)
        .ok_or(math_err!())?;

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Base depository's position size in native units PRE perp opening (to calculate the % filled later on)]
    let initial_base_lot_position = total_perp_base_lot_position(&pre_pa)?;

    // - [Find out how the best price and quantity for our order]
    let exposure_delta_in_quote_lot_unit = exposure_delta_in_quote_unit
        .checked_div(perp_info.quote_lot_size)
        .ok_or(math_err!())?;
    // Note : Reduce the delta neutral position, increasing long exposure, by buying perp.
    //        [BID: taker (us, the caller) | ASK: maker]
    let taker_side = Side::Bid;
    let quote_lot_amount = exposure_delta_in_quote_lot_unit
        .checked_to_num()
        .ok_or(math_err!())?;
    let best_order = ctx
        .accounts
        .get_best_order_for_quote_lot_amount_from_order_book(taker_side, quote_lot_amount)?;

    // - [Checks that the best price found is withing slippage range]
    check_effective_order_price_versus_limit_price(&perp_info, &best_order, slippage)?;

    // - [Place perp order CPI to Mango Market v3]
    mango_program::place_perp_order(
        ctx.accounts
            .into_close_mango_short_perp_context()
            .with_signer(depository_signer_seed),
        best_order.price,
        best_order.quantity,
        0,
        best_order.taker_side,
        mango::matching::OrderType::ImmediateOrCancel,
        true,
    )?;

    // - [Perp account state POST perp order]
    let post_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled]
    let post_perp_order_base_lot_position = total_perp_base_lot_position(&post_pa)?;
    check_perp_order_fully_filled(
        best_order.quantity,
        initial_base_lot_position,
        post_perp_order_base_lot_position,
    )?;

    // - 2 [BURN REDEEMABLE] -------------------------------------------------
    check!(
        pre_pa.taker_quote > post_pa.taker_quote,
        UxdErrorCode::InvalidOrderDirection
    )?;
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
    let redeemable_delta = order_delta
        .quote
        .checked_add(order_delta.fee)
        .ok_or(math_err!())?;
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_delta,
    )?;

    // - 3 [WITHDRAW COLLATERAL FROM MANGO THEN RETURN TO USER] ---------------

    // - [Mango withdraw CPI]
    mango_program::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_mango_context()
            .with_signer(depository_signer_seed),
        order_delta.collateral,
        false,
    )?;

    // - [Else return collateral back to user ATA]
    token::transfer(
        ctx.accounts
            .into_transfer_collateral_to_user_context()
            .with_signer(depository_signer_seed),
        order_delta.collateral,
    )?;

    // - [If ATA mint is WSOL, unwrap]
    if ctx.accounts.depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 4 [UPDATE ACCOUNTING] ------------------------------------------------

    ctx.accounts.update_onchain_accounting(
        order_delta.collateral,
        redeemable_delta,
        order_delta.fee,
    )?;

    // Disable until more computing available in Solana 1.9.0
    //
    // emit!(RedeemFromMangoDepositoryEvent {
    //     version: ctx.accounts.controller.version,
    //     controller: ctx.accounts.controller.key(),
    //     depository: ctx.accounts.depository.key(),
    //     user: ctx.accounts.user.key(),
    //     redeemable_amount,
    //     slippage,
    //     collateral_delta: order_delta.collateral,
    //     redeemable_delta,
    //     fee_delta: order_delta.fee,
    // });

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

    pub fn into_unwrap_wsol_by_closing_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.user_collateral.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Additional convenience methods related to the inputted accounts
impl<'info> RedeemFromMangoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> UxdResult<PerpInfo> {
        PerpInfo::new(
            &self.mango_group,
            &self.mango_cache,
            self.mango_perp_market.key,
            self.mango_program.key,
        )
    }

    // Return the uncommitted PerpAccount that represent the account balances
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
        taker_side: mango::matching::Side,
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

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_delta: u64,
        redeemable_delta: u64,
        fee_delta: u64,
    ) -> UxdResult {
        // Mango Depository
        let event = AccountingEvent::Withdraw;
        self.depository
            .update_collateral_amount_deposited(&event, collateral_delta)?;
        // Circulating supply delta
        self.depository
            .update_redeemable_amount_under_management(&event, redeemable_delta)?;
        // Amount of fees taken by the system so far to calculate efficiency
        self.depository
            .update_total_amount_paid_taker_fee(fee_delta)?;
        // Controller
        self.controller
            .update_redeemable_circulating_supply(&event, redeemable_delta)?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> RedeemFromMangoDepository<'info> {
    pub fn validate(&self, redeemable_amount: u64, slippage: u32) -> ProgramResult {
        // Valid slippage check
        check!(slippage <= SLIPPAGE_BASIS, UxdErrorCode::InvalidSlippage)?;

        check!(redeemable_amount > 0, UxdErrorCode::InvalidRedeemableAmount)?;
        check!(
            self.user_redeemable.amount >= redeemable_amount,
            UxdErrorCode::InsufficientRedeemableAmount
        )?;
        Ok(())
    }
}
