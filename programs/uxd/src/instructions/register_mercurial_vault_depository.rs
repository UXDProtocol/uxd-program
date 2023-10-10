use std::str::FromStr;

use crate::error::UxdError;
use crate::events::RegisterMercurialVaultDepositoryEvent;
use crate::state::MercurialVaultDepository;
use crate::validate_is_program_frozen;
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

pub fn handler(
    ctx: Context<RegisterMercurialVaultDepository>,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<()> {
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
    depository.redeemable_amount_under_management = u128::MIN;

    depository.minting_fee_in_bps = minting_fee_in_bps;
    depository.redeeming_fee_in_bps = redeeming_fee_in_bps;

    depository.minting_fee_total_accrued = u128::MIN;
    depository.redeeming_fee_total_accrued = u128::MIN;

    depository.lp_token_vault = ctx.accounts.depository_lp_token_vault.key();
    depository.lp_token_vault_bump = depository_lp_token_vault_bump;

    depository.mercurial_vault_lp_mint = ctx.accounts.mercurial_vault_lp_mint.key();
    depository.mercurial_vault_lp_mint_decimals = ctx.accounts.mercurial_vault_lp_mint.decimals;

    depository.mercurial_vault = ctx.accounts.mercurial_vault.key();

    depository.redeemable_amount_under_management_cap = redeemable_amount_under_management_cap;

    depository.profits_total_collected = u128::MIN;
    depository.last_profits_collection_unix_timestamp = 0;

    // enable minting by default
    depository.minting_disabled = false;

    // 3 - Emit event
    emit!(RegisterMercurialVaultDepositoryEvent {
        version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        mercurial_vault: ctx.accounts.mercurial_vault.key(),
        depository_lp_token_vault: ctx.accounts.depository_lp_token_vault.key(),
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
        let usdc_mint: Pubkey =
            Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();

        require!(
            self.collateral_mint.key().eq(&usdc_mint),
            UxdError::CollateralMintNotAllowed,
        );

        Ok(())
    }

    pub fn validate(
        &self,
        _minting_fee_in_bps: u8,
        _redeeming_fee_in_bps: u8,
        _redeemable_amount_under_management_cap: u128,
    ) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;

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
            UxdError::CollateralMintEqualToRedeemableMint,
        );

        // Collateral mint and redeemable mint should share the same decimals
        // due to the fact that decimal delta is not handled in the mint/redeem instructions
        require!(
            self.collateral_mint
                .decimals
                .eq(&self.controller.load()?.redeemable_mint_decimals),
            UxdError::CollateralMintNotAllowed,
        );

        #[cfg(feature = "production")]
        self.validate_collateral_mint()?;

        Ok(())
    }
}
