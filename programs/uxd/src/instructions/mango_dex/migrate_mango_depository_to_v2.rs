use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use crate::MANGO_DEPOSITORY_ACCOUNT_VERSION;
use crate::UxdResult;
use crate::MangoDepository;
use crate::Controller;
use crate::error::UxdIdlErrorCode;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::QUOTE_PASSTHROUGH_NAMESPACE;
use crate::events::MigrateMangoDepositoryToV2Event;

#[derive(Accounts)]
#[instruction(
    quote_passthrough_bump: u8,
)]
pub struct MigrateMangoDepositoryToV2<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.key().as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,
    pub quote_mint: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        seeds = [QUOTE_PASSTHROUGH_NAMESPACE, depository.key().as_ref()],
        bump = quote_passthrough_bump,
        token::mint = quote_mint,
        token::authority = depository,
        payer = payer,
    )]
    pub depository_quote_passthrough_account: Account<'info, TokenAccount>,
    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MigrateMangoDepositoryToV2>,
    quote_passthrough_bump: u8,
) -> UxdResult {
    let quote_mint = ctx.accounts.quote_mint.key();


    // - Update Depository State
    let from_version = ctx.accounts.depository.version;
    ctx.accounts.depository.version = MANGO_DEPOSITORY_ACCOUNT_VERSION;
    // - Add V2 new properties
    ctx.accounts.depository.quote_mint = quote_mint;
    ctx.accounts.depository.quote_mint_decimals = ctx.accounts.quote_mint.decimals;
    ctx.accounts.depository.quote_passthrough = ctx.accounts.depository_quote_passthrough_account.key();
    ctx.accounts.depository.quote_passthrough_bump = quote_passthrough_bump;

    emit!(MigrateMangoDepositoryToV2Event {
        version: ctx.accounts.controller.version,
        depository_from_version: from_version,
        depository_to_version: ctx.accounts.depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.depository.collateral_mint.key(),
        insurance_mint: ctx.accounts.depository.insurance_mint.key(),
        quote_mint: ctx.accounts.quote_mint.key(),
        mango_account: ctx.accounts.depository.mango_account.key(),
    });

    Ok(())
}