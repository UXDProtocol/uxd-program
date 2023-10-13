use crate::error::UxdError;
use crate::events::RebalanceAlloyxVaultDepositoryEvent;
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
pub struct RebalanceAlloyxVaultDepository<'info> {
    /// #1
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.identity_depository == identity_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.mercurial_vault_depository == mercurial_vault_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.credix_lp_depository == credix_lp_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.alloyx_vault_depository == alloyx_vault_depository.key() @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #4
    #[account(
        mut,
        seeds = [
            ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
            depository.load()?.alloyx_vault_info.key().as_ref(),
            depository.load()?.collateral_mint.as_ref()
        ],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = depository_collateral @UxdError::InvalidDepositoryCollateral,
        has_one = depository_shares @UxdError::InvalidDepositoryShares,
        has_one = alloyx_vault_info @UxdError::InvalidAlloyxVaultInfo,
        has_one = alloyx_vault_collateral @UxdError::InvalidAlloyxVaultCollateral,
        has_one = alloyx_vault_shares @UxdError::InvalidAlloyxVaultShares,
        has_one = alloyx_vault_mint @UxdError::InvalidAlloyxVaultMint,
    )]
    pub depository: AccountLoader<'info, AlloyxVaultDepository>,

    /// #10
    #[account(mut)]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(mut)]
    pub depository_shares: Box<Account<'info, TokenAccount>>,

    /// #12
    pub alloyx_vault_info: Box<Account<'info, alloyx_cpi::VaultInfo>>,

    /// #13
    #[account(mut)]
    pub alloyx_vault_collateral: Box<Account<'info, TokenAccount>>,

    /// #14
    #[account(mut)]
    pub alloyx_vault_shares: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(mut)]
    pub alloyx_vault_mint: Box<Account<'info, Mint>>,

    /// #16
    #[account(
        constraint = alloyx_vault_pass.investor == depository.key() @UxdError::InvalidAlloyxVaultPass,
    )]
    pub alloyx_vault_pass: Account<'info, alloyx_cpi::PassInfo>,

    /// #20
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub profits_beneficiary_collateral: Box<Account<'info, TokenAccount>>,

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
    ctx: Context<RebalanceAlloyxVaultDepository>,
    vault_id: &String,
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
    depository.alloyx_vault_info = ctx.accounts.alloyx_vault_info.key();
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
    emit!(RebalanceAlloyxVaultDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: depository.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        alloyx_vault_info: ctx.accounts.alloyx_vault_info.key(),
    });

    // Done
    Ok(())
}

// Validate
impl<'info> RebalanceAlloyxVaultDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
