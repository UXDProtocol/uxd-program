use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::events::MigrateMangoDepositoryToV2Event;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_ACCOUNT_VERSION;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::QUOTE_PASSTHROUGH_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

declare_check_assert_macros!(SourceFileId::InstructionMangoDexMigrateMangoDepositoryToV2);

/// Takes 9 accounts - 6 used locally - 0 for CPI - 2 Programs - 1 Sysvar
#[derive(Accounts)]
pub struct MigrateMangoDepositoryToV2<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 The quote mint used by the `depository` instance
    pub quote_mint: Box<Account<'info, Mint>>,

    /// #6 The `depository`'s TA for its `quote_mint`
    /// MangoAccounts can only transact with the TAs owned by their authority
    /// and this only serves as a passthrough
    #[account(
        init_if_needed,
        seeds = [QUOTE_PASSTHROUGH_NAMESPACE, depository.key().as_ref()],
        bump,
        token::mint = quote_mint,
        token::authority = depository,
        payer = payer,
    )]
    pub depository_quote_passthrough_account: Account<'info, TokenAccount>,

    /// #7 System Program
    pub system_program: Program<'info, System>,

    /// #8 Token Program
    pub token_program: Program<'info, Token>,

    /// #9 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<MigrateMangoDepositoryToV2>) -> UxdResult {
    let quote_mint = ctx.accounts.quote_mint.key();

    // - Update Depository State
    let from_version = ctx.accounts.depository.version;
    ctx.accounts.depository.version = MANGO_DEPOSITORY_ACCOUNT_VERSION;
    // - Add V2 new properties
    ctx.accounts.depository.quote_mint = quote_mint;
    ctx.accounts.depository.quote_mint_decimals = ctx.accounts.quote_mint.decimals;
    ctx.accounts.depository.quote_passthrough =
        ctx.accounts.depository_quote_passthrough_account.key();
    ctx.accounts.depository.quote_passthrough_bump = *ctx
        .bumps
        .get("depository_quote_passthrough_account")
        .ok_or(bump_err!())?;

    emit!(MigrateMangoDepositoryToV2Event {
        version: ctx.accounts.controller.version,
        depository_from_version: from_version,
        depository_to_version: ctx.accounts.depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.depository.collateral_mint,
        insurance_mint: ctx.accounts.depository.insurance_mint,
        quote_mint: ctx.accounts.quote_mint.key(),
        mango_account: ctx.accounts.depository.mango_account,
    });
    Ok(())
}
