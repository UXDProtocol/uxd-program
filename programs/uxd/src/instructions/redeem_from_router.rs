use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::error::UxdError;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::utils::calculate_depositories_redeemable_amount;
use crate::utils::calculate_depositories_redeemable_amount::DepositoryInfoForRedeemableAmount;
use crate::utils::calculate_depositories_target_redeemable_amount;
use crate::utils::calculate_depositories_target_redeemable_amount::DepositoryInfoForTargetRedeemableAmount;
use crate::utils::validate_redeemable_amount;
use crate::validate_is_program_frozen;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use crate::ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX;
use crate::ROUTER_IDENTITY_DEPOSITORY_INDEX;
use crate::ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX;

#[derive(Accounts)]
#[instruction(redeemable_amount: u64)]
pub struct RedeemFromRouter<'info> {
    /// #1
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mercurial_vault_depositories[0] == mercurial_vault_depository_0.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.registered_credix_lp_depositories[0] == credix_lp_depository_0.key() @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #5
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account(
        mut,
        constraint = user_redeemable.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidRedeemableMint,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        mut,
        constraint = user_collateral.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_collateral.mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #8 - UXDProgram on chain account bound to a Controller instance that represent the blank minting/redeeming
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE],
        bump = identity_depository.load()?.bump,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #9 - Token account holding the collateral from minting
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE],
        token::authority = identity_depository,
        token::mint = identity_depository.load()?.collateral_mint,
        bump = identity_depository.load()?.collateral_vault_bump,
    )]
    pub identity_depository_collateral_vault: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, mercurial_vault_depository_0.load()?.mercurial_vault.key().as_ref(), mercurial_vault_depository_0.load()?.collateral_mint.as_ref()],
        bump = mercurial_vault_depository_0.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = mercurial_vault_depository_0.load()?.mercurial_vault == mercurial_vault_depository_0_vault.key() @UxdError::InvalidMercurialVault,
        constraint = mercurial_vault_depository_0.load()?.mercurial_vault_lp_mint == mercurial_vault_depository_0_vault_lp_mint.key() @UxdError::InvalidMercurialVaultLpMint,
        constraint = mercurial_vault_depository_0.load()?.lp_token_vault == mercurial_vault_depository_0_lp_token_vault.key() @UxdError::InvalidDepositoryLpTokenVault,
    )]
    pub mercurial_vault_depository_0: AccountLoader<'info, MercurialVaultDepository>,

    /// #11 - Token account holding the LP tokens minted by depositing collateral on mercurial vault
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE, mercurial_vault_depository_0_vault.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = mercurial_vault_depository_0,
        token::mint = mercurial_vault_depository_0_vault_lp_mint,
        bump = mercurial_vault_depository_0.load()?.lp_token_vault_bump,
    )]
    pub mercurial_vault_depository_0_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #12
    #[account(
        mut,
        constraint = mercurial_vault_depository_0_vault.token_vault == mercurial_vault_depository_0_collateral_token_safe.key() @UxdError::InvalidMercurialVaultCollateralTokenSafe,
    )]
    pub mercurial_vault_depository_0_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #13
    #[account(mut)]
    pub mercurial_vault_depository_0_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #14 - Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.
    #[account(mut)]
    pub mercurial_vault_depository_0_collateral_token_safe: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(
        mut,
        seeds = [CREDIX_LP_DEPOSITORY_NAMESPACE, credix_lp_depository_0.load()?.credix_global_market_state.key().as_ref(), credix_lp_depository_0.load()?.collateral_mint.as_ref()],
        bump = credix_lp_depository_0.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub credix_lp_depository_0: AccountLoader<'info, CredixLpDepository>,

    /// #23
    pub system_program: Program<'info, System>,

    /// #24
    pub token_program: Program<'info, Token>,

    /// #25
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #26
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,

    /// #28
    pub uxd_program: Program<'info, crate::program::Uxd>,

    /// #29
    pub rent: Sysvar<'info, Rent>,
}

struct DepositoryInfoForRedeemFromRouter {
    pub is_liquid: bool,
    pub weight_bps: u16,
    pub redeemable_amount_under_management: u128,
    pub redeemable_amount_under_management_cap: u128,
}

pub(crate) fn handler(ctx: Context<RedeemFromRouter>, redeemable_amount: u64) -> Result<()> {
    // Gather all the onchain states we need (caps, weights and supplies)
    let controller = ctx.accounts.controller.load()?;
    let identity_depository = ctx.accounts.identity_depository.load()?;
    let mercurial_vault_depository_0 = ctx.accounts.mercurial_vault_depository_0.load()?;
    let credix_lp_depository_0 = ctx.accounts.credix_lp_depository_0.load()?;

    let redeemable_circulating_supply_before = controller.redeemable_circulating_supply;
    let redeemable_circulating_supply_after = redeemable_circulating_supply_before
        .checked_sub(redeemable_amount.into())
        .ok_or(UxdError::MathError)?;

    let depository_info = vec![
        // Identity depository details
        DepositoryInfoForRedeemFromRouter {
            is_liquid: true,
            weight_bps: controller.identity_depository_weight_bps,
            redeemable_amount_under_management: identity_depository
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: identity_depository
                .redeemable_amount_under_management_cap,
        },
        // Mercurial Vault Depository 0 details
        DepositoryInfoForRedeemFromRouter {
            is_liquid: true,
            weight_bps: controller.mercurial_vault_depository_0_weight_bps,
            redeemable_amount_under_management: mercurial_vault_depository_0
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: mercurial_vault_depository_0
                .redeemable_amount_under_management_cap,
        },
        // Credix Lp Depository 0 details
        DepositoryInfoForRedeemFromRouter {
            is_liquid: false,
            weight_bps: controller.credix_lp_depository_0_weight_bps,
            redeemable_amount_under_management: credix_lp_depository_0
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: credix_lp_depository_0
                .redeemable_amount_under_management_cap,
        },
    ];

    drop(controller);
    drop(identity_depository);
    drop(mercurial_vault_depository_0);
    drop(credix_lp_depository_0);

    // Compute the desired target amounts for each depository
    let depositories_target_redeemable_amount = calculate_depositories_target_redeemable_amount(
        redeemable_circulating_supply_after,
        &depository_info
            .iter()
            .map(|depository_info| DepositoryInfoForTargetRedeemableAmount {
                weight_bps: depository_info.weight_bps,
                redeemable_amount_under_management_cap: depository_info
                    .redeemable_amount_under_management_cap,
            })
            .collect::<Vec<DepositoryInfoForTargetRedeemableAmount>>(),
    )?;

    // Compute the desired mint amounts for each depository
    let depositories_redeemable_amount = calculate_depositories_redeemable_amount(
        redeemable_amount,
        &std::iter::zip(
            depository_info.iter(),
            depositories_target_redeemable_amount.iter(),
        )
        .map(|(depository_info, depository_target_redeemable_amount)| {
            DepositoryInfoForRedeemableAmount {
                is_liquid: depository_info.is_liquid,
                target_redeemable_amount: *depository_target_redeemable_amount,
                redeemable_amount_under_management: depository_info
                    .redeemable_amount_under_management,
            }
        })
        .collect::<Vec<DepositoryInfoForRedeemableAmount>>(),
    )?;

    // Redeem the desired amount from identity_depository
    let identity_depository_redeemable_amount =
        depositories_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX];
    msg!(
        "[redeem_from_router:redeem_from_identity_depository:{}]",
        identity_depository_redeemable_amount
    );
    if identity_depository_redeemable_amount > 0 {
        uxd_cpi::cpi::redeem_from_identity_depository(
            ctx.accounts.into_redeem_from_identity_depository_context(),
            identity_depository_redeemable_amount,
        )?;
    }

    // Redeem the desired amount from mercurial_vault_depository_0
    let mercurial_vault_depository_0_redeemable_amount =
        depositories_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX];
    msg!(
        "[redeem_from_router:redeem_from_mercurial_vault_depository_0:{}]",
        mercurial_vault_depository_0_redeemable_amount
    );
    if mercurial_vault_depository_0_redeemable_amount > 0 {
        uxd_cpi::cpi::redeem_from_mercurial_vault_depository(
            ctx.accounts
                .into_redeem_from_mercurial_vault_depository_0_context(),
            mercurial_vault_depository_0_redeemable_amount,
        )?;
    }

    // Done
    Ok(())
}

// Into functions
impl<'info> RedeemFromRouter<'info> {
    pub fn into_redeem_from_identity_depository_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, uxd_cpi::cpi::accounts::RedeemFromIdentityDepository<'info>>
    {
        let cpi_accounts = uxd_cpi::cpi::accounts::RedeemFromIdentityDepository {
            user: self.user.to_account_info(),
            payer: self.payer.to_account_info(),
            controller: self.controller.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            user_collateral: self.user_collateral.to_account_info(),
            depository: self.identity_depository.to_account_info(),
            collateral_vault: self.identity_depository_collateral_vault.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.uxd_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_redeem_from_mercurial_vault_depository_0_context(
        &self,
    ) -> CpiContext<
        '_,
        '_,
        '_,
        'info,
        uxd_cpi::cpi::accounts::RedeemFromMercurialVaultDepository<'info>,
    > {
        let cpi_accounts = uxd_cpi::cpi::accounts::RedeemFromMercurialVaultDepository {
            user: self.user.to_account_info(),
            payer: self.payer.to_account_info(),
            controller: self.controller.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            collateral_mint: self.collateral_mint.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            user_collateral: self.user_collateral.to_account_info(),
            depository: self.mercurial_vault_depository_0.to_account_info(),
            depository_lp_token_vault: self
                .mercurial_vault_depository_0_lp_token_vault
                .to_account_info(),
            mercurial_vault: self.mercurial_vault_depository_0_vault.to_account_info(),
            mercurial_vault_lp_mint: self
                .mercurial_vault_depository_0_vault_lp_mint
                .to_account_info(),
            mercurial_vault_collateral_token_safe: self
                .mercurial_vault_depository_0_collateral_token_safe
                .to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            mercurial_vault_program: self.mercurial_vault_program.to_account_info(),
        };
        let cpi_program = self.mercurial_vault_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> RedeemFromRouter<'info> {
    pub(crate) fn validate(&self, redeemable_amount: u64) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_redeemable_amount(&self.user_collateral, redeemable_amount)?;
        Ok(())
    }
}
