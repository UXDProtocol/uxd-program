use crate::SAFETY_VAULT_ACCOUNT_VERSION;
use crate::error::UxdError;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::SAFETY_VAULT_NAMESPACE;
use crate::COLLATERAL_VAULT_NAMESPACE;
use crate::QUOTE_VAULT_NAMESPACE;
use crate::SAFETY_VAULT_SPACE;
use crate::state::MangoDepository;
use crate::state::SafetyVault;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

/// Takes x accounts
#[derive(Accounts)]
pub struct InitializeSafetyVault<'info> {
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
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 The SafetyVault responsible to holding quote from liquidation
    #[account(
        init,
        seeds = [SAFETY_VAULT_NAMESPACE, depository.key().as_ref()],
        bump,
        payer = payer,
        space = SAFETY_VAULT_SPACE,
    )]
    pub safety_vault: AccountLoader<'info, SafetyVault>,

    /// #6 The token account of the SafetyVault to hold quote
    #[account(
        init,
        seeds = [QUOTE_VAULT_NAMESPACE, safety_vault.key().as_ref()],
        bump,
        token::mint = quote_mint,
        token::authority = safety_vault,
        payer = payer,
    )]
    pub quote_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [COLLATERAL_VAULT_NAMESPACE, safety_vault.key().as_ref()],
        bump,
        token::mint = collateral_mint,
        token::authority = safety_vault,
        payer = payer,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    /// #7 The mint of the quote which collateral will be liquidated into
    #[account(
        constraint = quote_mint.key() == depository.load()?.quote_mint,
    )]
    pub quote_mint: Account<'info, Mint>,

    /// #8 The mint of the collateral of the depository
    #[account(
        constraint = collateral_mint.key() == depository.load()?.collateral_mint,
    )]
    pub collateral_mint: Account<'info, Mint>,

    /// #8 System Program
    pub system_program: Program<'info, System>,

    /// #9 Token Program
    pub token_program: Program<'info, Token>,

    /// #10 Rent Sysvar
    pub rent: Sysvar<'info, Rent>, 
}

pub fn handler(ctx: Context<InitializeSafetyVault>,) -> Result<()> {
    let safety_vault = &mut ctx.accounts.safety_vault.load_init()?;
    
    // Set values for the SafetyVault
    safety_vault.bump = *ctx
        .bumps
        .get("safety_vault")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    safety_vault.quote_vault_bump = *ctx
        .bumps
        .get("quote_vault")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    safety_vault.collateral_vault_bump = *ctx
        .bumps
        .get("collateral_vault")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    safety_vault.version = SAFETY_VAULT_ACCOUNT_VERSION;
    safety_vault.authority = ctx.accounts.authority.key();
    safety_vault.depository = ctx.accounts.depository.key();
    safety_vault.quote_vault = ctx.accounts.quote_vault.key();
    safety_vault.collateral_vault = ctx.accounts.collateral_vault.key();
    safety_vault.collateral_liquidated = u128::MIN;
    safety_vault.quote_vault_balance = u128::MIN;

    Ok(())
}
