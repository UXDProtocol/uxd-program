use crate::error::UxdError;
use crate::Controller;
use crate::MercurialVaultDepository;
// use crate::WrappedMercurialVault;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_ACCOUNT_VERSION;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_SPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RegisterMercurialVaultDepository<'info> {
    pub authority: Signer<'info>,

    // In order to use with governance program, as the PDA cannot be the payer in nested TX.
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    #[account(
        init,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump,
        payer = payer,
        space = MERCURIAL_VAULT_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, MercurialVaultDepository>,

    pub mercurial_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    pub mercurial_vault_lp_mint: Box<Account<'info, Mint>>,

    pub collateral_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref(), mercurial_vault_lp_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_vault_lp_mint,
        bump,
        payer = payer,
    )]
    pub depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RegisterMercurialVaultDepository>) -> Result<()> {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    let depository_lp_token_vault_bump = *ctx
        .bumps
        .get("depository_lp_token_vault")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    let depository = &mut ctx.accounts.depository.load_init()?;

    // 1 - Initialize Depository state
    depository.bump = depository_bump;

    depository.version = MERCURIAL_VAULT_DEPOSITORY_ACCOUNT_VERSION;
    depository.collateral_mint = collateral_mint;
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    depository.controller = ctx.accounts.controller.key();
    depository.collateral_amount_deposited = u128::MIN;

    depository.lp_tokens_vault = ctx.accounts.depository_lp_token_vault.key();
    depository.lp_tokens_vault_bump = depository_lp_token_vault_bump;

    depository.lp_token_mint = ctx.accounts.mercurial_vault_lp_mint.key();
    depository.lp_token_decimals = ctx.accounts.mercurial_vault_lp_mint.decimals;

    // 2 - Update Controller state
    ctx.accounts
        .controller
        .load_mut()?
        .add_registered_mercurial_vault_depository_entry(ctx.accounts.depository.key())?;

    // TODO
    // Should we check the mint to be about only specific onces like USDC, SOL etc. ?

    // TODO
    // Emit an event
    Ok(())
}
