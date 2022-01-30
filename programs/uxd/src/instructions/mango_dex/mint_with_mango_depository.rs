use crate::check_assert;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdIdlErrorCode;
// use crate::events::MintWithMangoDepositoryEvent;
use crate::mango_program;
use crate::mango_utils::check_effective_order_price_versus_limit_price;
use crate::mango_utils::check_perp_order_fully_filled;
use crate::mango_utils::derive_order_delta;
use crate::mango_utils::get_best_order_for_base_lot_quantity;
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
use mango::matching::Book;
use mango::matching::Side;
use mango::state::MangoAccount;
use mango::state::PerpAccount;
use mango::state::PerpMarket;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexMintWithMangoDepository);

#[derive(Accounts)]
pub struct MintWithMangoDepository<'info> {
    pub user: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump
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
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @UxdIdlErrorCode::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = depository.collateral_mint, // @UxdIdlErrorCode::InvalidUserCollateralATAMint
        associated_token::authority = user,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        associated_token::mint = redeemable_mint, // @UxdIdlErrorCode::InvalidUserRedeemableATAMint
        associated_token::authority = user,
        payer = payer,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [COLLATERAL_PASSTHROUGH_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.collateral_passthrough_bump,
        constraint = depository.collateral_passthrough == depository_collateral_passthrough_account.key() @UxdIdlErrorCode::InvalidCollateralPassthroughAccount,
        constraint = depository_collateral_passthrough_account.mint == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralPassthroughATAMint
    )]
    pub depository_collateral_passthrough_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.mango_account_bump,
        constraint = depository.mango_account == depository_mango_account.key() @UxdIdlErrorCode::InvalidMangoAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,
    // Mango CPI accounts
    pub mango_group: UncheckedAccount<'info>,
    pub mango_cache: UncheckedAccount<'info>,
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
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub mango_program: Program<'info, mango_program::Mango>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintWithMangoDepository>,
    collateral_amount: u64, // native units
    slippage: u32,
) -> UxdResult {
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        MANGO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];
    let controller_signer_seed: &[&[&[u8]]] =
        &[&[CONTROLLER_NAMESPACE, &[ctx.accounts.controller.bump]]];

    // - 1 [FIND BEST ORDER FOR SHORT PERP POSITION] --------------------------

    // - [Get perp information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // - [Get the amount of Base Lots for the perp order]
    let base_lot_amount = I80F48::from_num(collateral_amount)
        .checked_div(perp_info.base_lot_size)
        .ok_or(math_err!())?
        // Round down
        .checked_floor()
        .ok_or(math_err!())?;

    // - [Find the best order]
    let best_order = ctx
        .accounts
        .get_best_order_for_base_lot_quantity_from_order_book(
            Side::Bid,
            base_lot_amount.checked_to_num().ok_or(math_err!())?,
        )?;

    // - [Checks that the best price found is within slippage range]
    check_effective_order_price_versus_limit_price(&perp_info, &best_order, slippage)?;

    // - 2 [TRANSFER COLLATERAL TO MANGO (LONG)] ------------------------------

    // Note : Done after calculating the mango order so that we don't overdraft collateral.
    //        But needs to be deposited before the actual order placement as the
    //        collateral deposited is used as leverage for opening the perp short.

    // This value is verified after by checking if the perp order was fully filled
    let planned_collateral_delta = I80F48::from_num(best_order.quantity)
        .checked_mul(perp_info.base_lot_size)
        .ok_or(math_err!())?
        .checked_to_num()
        .ok_or(math_err!())?;
    // msg!("planned_collateral_delta {}", planned_collateral_delta);

    // - [Transferring user collateral to the passthrough account]
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_passthrough_context(),
        planned_collateral_delta,
    )?;

    // - [Deposit to Mango CPI]
    mango_program::deposit(
        ctx.accounts
            .into_deposit_to_mango_context()
            .with_signer(depository_signer_seed),
        planned_collateral_delta,
    )?;

    // - 3 [OPEN SHORT PERP] --------------------------------------------------

    // - [Perp account state PRE perp order]
    let pre_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Base depository's position size in native units PRE perp opening (to calculate the % filled later on)]
    let initial_base_position = total_perp_base_lot_position(&pre_pa)?;

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
    let post_pa = ctx.accounts.perp_account(&perp_info)?;

    // - [Checks that the order was fully filled]
    let post_perp_order_base_lot_position = total_perp_base_lot_position(&post_pa)?;
    check_perp_order_fully_filled(
        best_order.quantity,
        initial_base_position,
        post_perp_order_base_lot_position,
    )?;

    // - 3 [ENSURE MINTING DOESN'T OVERFLOW THE MANGO DEPOSITORIES REDEEMABLE SOFT CAP]

    // ensure current context make sense as the derive_order_delta is generic
    check!(
        pre_pa.taker_quote < post_pa.taker_quote,
        UxdErrorCode::InvalidOrderDirection
    )?;
    let order_delta = derive_order_delta(&pre_pa, &post_pa, &perp_info)?;
    let redeemable_delta = order_delta
        .quote
        .checked_sub(order_delta.fee)
        .ok_or(math_err!())?;
    // msg!("redeemable_delta {}", redeemable_delta);
    ctx.accounts
        .check_mango_depositories_redeemable_soft_cap_overflow(redeemable_delta)?;

    // - 4 [MINTS THE HEDGED AMOUNT OF REDEEMABLE (minus fees)] ---------------
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_signer_seed),
        redeemable_delta,
    )?;

    // - [If ATA mint is WSOL, unwrap]
    if ctx.accounts.depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------
    ctx.accounts.update_onchain_accounting(
        order_delta.collateral,
        redeemable_delta,
        order_delta.fee,
    )?;

    // - 6 [ENSURE MINTING DOESN'T OVERFLOW THE GLOBAL REDEEMABLE SUPPLY CAP] -
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    // Commented as there is computing budget issues for now (this costs around 4k)
    // emit!(MintWithMangoDepositoryEvent {
    //     version: ctx.accounts.controller.version,
    //     controller: ctx.accounts.controller.key(),
    //     depository: ctx.accounts.depository.key(),
    //     user: ctx.accounts.user.key(),
    //     collateral_amount,
    //     slippage,
    //     collateral_delta: order_delta.collateral,
    //     redeemable_delta,
    //     fee_delta: order_delta.fee,
    // });

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

    fn get_best_order_for_base_lot_quantity_from_order_book(
        &self,
        side: mango::matching::Side,
        base_lot_amount: i64,
    ) -> UxdResult<Order> {
        let perp_market = PerpMarket::load_checked(
            &self.mango_perp_market,
            self.mango_program.key,
            self.mango_group.key,
        )?;
        let bids_ai = self.mango_bids.to_account_info();
        let asks_ai = self.mango_asks.to_account_info();
        let book = Book::load_checked(self.mango_program.key, &bids_ai, &asks_ai, &perp_market)?;
        let best_order = get_best_order_for_base_lot_quantity(&book, side, base_lot_amount)?;

        Ok(best_order.ok_or(throw_err!(UxdErrorCode::InsufficientOrderBookDepth))?)
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

// Validate
impl<'info> MintWithMangoDepository<'info> {
    pub fn validate(&self, collateral_amount: u64, slippage: u32) -> ProgramResult {
        // Valid slippage check
        check!(slippage <= SLIPPAGE_BASIS, UxdErrorCode::InvalidSlippage)?;

        check!(collateral_amount > 0, UxdErrorCode::InvalidCollateralAmount)?;
        check!(
            self.user_collateral.amount >= collateral_amount,
            UxdErrorCode::InsufficientCollateralAmount
        )?;
        Ok(())
    }
}
