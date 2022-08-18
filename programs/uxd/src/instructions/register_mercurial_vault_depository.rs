use crate::error::UxdError;
use crate::Controller;
use crate::MercurialVaultDepository;
use crate::WrappedMercurialVault;
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

    // TODO find a way to use mercurial_vault::state::Vault here
    // The mercurial vault the program will interact with (deposit/withdraw funds)
    // pub mercurial_vault: Account<'info, mercurial_vault::state::Vault>,
    // pub mercurial_vault: UncheckedAccount<'info>,
    pub mercurial_vault: AccountLoader<'info, WrappedMercurialVault>,

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
    pub depository_v_token_vault: Box<Account<'info, TokenAccount>>,

    // programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    // sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RegisterMercurialVaultDepository>) -> Result<()> {
    let collateral_mint = ctx.accounts.collateral_mint.key();

    let depository_bump = *ctx
        .bumps
        .get("depository")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    let depository_v_token_vault_bump = *ctx
        .bumps
        .get("depository_v_token_vault")
        .ok_or_else(|| error!(UxdError::BumpError))?;

    let mercurial_vault = ctx.accounts.mercurial_vault.load()?;
    let depository = &mut ctx.accounts.depository.load_init()?;

    // - Check the information about mercurial vault
    require!(
        ctx.accounts
            .mercurial_vault_lp_mint
            .key()
            .eq(&mercurial_vault.lp_mint.key()),
        UxdError::InvalidMercurialVaultLpMint
    );

    require!(
        ctx.accounts
            .collateral_mint
            .key()
            .eq(&mercurial_vault.token_mint.key()),
        UxdError::InvalidMercurialVaultCollateralMint
    );

    // - Initialize Depository state
    depository.bump = depository_bump;

    depository.version = MERCURIAL_VAULT_DEPOSITORY_ACCOUNT_VERSION;
    depository.collateral_mint = collateral_mint;
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;
    depository.controller = ctx.accounts.controller.key();
    depository.collateral_amount_deposited = u128::MIN;

    depository.v_tokens_vault = ctx.accounts.depository_v_token_vault.key();
    depository.v_tokens_vault_bump = depository_v_token_vault_bump;

    depository.v_token_mint = ctx.accounts.mercurial_vault_lp_mint.key();
    depository.v_token_decimals = ctx.accounts.mercurial_vault_lp_mint.decimals;

    // - Update Controller state
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
