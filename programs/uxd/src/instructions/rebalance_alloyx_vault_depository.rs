use crate::error::UxdError;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::state::AlloyxVaultDepository;
use crate::state::CredixLpDepository;
use crate::validate_is_program_frozen;
use crate::Controller;
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

    /// #3
    #[account(
        mut,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = identity_depository.load()?.collateral_vault == identity_depository_collateral.key() @UxdError::InvalidDepositoryCollateral,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #4
    #[account(mut)]
    pub identity_depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #5
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub mercurial_vault_depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #6
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub credix_lp_depository: AccountLoader<'info, CredixLpDepository>,

    /// #4
    #[account(
        mut,
        seeds = [
            ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
            alloyx_vault_depository.load()?.alloyx_vault_info.key().as_ref(),
            alloyx_vault_depository.load()?.collateral_mint.as_ref()
        ],
        bump = alloyx_vault_depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = alloyx_vault_depository.load()?.depository_collateral == alloyx_vault_depository_collateral.key() @UxdError::InvalidDepositoryCollateral,
        constraint = alloyx_vault_depository.load()?.depository_shares == alloyx_vault_depository_shares.key() @UxdError::InvalidDepositoryShares,
        has_one = alloyx_vault_info @UxdError::InvalidAlloyxVaultInfo,
        has_one = alloyx_vault_collateral @UxdError::InvalidAlloyxVaultCollateral,
        has_one = alloyx_vault_shares @UxdError::InvalidAlloyxVaultShares,
        has_one = alloyx_vault_mint @UxdError::InvalidAlloyxVaultMint,
    )]
    pub alloyx_vault_depository: AccountLoader<'info, AlloyxVaultDepository>,

    /// #10
    #[account(mut)]
    pub alloyx_vault_depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(mut)]
    pub alloyx_vault_depository_shares: Box<Account<'info, TokenAccount>>,

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
        constraint = alloyx_vault_pass.investor == alloyx_vault_depository.key() @UxdError::InvalidAlloyxVaultPass,
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
    _ctx: Context<RebalanceAlloyxVaultDepository>,
    _vault_id: &str,
) -> Result<()> {
    // TODO - run rebalance logic
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
