use crate::check_assert;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdIdlErrorCode;
use crate::events::MintWithMangoDepositoryEvent;
use crate::ZoDepository;
use zo_abi::{self as zo, program::ZoAbi as Zo};

// use crate::mango_program;
// use crate::mango_utils::check_effective_order_price_versus_limit_price;
// use crate::mango_utils::check_perp_order_fully_filled;
// use crate::mango_utils::derive_order_delta;
// use crate::mango_utils::get_best_order_for_base_lot_quantity;
// use crate::mango_utils::total_perp_base_lot_position;
// use crate::mango_utils::Order;
// use crate::mango_utils::PerpInfo;
use crate::AccountingEvent;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdError;
use crate::UxdErrorCode;
use crate::UxdResult;
use crate::SLIPPAGE_BASIS;
use crate::COLLATERAL_PASSTHROUGH_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::REDEEMABLE_MINT_NAMESPACE;
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
pub struct MintWithZODepository<'info> {
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
        payer = user,
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
        seeds = [ZO_ACCOUNT_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.zo_account_bump,
        constraint = depository.zo_account == depository_zo_account.key() @UxdIdlErrorCode::InvalidZOAccount,
    )]
    pub depository_mango_account: AccountInfo<'info>,
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


    // pass the specific perp market in, and then can get the perp_market_info easily from that

    // 1. loop thru state.perp_markets until find the right one (based on dex_program?)
    // 2. get an index of the market and then just do state.perp_markets[index]
    // 3. pass in the direct perp_market as an account // need client side to comply
}

pub fn handler(
    ctx: Context<MintWithZODepository>,
    collateral_amount: u64, // native units
    slippage: u32,
) -> UxdResult {
    let depository_signer_seed: &[&[&[u8]]] = &[&[
        ZO_DEPOSITORY_NAMESPACE,
        ctx.accounts.depository.collateral_mint.as_ref(),
        &[ctx.accounts.depository.bump],
    ]];
    let controller_signer_seed: &[&[&[u8]]] = &[&[
        CONTROLLER_NAMESPACE,
        &[ctx.accounts.controller.bump]
    ]];
    // NEED TO CHECK HERE TO MAKE SURE ORDER THAT IS GOING TO BE TAKEN IS WITHIN SLIPPAGE

    // - 1 [TRASNFER COLLATERAL TO ZO (LONG)] ---------------------------------

    // - [Get per information]
    let perp_info = ctx.accounts.perpetual_info()?;

    // NEED TO GET PLANNED COLLATEARL DELTA
    // ^^ only need to get if they don't fill the full order

    token::trasnfer(
        ctx.accounts
        .into_trasnfer_user_collateral_to_passthrough_context(),
        collateral_amount, // so need to fix this (?)
    )?;

    // - [Deposit to ZO CPI]
    zo::cpi::deposit(
        ctx.accounts
            .into_deposit_to_zo_context()
            .with_signer(depository_signer_seed),
        collateral_amount,
    )?;
    // zo_program::deposit(
    //     ctx.accounts
    //         .into_deposit_to_zo_context()
    //         .with_signer(depository_signer_seed),
    //     planned_collateral_delta,
    // )?;

    // - 3 [OPEN SHORT PERP] --------------------------------------------------

    // NEED TO GET INITIAL BASE POSITION
    let slippage_amount = I80F48::from_num(collateral_amount)
        .checked_mul(
            I80F48::from_num(slippage)
        )
        .ok_or(math_err!())?;

    let limit_price = I80F48::from_num(collateral_amount)
        .checked_add(slippage_amount)
        .ok_or(math_err!())?;

    // - [Place perp order CPI to ZO]
    zo::cpi::place_perp_order(
        ctx.accounts
            .into_oepn_zo_short_perp_context()
            .with_signer(depository_signer_seed),
        false,
        limit_price, // price with slippage
        None, // what to put for max_base_quantity // amount to mint
        None, // what to put for max_quote_quantity // price per slippage // don't need
        OrderType::ImmediateOrCancel,
        None, // what to put for limit,
        None, // what to put for client_id
    )?;

    // zo_program::place_perp_order(
    //     ctx.accounts
    //         .into_oepn_zo_short_perp_context()
    //         .with_signer(depository_signer_seed),
    //     false,
    //     None, // what to put for limit_price // price with slippage
    //     None, // what to put for max_base_quantity // amount to mint
    //     None, // what to put for max_quote_quantity // price per slippage
    //     OrderType::ImmediateOrCancel,
    //     None, // what to put for limit,
    //     None, // what to put for client_id
    // )?;

    // CHECK SOMEHOW THAT THE ORDER WAS FULLY FILLED (?)

    // - 3 [ENSURE MINTING DOESN'T OVERFLOW THE ZO DEPOSITORIES REDEEMABLE SOFT CAP]
    
    // check some logic with the derive_roder_delta

    // do some delta stuff lol

    // - 4 [MINT THE HEDGED AMOUNT OF REDEEMABLE (minus fee)] -----------------
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_signer_seed),
        redeemable_delta,
    )?;

    // - [IF ATA mint is WSOL, unwrap]
    if ctx.accounts.depository.collateral_mint == spl_token::native_mint::id() {
        token::close_account(ctx.accounts.into_unwrap_wsol_by_closing_ata_context())?;
    }

    // - 5 [UPDATE ACCOUNTING] ------------------------------------------------

    // NEED TO UPDATE ONCHAIN ACCOUNTING

    // - 6 [ENSURE MINTING DOESN'T OVERFLOW THE GLOBAL REDEEMABLE SUPPLY CAP] -
    ctx.accounts.check_redeemable_global_supply_cap_overflow()?;

    // NEED TO EMIT EVENTE

    Ok(())
}

impl<'info> MintWithZODepository<'info> {
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

    pub fn into_deposit_to_zo_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo_program::Deposit<'info>> {
        let cpi_accounts = zo_program::Deposit {
            state: self.state.to_account_info(),
            state_signer: self.state_signer.to_account_info(),
            cache: self.cache.to_account_info(),
            authority: self.depository.to_account_info(),
            margin: self.depository_zo_account.to_account_info(),
            token_account: self
                .depository_collateral_passthrough_account
                .to_account_info(),
            vault: self.vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.zo_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_oepn_zo_short_perp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, zo_program::PlacePerpOrder<'info>> {
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

    pub fn into_mint_redeemable_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_unwrap_wsol_by_closing_ata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::CloseAccount<'info>> {
        let cpi_context = token::CloseAccount {
            account: self.user_collateral.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Add additional convenience methods related to the inputted accounts
impl<'info> MintWithZODepository<'info> {
    fn perpetual_info(
        &self
    ) -> UxdResult<zo::types::PerpMarketInfo> {
        let perp_market_list = &ctx.accounts.state.perp_market;
        for market in &perp_market_list { // is this how i reference it? same here ^^
            if market.dex_market == ctx.accounts.dex_market.key { // what do I use to identify it here // dex_market?
                Ok(market) // there isn't a way to index right? im guessing won't be too many markets
            }
        }
        Err(())
    }

    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info_wrong(
        &self
    ) -> UxdResult<zo::types::PerpMarketInfo> { // is this right?
        let perp_info = zo::types::PerpMarketInfo { // how to get?
            symbol: None,
            oracle_symbol: None,
            perp_type: None,
            asset_decimals: None,
            asset_lot_size: None,
            quote_lot_size: None,
            strike: None,
            base_imf: None,
            liq_fee: None,
            dex_market: self.dex_market.key,
        };
        Ok(perp_info)
    }
}

// Validate
impl<'info> MintWithZODepository<'info> {
    pub fn validate(
        &self,
        collateral_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
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
