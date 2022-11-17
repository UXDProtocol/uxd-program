use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::DEFAULT_REDEEMABLE_UNDER_MANAGEMENT_CAP;
use crate::IDENTITY_DEPOSITORY_ACCOUNT_VERSION;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_VAULT_NAMESPACE;
use crate::error::UxdError;
use crate::events::InitializeIdentityDepositoryEvent;
use crate::state::identity_depository::IDENTITY_DEPOSITORY_SPACE;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use crate::state::identity_depository::IdentityDepository;

#[derive(Accounts)]
pub struct InitializeIdentityDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    #[account(
        init, 
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE], // Only a single instance per controller instance
        bump,
        payer = payer,
        space = IDENTITY_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, IdentityDepository>,

    /// #5
    /// Token account holding the collateral from minting
    #[account(
        init,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_VAULT_NAMESPACE],
        token::authority = depository,
        token::mint = collateral_mint,
        bump,
        payer = payer,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    /// #6 The collateral mint used by the `depository` instance
    pub collateral_mint: Account<'info, Mint>,

    /// #7 System Program
    pub system_program: Program<'info, System>,

    /// #8 Token Program
    pub token_program: Program<'info, Token>,

    /// #9 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<InitializeIdentityDepository>) -> Result<()> {
    let depository_bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    let collateral_vault_bump = *ctx
        .bumps
        .get("collateral_vault")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    // - Initialize Depository state
    let depository = &mut ctx.accounts.depository.load_init()?;
    depository.bump = depository_bump;
    depository.version = IDENTITY_DEPOSITORY_ACCOUNT_VERSION;
    depository.collateral_mint = ctx.accounts.collateral_mint.key();
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    depository.collateral_amount_deposited = u128::MIN;
    depository.collateral_vault = ctx.accounts.collateral_vault.key();
    depository.collateral_vault_bump = collateral_vault_bump;
    depository.redeemable_amount_under_management = u128::MIN;
    depository.redeemable_amount_under_management_cap = DEFAULT_REDEEMABLE_UNDER_MANAGEMENT_CAP;
    depository.minting_disabled = true;

    emit!(InitializeIdentityDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
    });

    Ok(())
}
