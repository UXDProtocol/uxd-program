use crate::error::UxdError;
use crate::state::mercurial_vault_depository_1::MercurialVaultDepository;
use crate::state::CredixLpDepository;
use crate::utils::checked_u128_to_u64;
use crate::utils::compute_amount_fraction;
use crate::utils::compute_amount_less_fraction;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RebalanceDepositoriesTarget<'info> {
    /// #1 Permissionless IX that can be called by anyone
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mercurial_vault_depositories.contains(&mercurial_vault_depository_1.key()) @UxdError::InvalidDepository,
        constraint = controller.load()?.registered_credix_lp_depositories.contains(&credix_lp_depository_1.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3 The first known active mercurial vault depository
    #[account(
        mut,
        seeds = [
            MERCURIAL_VAULT_DEPOSITORY_NAMESPACE,
            mercurial_vault_depository_1.load()?.mercurial_vault.key().as_ref(),
            mercurial_vault_depository_1.load()?.collateral_mint.as_ref()
        ],
        bump = mercurial_vault_depository_1.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub mercurial_vault_depository_1: AccountLoader<'info, MercurialVaultDepository>,

    /// #4 The first known active credix lp depository
    #[account(
        mut,
        seeds = [
            CREDIX_LP_DEPOSITORY_NAMESPACE,
            credix_lp_depository_1.load()?.credix_global_market_state.key().as_ref(),
            credix_lp_depository_1.load()?.collateral_mint.as_ref()
        ],
        bump = credix_lp_depository_1.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub credix_lp_depository_1: AccountLoader<'info, CredixLpDepository>,
}

pub(crate) fn handler(ctx: Context<RebalanceDepositoriesTarget>) -> Result<()> {
    let controller = &mut ctx.accounts.controller.load()?;
    let mercurial_vault_depository_1 = &mut ctx.accounts.mercurial_vault_depository_1.load_mut()?;
    let credix_lp_depository_1 = &mut ctx.accounts.mercurial_vault_depository_1.load_mut()?;

    let redeemable_circulating_supply =
        checked_u128_to_u64(controller.redeemable_circulating_supply)?;

    let mercurial_vault_fraction_numerator: u64 = 100; // those values could be dynamic depending on on-chain accounts
    let mercurial_vault_fraction_denominator: u64 = 50;
    let mercurial_vault_redeemable_amount_cap: u64 =
        checked_u128_to_u64(mercurial_vault_depository_1.redeemable_amount_under_management_cap)?;
    let mercurial_vault_target_amount = compute_amount_fraction(
        redeemable_circulating_supply,
        mercurial_vault_fraction_numerator,
        mercurial_vault_fraction_denominator,
    )?

    let desired_credix_lp_fraction_numerator: u64 = 100; // those values could be dynamic depending on on-chain accounts
    let desired_credix_lp_fraction_denominator: u64 = 50;
    let desired_credix_lp_target_amount = compute_amount_fraction(
        redeemable_circulating_supply,
        desired_credix_lp_fraction_numerator,
        desired_credix_lp_fraction_denominator,
    )?;

    mercurial_vault_depository_1.rebalancing_redeemable_target_amount =
        desired_mercurial_vault_target_amount;
    credix_lp_depository_1.rebalancing_redeemable_target_amount = desired_credix_lp_target_amount;
    Ok(())
}

impl<'info> RebalanceDepositoriesTarget<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
