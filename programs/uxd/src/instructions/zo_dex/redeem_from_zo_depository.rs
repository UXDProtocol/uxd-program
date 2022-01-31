use crate::check_assert;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::events::RedeemFromMangoDepositoryEvent;
use crate::zo_rogram;
// use crate::mango_program;
// use crate::mango_utils::check_effective_order_price_versus_limit_price;
// use crate::mango_utils::check_perp_order_fully_filled;
// use crate::mango_utils::derive_order_delta;
// use crate::mango_utils::get_best_order_for_quote_lot_amount;
// use crate::mango_utils::total_perp_base_lot_position;
// use crate::mango_utils::Order;
// use crate::mango_utils::PerpInfo;
use crate::AccountingEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdError;
use crate::UxdResult;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SLIPPAGE_BASIS;
use crate::ZO_ACCOUNT_NAMESPACE;
use crate::ZO_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::matching::Book;
use mango::state::MangoAccount;
use mango::state::PerpAccount;
use mango::state::PerpMarket;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexRedeemFromMangoDepository); // change

#[derive(Accounts)]
pub struct RedeemFromZODepository<'info> {
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
        seeds = [ZO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_zo_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, ZODepository>>,
    #[account(
        init_if_needed,
        associated_token::mint = collateral_mint, // @UxdIdlErrorCode::InvalidCollateralATAMint
        associated_token::authority = user,
        payer = user,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = redeemable_mint.key(),
        associated_token::authority = user,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @UxdIdlErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = controller.redeemable_mint_bump,
        constraint = redeemable_mint.key() == controller.redeemable_mint @UxdIdlErrorCode::InvalidRedeemableMint
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,
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
        seeds = [ZO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.zo_account_bump,
        constraint = depository.zo_account == depository_zo_account.key() @UxdIdlErrorCode::InvalidZOAccount,
    )]
    pub depository_zo_account: AccountInfo<'info>,
    // ZO CPI accounts
    pub state: AccountInfo<'info>,
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub control: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub req_q: AccountInfo<'info>,
    #[account(mut)]
    pub event_q: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = dex::ID)]
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub zo_program: Program<'info, zo_program::ZO>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RedeemFromZODepository>,
    redeemable_amount: u64,
    slippage: u32,
) -> UxdResult {
    let depository_signer_seeds: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];

    // - 1 [CLOSE THE EQUIVALENT PERP SHORT ON ZO] -------------------------
    // get perp info?

    // - [Calculates the quantity of short to close]
    let mut exposure_delta_in_quote_unit = I80F48::from_num(redeemable_amount);

    // taker fees on zo?

    // get perp info b4 order?

    // - [Base depo]





    
    // - 2 [BURN REDEEMABLE] -------------------------------------------------

    // Some check

    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_delta,
    )?;







    // - 3 [WITHDRAW COLLATERAL FROM MANGO THEN RETURN TO USER] ---------------

    // - [ZO withdraw CPI]
    zo::cpi::withdraw(
        ctx.accounts
            .into_withdraw_collateral_from_zo_context()
            .with_signer(depository_signer_seed),
        redeemable_amount,
    )?;

    // - [Else return collateral back to user ATA]
    token::transfer(
        ctx.accounts
            .into_transfer_collateral_to_user_context()
            .with_signer(depository_signer_seed),
        redeemable_amount,
    )?;

    // - [If ATA mint is WSOL, unwrap]
    if ctx.accounts.depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 4 [UPDATE ACCOUNTING]







    Ok(())

}

// MARK: - Contexts -----

impl<'info> RedeemFromZODepository<'info> {
    pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_close_zo_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo_program::PlacePerpOrder> {
        let cpi_accounts = zo_program::PlacePerpOrder {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            cache: self.cache.to_account_info(),
            authority: self.depository.to_account_info(),
            margin: self.depository_zo_account.to_account_info(),
            control: self.control.to_account_info(),
            open_orders: self.open_orders.to_account_info(),
            dex_market: self.dex_market.to_account_info(),
            req_q: self.req_q.to_account_info(),
            event_q: self.event_q.to_account_info(),
            market_bids: self.market_bids.to_account_info(),
            market_asks: self.market_asks.to_account_info(),
            dex_program: self.dex_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_collateral_from_zo_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, zo_program::Withdraw<'info>> {
        let cpi_accounts = zo_program::Withdraw {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            cache: self.cache.to_account_info(),
            authority: self.depository.to_account_info(),
            margin: self.depository_zo_account.to_account_info(),
            control: self.control.to_account_info(),
            token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            vault: self.vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
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
impl<'info> RedeemFromZoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(
        &self
    ) -> UxdResult<zo::types::PerpMarketInfo> {
        let perp_markets = &self.state.perp_markets;
        for market in perp_markets {
            if market.dex_market == self.dex_market.key {
                Ok(market)
            }
        }
        Err(())
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

// Validate
impl<'info> RedeemFromZODepository<'info> {
    pub fn validate(
        &self,
        redeemable_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
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
