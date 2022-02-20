use crate::check_assert;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdIdlErrorCode;
use crate::events::MintWithMangoDepositoryEvent;
use crate::mango_program;
use crate::mango_utils::check_perp_order_fully_filled;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::limit_price;
use crate::mango_utils::price_to_lot_price;
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
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexMintWithMangoDepository);

/// Takes 24 accounts - 10 used locally - 9 for MangoMarkets CPI - 4 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct MintWithMangoDepository<'info> {
    /// #1 Public call accessible to any user
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 The redeemable mint managed by the `controller` instance
    /// Tokens will be minted during this instruction
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @UxdIdlErrorCode::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #6 The collateral mint and used by the `depository` instance
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #7 The `user`'s ATA for the `depository` `collateral_mint`
    /// Will be debited during this instruction
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = user,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 The `user`'s ATA for the `controller`'s `redeemable_mint`
    /// Will be credited during this instruction
    #[account(
        init_if_needed,
        associated_token::mint = redeemable_mint,
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #9 The `depository`'s TA for its `insurance_mint`
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

    /// #10 The MangoMarkets Account (MangoAccount) managed by the `depository`
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdIdlErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,

    /// #11 [MangoMarkets CPI] Index grouping perp and spot markets
    pub mango_group: AccountInfo<'info>,

    /// #12 [MangoMarkets CPI] Cache
    pub mango_cache: AccountInfo<'info>,

    /// #13 [MangoMarkets CPI] Root Bank for the `depository`'s `collateral_mint`
    pub mango_root_bank: AccountInfo<'info>,

    /// #14 [MangoMarkets CPI] Node Bank for the `depository`'s `collateral_mint`
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,

    /// #15 [MangoMarkets CPI] Vault for the `depository`'s `collateral_mint`
    #[account(mut)]
    pub mango_vault: AccountInfo<'info>,

    /// #16 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,

    /// #17 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook bids
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,

    /// #18 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market orderbook asks
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,

    /// #19 [MangoMarkets CPI] `depository`'s `collateral_mint` perp market event queue
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,

    /// #20 System Program
    pub system_program: Program<'info, System>,

    /// #21 Token Program
    pub token_program: Program<'info, Token>,

    /// #22 Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #23 MangoMarketv3 Program
    pub mango_program: Program<'info, mango_program::Mango>,

    /// #24 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintWithMangoDepository>,
    collateral_amount: u64,
    slippage: u32,
) -> UxdResult {
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];
    let controller_pda_signer: &[&[&[u8]]] =
        &[&[CONTROLLER_NAMESPACE, &[ctx.accounts.controller.bump]]];

    // - 1 [FIND BEST ORDER FOR SHORT PERP POSITION] --------------------------

    // - [Get MangoMarkets  Collateral-Perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Get the amount of Base Lots for the perp order (odd lots won't be processed)]
    let base_lot_amount = I80F48::from_num(collateral_amount)
        .checked_div(perp_info.base_lot_size)
        .ok_or(math_err!())?
        .floor();

    // - [Define perp order]
    // Note : Augment the delta neutral position, increasing short exposure, by selling perp.
    //        [BID: maker | ASK: taker (us, the caller)]
    let taker_side = Side::Ask;
    let limit_price = limit_price(perp_info.price, slippage, taker_side)?;
    let limit_price_lot = price_to_lot_price(limit_price, &perp_info)?;
    let perp_order = Order {
        quantity: base_lot_amount.checked_to_num().ok_or(math_err!())?,
        price: limit_price_lot.checked_to_num().ok_or(math_err!())?, // worth execution price
        taker_side,
    };

    // - 2 [TRANSFER COLLATERAL TO MANGO (LONG)] ------------------------------
    // This value is verified after by checking if the perp order was fully filled
    let planned_collateral_delta = I80F48::from_num(perp_order.quantity)
        .checked_mul(perp_info.base_lot_size)
        .ok_or(math_err!())?
        .checked_to_num()
        .ok_or(math_err!())?;

    // - [Transferring user collateral to the passthrough account]
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_passthrough_context(),
        planned_collateral_delta,
    )?;

    // - [MangoMarkets CPI - Deposit collateral to Depository MangoAccount]
    mango_program::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_pda_signer),
        planned_collateral_delta,
    )?;

    // - 3 [OPEN SHORT PERP] --------------------------------------------------

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Base depository's position size in native units PRE perp opening (to calculate the % filled later on)]
    let initial_base_position = total_perp_base_lot_position(&pre_pa)?;

    // - [MangoMarkets CPI - Place perp order]
    mango_program::place_perp_order(
        ctx.accounts
            .into_open_mango_short_perp_context()
            .with_signer(depository_pda_signer),
        perp_order.price,
        perp_order.quantity,
        0,
        perp_order.taker_side,
        mango::matching::OrderType::ImmediateOrCancel,
        false,
    )?;

    // - [Perp account state POST perp order]
    let post_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled (FoK)]
    let post_perp_order_base_lot_position = total_perp_base_lot_position(&post_pa)?;
    check_perp_order_fully_filled(
        perp_order.quantity,
        initial_base_position,
        post_perp_order_base_lot_position,
    )?;

    // - 3 [CHECK REDEEMABLE SOFT CAP OVERFLOW] -------------------------------

    // ensure current context is valid as the derive_order_delta is generic
    check!(
        pre_pa.taker_quote < post_pa.taker_quote,
        UxdErrorCode::InvalidOrderDirection
    )?;
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
    let redeemable_delta = order_delta
        .quote
        .checked_sub(order_delta.fee)
        .ok_or(math_err!())?;
    ctx.accounts
        .check_mango_depositories_redeemable_soft_cap_overflow(redeemable_delta)?;

    // - 4 [MINTS THE HEDGED AMOUNT OF REDEEMABLE (minus fees)] ----------------

    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_delta,
    )?;

    // - [if ATA mint is WSOL, unwrap]
    if ctx.accounts.depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_onchain_accounting(
        order_delta.collateral,
        redeemable_delta,
        order_delta.fee,
    )?;

    // - 6 [CHECK GLOBAL REDEEMABLE SUPPLY CAP OVERFLOW] ----------------------
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    emit!(MintWithMangoDepositoryEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        collateral_amount,
        slippage,
        collateral_delta: order_delta.collateral,
        redeemable_delta,
        fee_delta: order_delta.fee,
    });

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
impl<'info> MintWithMangoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> UxdResult<PerpInfo> {
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
    fn perp_account(&self, perp_info: &PerpInfo) -> UxdResult<PerpAccount> {
        // - loads Mango's accounts
        let mango_account = MangoAccount::load_checked(
            &self.depository_mango_account,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        Ok(mango_account.perp_accounts[perp_info.market_index])
    }

    // Ensure that the minted amount does not raise the Redeemable supply beyond the Global Redeemable Supply Cap
    fn check_redeemable_global_supply_cap_overflow(&self) -> UxdResult {
        check!(
            self.controller.redeemable_circulating_supply
                <= self.controller.redeemable_global_supply_cap,
            UxdErrorCode::RedeemableGlobalSupplyCapReached
        )?;
        Ok(())
    }

    fn check_mango_depositories_redeemable_soft_cap_overflow(
        &self,
        redeemable_delta: u64,
    ) -> UxdResult {
        check!(
            redeemable_delta <= self.controller.mango_depositories_redeemable_soft_cap,
            UxdErrorCode::MangoDepositoriesSoftCapOverflow
        )?;
        Ok(())
    }

    // Update the accounting in the Depository and Controller Accounts to reflect changes
    fn update_onchain_accounting(
        &mut self,
        collateral_delta: u64,
        redeemable_delta: u64,
        fee_delta: u64,
    ) -> UxdResult {
        // Mango Depository
        let event = AccountingEvent::Deposit;
        self.depository
            .update_collateral_amount_deposited(&event, collateral_delta)?;
        self.depository
            .update_redeemable_amount_under_management(&event, redeemable_delta)?;
        self.depository
            .update_total_amount_paid_taker_fee(fee_delta)?;
        // Controller
        self.controller
            .update_redeemable_circulating_supply(&event, redeemable_delta)?;
        Ok(())
    }
}

// Validate input arguments
impl<'info> MintWithMangoDepository<'info> {
    pub fn validate(&self, collateral_amount: u64, slippage: u32) -> ProgramResult {
        // Valid slippage check
        check!(
            (slippage > 0) && (slippage <= SLIPPAGE_BASIS),
            UxdErrorCode::InvalidSlippage
        )?;

        check!(collateral_amount > 0, UxdErrorCode::InvalidCollateralAmount)?;
        check!(
            self.user_collateral.amount >= collateral_amount,
            UxdErrorCode::InsufficientCollateralAmount
        )?;
        Ok(())
    }
}
