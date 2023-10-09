use crate::error::UxdError;
use crate::events::RegisterAlloyxVaultDepositoryEvent;
use crate::state::alloyx_vault_depository::ALLOYX_VAULT_DEPOSITORY_SPACE;
use crate::state::AlloyxVaultDepository;
use crate::utils::validate_collateral_mint_usdc;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::ALLOYX_VAULT_DEPOSITORY_ACCOUNT_VERSION;
use crate::ALLOYX_VAULT_DEPOSITORY_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RegisterAlloyxVaultDepository<'info> {
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
        seeds = [
            ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
            alloyx_vault.key().as_ref(),
            collateral_mint.key().as_ref()
        ],
        bump,
        payer = payer,
        space = ALLOYX_VAULT_DEPOSITORY_SPACE,
    )]
    pub depository: AccountLoader<'info, AlloyxVaultDepository>,

    /// #5
    #[account(
        constraint = collateral_mint.key() == alloyx_vault.usdc_mint @UxdError::CollateralMintMismatch,
        constraint = collateral_mint.key() != alloyx_vault.alloyx_mint @UxdError::CollateralMintConflict
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account(
        init,
        associated_token::mint = collateral_mint,
        associated_token::authority = depository,
        payer = payer,
    )]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        init,
        associated_token::mint = alloyx_vault_mint,
        associated_token::authority = depository,
        payer = payer,
    )]
    pub depository_shares: Box<Account<'info, TokenAccount>>,

    /// #8
    pub alloyx_vault: Box<Account<'info, alloyx_cpi::VaultInfo>>,

    /// #9
    #[account(
        token::mint = collateral_mint,
    )]
    pub alloyx_vault_collateral: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(
        token::mint = alloyx_vault_mint,
    )]
    pub alloyx_vault_shares: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(
        constraint = alloyx_vault_mint.key() == alloyx_vault.alloyx_mint @UxdError::CustomMintMismatch,
        constraint = alloyx_vault_mint.key() != alloyx_vault.usdc_mint @UxdError::CustomMintConflict
    )]
    pub alloyx_vault_mint: Box<Account<'info, Mint>>,

    /// #12
    pub system_program: Program<'info, System>,
    /// #13
    pub token_program: Program<'info, Token>,
    /// #14
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #15
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(
    ctx: Context<RegisterAlloyxVaultDepository>,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u64,
) -> Result<()> {
    // Read some of the depositories required informations
    let depository_bump = *ctx.bumps.get("depository").ok_or(UxdError::BumpError)?;

    // Initialize the depository account
    let depository = &mut ctx.accounts.depository.load_init()?;

    // Initialize depository state
    depository.bump = depository_bump;
    depository.version = ALLOYX_VAULT_DEPOSITORY_ACCOUNT_VERSION;

    depository.controller = ctx.accounts.controller.key();
    depository.collateral_mint = ctx.accounts.collateral_mint.key();

    depository.depository_collateral = ctx.accounts.depository_collateral.key();
    depository.depository_shares = ctx.accounts.depository_shares.key();

    // We register all necessary credix accounts to facilitate other instructions safety checks
    depository.alloyx_vault = ctx.accounts.alloyx_vault.key();
    depository.alloyx_vault_collateral = ctx.accounts.alloyx_vault_collateral.key();
    depository.alloyx_vault_shares = ctx.accounts.alloyx_vault_shares.key();
    depository.alloyx_vault_mint = ctx.accounts.alloyx_vault_mint.key();

    // Depository configuration
    depository.redeemable_amount_under_management_cap = redeemable_amount_under_management_cap;
    depository.minting_fee_in_bps = minting_fee_in_bps;
    depository.redeeming_fee_in_bps = redeeming_fee_in_bps;
    depository.minting_disabled = false;

    // Depository accounting
    depository.collateral_amount_deposited = 0;
    depository.redeemable_amount_under_management = 0;
    depository.minting_fee_total_accrued = 0;
    depository.redeeming_fee_total_accrued = 0;

    // Profits collection
    depository.profits_total_collected = 0;

    // Emit event
    emit!(RegisterAlloyxVaultDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        alloyx_vault: ctx.accounts.alloyx_vault.key(),
    });

    // Done
    Ok(())
}

// Validate
impl<'info> RegisterAlloyxVaultDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_collateral_mint_usdc(&self.collateral_mint, &self.controller)?;
        Ok(())
    }
}
