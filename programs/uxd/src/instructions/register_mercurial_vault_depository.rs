use crate::error::UxdError;
use crate::events::RegisterMercurialVaultDepositoryEvent;
use crate::state::MercurialVaultDepository;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_ACCOUNT_VERSION;
use crate::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_SPACE;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RegisterMercurialVaultDepository<'info> {
    /// #1
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        init,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, mercurial_vault.key().as_ref(), collateral_mint.key().as_ref()],
        bump,
        payer = payer,
        space = MERCURIAL_VAULT_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #5
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account(
        constraint = mercurial_vault.lp_mint == mercurial_vault_lp_mint.key() @UxdError::InvalidMercurialVaultLpMint,
    )]
    pub mercurial_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #7
    pub mercurial_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #8
    /// Token account holding the LP tokens minted by depositing collateral on mercurial vault
    #[account(
        init,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE, mercurial_vault.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = depository,
        token::mint = mercurial_vault_lp_mint,
        bump,
        payer = payer,
    )]
    pub depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #9
    pub system_program: Program<'info, System>,

    /// #10
    pub token_program: Program<'info, Token>,

    /// #11
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RegisterMercurialVaultDepository>) -> Result<()> {
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

    depository.collateral_mint = ctx.accounts.collateral_mint.key();
    depository.collateral_mint_decimals = ctx.accounts.collateral_mint.decimals;

    depository.controller = ctx.accounts.controller.key();

    depository.collateral_amount_deposited = u128::MIN;
    depository.minted_redeemable_amount = u128::MIN;

    depository.lp_token_vault = ctx.accounts.depository_lp_token_vault.key();
    depository.lp_token_vault_bump = depository_lp_token_vault_bump;

    depository.mercurial_vault_lp_mint = ctx.accounts.mercurial_vault_lp_mint.key();
    depository.mercurial_vault_lp_mint_decimals = ctx.accounts.mercurial_vault_lp_mint.decimals;

    depository.mercurial_vault = ctx.accounts.mercurial_vault.key();

    // 2 - Update Controller state
    ctx.accounts
        .controller
        .load_mut()?
        .add_registered_mercurial_vault_depository_entry(ctx.accounts.depository.key())?;

    // 3 - Emit event
    emit!(RegisterMercurialVaultDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
    });

    Ok(())
}

// Validate
impl<'info> RegisterMercurialVaultDepository<'info> {
    // Only few stablecoin collateral mint are whitelisted
    // This check exists to avoid the creation of an imbalanced mercurial vault depository
    // Redeemable and collateral should always be 1:1
    pub fn validate_collateral_mint(&self) -> Result<()> {
        let usdc_mint: Pubkey = Pubkey::new(b"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

        require!(
            self.collateral_mint.key().eq(&usdc_mint),
            UxdError::CollateralMintNotAllowed,
        );

        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        require!(
            self.mercurial_vault
                .token_mint
                .eq(&self.collateral_mint.key()),
            UxdError::MercurialVaultDoNotMatchCollateral,
        );

        // Collateral mint should be different than redeemable mint
        require!(
            self.collateral_mint
                .key()
                .ne(&self.controller.load()?.redeemable_mint),
            UxdError::CollateralEqualToRedeemable,
        );

        #[cfg(feature = "production")]
        self.validate_collateral_mint()?;

        Ok(())
    }
}
