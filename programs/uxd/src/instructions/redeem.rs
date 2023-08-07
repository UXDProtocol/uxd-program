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
use crate::utils::checked_add;
use crate::utils::checked_as_u64;
use crate::utils::checked_div;
use crate::utils::checked_mul;
use crate::utils::checked_sub;
use crate::utils::validate_redeemable_amount;
use crate::validate_is_program_frozen;
use crate::BPS_POWER;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
pub struct Redeem<'info> {
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
        constraint = controller.load()?.identity_depository == identity_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.mercurial_vault_depository == mercurial_vault_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.credix_lp_depository == credix_lp_depository.key() @UxdError::InvalidDepository,
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
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, mercurial_vault_depository.load()?.mercurial_vault.key().as_ref(), mercurial_vault_depository.load()?.collateral_mint.as_ref()],
        bump = mercurial_vault_depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = mercurial_vault_depository.load()?.mercurial_vault == mercurial_vault_depository_vault.key() @UxdError::InvalidMercurialVault,
        constraint = mercurial_vault_depository.load()?.mercurial_vault_lp_mint == mercurial_vault_depository_vault_lp_mint.key() @UxdError::InvalidMercurialVaultLpMint,
        constraint = mercurial_vault_depository.load()?.lp_token_vault == mercurial_vault_depository_lp_token_vault.key() @UxdError::InvalidDepositoryLpTokenVault,
    )]
    pub mercurial_vault_depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #11 - Token account holding the LP tokens minted by depositing collateral on mercurial vault
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE, mercurial_vault_depository_vault.key().as_ref(), collateral_mint.key().as_ref()],
        token::authority = mercurial_vault_depository,
        token::mint = mercurial_vault_depository_vault_lp_mint,
        bump = mercurial_vault_depository.load()?.lp_token_vault_bump,
    )]
    pub mercurial_vault_depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #12
    #[account(
        mut,
        constraint = mercurial_vault_depository_vault.token_vault == mercurial_vault_depository_collateral_token_safe.key() @UxdError::InvalidMercurialVaultCollateralTokenSafe,
    )]
    pub mercurial_vault_depository_vault: Box<Account<'info, mercurial_vault::state::Vault>>,

    /// #13
    #[account(mut)]
    pub mercurial_vault_depository_vault_lp_mint: Box<Account<'info, Mint>>,

    /// #14 - Token account owned by the mercurial vault program. Hold the collateral deposited in the mercurial vault.
    #[account(mut)]
    pub mercurial_vault_depository_collateral_token_safe: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(
        mut,
        seeds = [CREDIX_LP_DEPOSITORY_NAMESPACE, credix_lp_depository.load()?.credix_global_market_state.key().as_ref(), credix_lp_depository.load()?.collateral_mint.as_ref()],
        bump = credix_lp_depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub credix_lp_depository: AccountLoader<'info, CredixLpDepository>,

    /// #16
    pub system_program: Program<'info, System>,

    /// #17
    pub token_program: Program<'info, Token>,

    /// #18
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// #19
    pub mercurial_vault_program: Program<'info, mercurial_vault::program::Vault>,

    /// #20
    pub uxd_program: Program<'info, crate::program::Uxd>,

    /// #21
    pub rent: Sysvar<'info, Rent>,
}

struct DepositoryInfoForRedeem<'info> {
    pub weight_bps: u16,
    pub redeemable_amount_under_management: u128,
    pub redeemable_amount_under_management_cap: u128,
    pub redeem_fn: Option<Box<dyn Fn(u64) -> Result<()> + 'info>>,
}

pub(crate) fn handler(ctx: Context<Redeem>, redeemable_amount: u64) -> Result<()> {
    // Gather all the onchain states we need (caps, weights and supplies)
    let mut controller = ctx.accounts.controller.load_mut()?;
    let identity_depository = ctx.accounts.identity_depository.load()?;
    let mercurial_vault_depository = ctx.accounts.mercurial_vault_depository.load()?;
    let credix_lp_depository = ctx.accounts.credix_lp_depository.load()?;

    // Compute real outflow limit for this epoch (max of bps/amount options)
    // Note: intermediary maths forced to use u128 to be able to multiply u64s safely
    let outflow_limit_per_epoch_amount = std::cmp::max(
        controller.outflow_limit_per_epoch_amount,
        checked_as_u64(checked_div::<u128>(
            checked_mul::<u128>(
                controller.redeemable_circulating_supply,
                u128::from(controller.outflow_limit_per_epoch_bps),
            )?,
            u128::from(BPS_POWER),
        )?)?,
    );

    // How long ago was the last outflow
    let current_slot = Clock::get()?.slot;
    let last_outflow_elapsed_slots = std::cmp::min(
        checked_as_u64(checked_sub(current_slot, controller.last_outflow_slot)?)?,
        controller.slots_per_epoch,
    );

    // How much was unlocked by waiting since last redeem
    // Note: intermediary maths forced to use u128 to be able to multiply u64s safely
    let unlocked_outflow_amount = checked_as_u64(checked_div::<u128>(
        checked_mul::<u128>(
            u128::from(last_outflow_elapsed_slots),
            u128::from(outflow_limit_per_epoch_amount),
        )?,
        u128::from(controller.slots_per_epoch),
    )?)?;

    // How much outflow in the current epoch right before the redeem IX
    let previous_epoch_outflow_amount = controller
        .epoch_outflow_amount
        .saturating_sub(unlocked_outflow_amount);

    // How much outflow in the current epoch right after this current redeem IX
    let new_epoch_outflow_amount = checked_add(previous_epoch_outflow_amount, redeemable_amount)?;

    // Make sure we are not over the outflow limit!
    require!(
        new_epoch_outflow_amount <= outflow_limit_per_epoch_amount,
        UxdError::MaximumOutflowAmountError
    );

    // Update outflow limitations flags
    controller.epoch_outflow_amount = new_epoch_outflow_amount;
    controller.last_outflow_slot = current_slot;

    // Make controller signer
    let controller_pda_signer: &[&[&[u8]]] = &[&[CONTROLLER_NAMESPACE, &[controller.bump]]];

    // The actual post-redeem circulating supply may be slightly higher
    // Due to redeem fees and precision loss. But the difference should be negligible and
    // Any future mint/redeem will recompute the targets based on the exact future circulating supply anyway
    let minimum_after_redeem_circulating_supply = checked_sub(
        controller.redeemable_circulating_supply,
        u128::from(redeemable_amount),
    )?;

    // Build the vector of all known depository participating in the routing system
    let depository_info = vec![
        // Identity depository details
        DepositoryInfoForRedeem {
            weight_bps: controller.identity_depository_weight_bps,
            redeemable_amount_under_management: identity_depository
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: identity_depository
                .redeemable_amount_under_management_cap,
            redeem_fn: Some(Box::new(|redeemable_amount| {
                msg!(
                    "[redeem:redeem_from_identity_depository:{}]",
                    redeemable_amount
                );
                if redeemable_amount > 0 {
                    uxd_cpi::cpi::redeem_from_identity_depository(
                        ctx.accounts
                            .into_redeem_from_identity_depository_context()
                            .with_signer(controller_pda_signer),
                        redeemable_amount,
                    )?;
                }
                Ok(())
            })),
        },
        // Mercurial Vault Depository details
        DepositoryInfoForRedeem {
            weight_bps: controller.mercurial_vault_depository_weight_bps,
            redeemable_amount_under_management: mercurial_vault_depository
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: mercurial_vault_depository
                .redeemable_amount_under_management_cap,
            redeem_fn: Some(Box::new(|redeemable_amount| {
                msg!(
                    "[redeem:redeem_from_mercurial_vault_depository:{}]",
                    redeemable_amount
                );
                if redeemable_amount > 0 {
                    uxd_cpi::cpi::redeem_from_mercurial_vault_depository(
                        ctx.accounts
                            .into_redeem_from_mercurial_vault_depository_context()
                            .with_signer(controller_pda_signer),
                        redeemable_amount,
                    )?;
                }
                Ok(())
            })),
        },
        // Credix Lp Depository details
        DepositoryInfoForRedeem {
            weight_bps: controller.credix_lp_depository_weight_bps,
            redeemable_amount_under_management: credix_lp_depository
                .redeemable_amount_under_management,
            redeemable_amount_under_management_cap: credix_lp_depository
                .redeemable_amount_under_management_cap,
            redeem_fn: None, // credix is illiquid
        },
    ];

    drop(controller);
    drop(identity_depository);
    drop(mercurial_vault_depository);
    drop(credix_lp_depository);

    // Compute the desired target amounts for each depository
    let depositories_target_redeemable_amount = calculate_depositories_target_redeemable_amount(
        minimum_after_redeem_circulating_supply,
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
                is_liquid: depository_info.redeem_fn.is_some(), // we are liquid if we can redeem
                target_redeemable_amount: *depository_target_redeemable_amount,
                redeemable_amount_under_management: depository_info
                    .redeemable_amount_under_management,
            }
        })
        .collect::<Vec<DepositoryInfoForRedeemableAmount>>(),
    )?;

    // Run all the redeem cpi functions with the redeemable amount if liquid
    std::iter::zip(
        depository_info.iter(),
        depositories_redeemable_amount.iter(),
    )
    .try_for_each(
        |(depository_info, depository_redeemable_amount)| match &depository_info.redeem_fn {
            Some(redeem_fn) => redeem_fn(*depository_redeemable_amount),
            None => Ok(()),
        },
    )?;

    // Done
    Ok(())
}

// Into functions
impl<'info> Redeem<'info> {
    pub fn into_redeem_from_identity_depository_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, uxd_cpi::cpi::accounts::RedeemFromIdentityDepository<'info>>
    {
        let cpi_accounts = uxd_cpi::cpi::accounts::RedeemFromIdentityDepository {
            authority: self.controller.to_account_info(),
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

    pub fn into_redeem_from_mercurial_vault_depository_context(
        &self,
    ) -> CpiContext<
        '_,
        '_,
        '_,
        'info,
        uxd_cpi::cpi::accounts::RedeemFromMercurialVaultDepository<'info>,
    > {
        let cpi_accounts = uxd_cpi::cpi::accounts::RedeemFromMercurialVaultDepository {
            authority: self.controller.to_account_info(),
            user: self.user.to_account_info(),
            payer: self.payer.to_account_info(),
            controller: self.controller.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            collateral_mint: self.collateral_mint.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            user_collateral: self.user_collateral.to_account_info(),
            depository: self.mercurial_vault_depository.to_account_info(),
            depository_lp_token_vault: self
                .mercurial_vault_depository_lp_token_vault
                .to_account_info(),
            mercurial_vault: self.mercurial_vault_depository_vault.to_account_info(),
            mercurial_vault_lp_mint: self
                .mercurial_vault_depository_vault_lp_mint
                .to_account_info(),
            mercurial_vault_collateral_token_safe: self
                .mercurial_vault_depository_collateral_token_safe
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
impl<'info> Redeem<'info> {
    pub(crate) fn validate(&self, redeemable_amount: u64) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_redeemable_amount(&self.user_redeemable, redeemable_amount)?;
        Ok(())
    }
}
