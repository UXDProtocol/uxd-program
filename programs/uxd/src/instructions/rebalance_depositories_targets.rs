use crate::error::UxdError;
use crate::state::CredixLpDepository;
use crate::state::MercurialVaultDepository;
use crate::utils::checked_u128_to_u64;
use crate::utils::compute_amount_fraction;
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
    let mercurial_vault_depository_1 = &mut ctx.accounts.mercurial_vault_depository_1.load_mut()?;
    let credix_lp_depository_1 = &mut ctx.accounts.mercurial_vault_depository_1.load_mut()?;

    // We want to balance the supply on percents of the circulating supply for now
    // This could be based on dynamic on-chain account values and liquidity later
    let mercurial_vault_depository_1_percent: u64 = 50;
    let credix_lp_depository_1_percent: u64 = 50;

    // Compute raw target values based on percent of total circulating supply
    let mercurial_vault_depository_1_raw_target = ctx
        .accounts
        .compute_raw_target(mercurial_vault_depository_1_percent)?;
    let credix_lp_depository_1_raw_target = ctx
        .accounts
        .compute_raw_target(credix_lp_depository_1_percent)?;

    // Read the minting hard caps of each depository
    let mercurial_vault_depository_1_cap =
        checked_u128_to_u64(mercurial_vault_depository_1.redeemable_amount_under_management_cap)?;
    let credix_lp_depository_1_cap =
        checked_u128_to_u64(credix_lp_depository_1.redeemable_amount_under_management_cap)?;

    // Compute the overflow amount of raw target that doesn't fit within the cap of each depository
    let mercurial_vault_depository_1_overflow = ctx.accounts.compute_overflow(
        mercurial_vault_depository_1_raw_target,
        mercurial_vault_depository_1_cap,
    )?;
    let credix_lp_depository_1_overflow = ctx.accounts.compute_overflow(
        credix_lp_depository_1_raw_target,
        credix_lp_depository_1_cap,
    )?;

    // Compute the amount of space available under the hard cap in each depository
    let mercurial_vault_depository_1_availability = ctx.accounts.compute_availability(
        mercurial_vault_depository_1_raw_target,
        mercurial_vault_depository_1_cap,
    )?;
    let credix_lp_depository_1_availability = ctx.accounts.compute_availability(
        credix_lp_depository_1_raw_target,
        credix_lp_depository_1_cap,
    )?;

    // Compute total amount that doesn't fit within depositories cap
    let total_overflow = ctx.accounts.compute_total(
        mercurial_vault_depository_1_overflow,
        credix_lp_depository_1_overflow,
    )?;
    // Compute total amount that doesn't fit within depositories cap
    let total_availability = ctx.accounts.compute_total(
        mercurial_vault_depository_1_availability,
        credix_lp_depository_1_availability,
    )?;

    // Compute the final targets for each depository
    let mercurial_vault_depository_1_final_target = ctx.accounts.compute_final_target(
        mercurial_vault_depository_1_raw_target,
        mercurial_vault_depository_1_overflow,
        mercurial_vault_depository_1_availability,
        total_overflow,
        total_availability,
    )?;
    let credix_lp_depository_1_final_target = ctx.accounts.compute_final_target(
        credix_lp_depository_1_raw_target,
        credix_lp_depository_1_overflow,
        credix_lp_depository_1_availability,
        total_overflow,
        total_availability,
    )?;

    // Update onchain accounts
    mercurial_vault_depository_1.rebalancing_redeemable_target_amount =
        mercurial_vault_depository_1_final_target;
    credix_lp_depository_1.rebalancing_redeemable_target_amount =
        credix_lp_depository_1_final_target;

    // Success
    Ok(())
}

// Into functions
impl<'info> RebalanceDepositoriesTarget<'info> {
    pub fn compute_raw_target(&self, percent_of_circulating_supply: u64) -> Result<u64> {
        let controller = &self.controller.load()?;
        let raw_target = compute_amount_fraction(
            checked_u128_to_u64(controller.redeemable_circulating_supply)?,
            percent_of_circulating_supply,
            100,
        )?;
        Ok(raw_target)
    }

    pub fn compute_overflow(
        &self,
        raw_target: u64,
        redeemable_under_management_cap: u64,
    ) -> Result<u64> {
        if raw_target <= redeemable_under_management_cap {
            return Ok(0);
        }
        Ok(raw_target
            .checked_sub(redeemable_under_management_cap)
            .ok_or(UxdError::MathError)?)
    }

    pub fn compute_availability(
        &self,
        raw_target: u64,
        redeemable_under_management_cap: u64,
    ) -> Result<u64> {
        if raw_target >= redeemable_under_management_cap {
            return Ok(0);
        }
        Ok(redeemable_under_management_cap
            .checked_sub(raw_target)
            .ok_or(UxdError::MathError)?)
    }

    pub fn compute_total(
        &self,
        mercurial_vault_depository_1_value: u64,
        credix_lp_depository_1_value: u64,
    ) -> Result<u64> {
        Ok(mercurial_vault_depository_1_value
            .checked_add(credix_lp_depository_1_value)
            .ok_or(UxdError::MathError)?)
    }

    pub fn compute_final_target(
        &self,
        raw_target: u64,
        overflow: u64,
        availability: u64,
        total_overflow: u64,
        total_availability: u64,
    ) -> Result<u64> {
        let overflow_from_others: u128 = u128::from(availability)
            .checked_mul(total_overflow.into())
            .ok_or(UxdError::MathError)?
            .checked_div(total_availability.into())
            .ok_or(UxdError::MathError)?;

        let final_target = raw_target
            .checked_sub(overflow)
            .ok_or(UxdError::MathError)?
            .checked_add(checked_u128_to_u64(overflow_from_others)?)
            .ok_or(UxdError::MathError)?;

        Ok(final_target)
    }
}

// Validate
impl<'info> RebalanceDepositoriesTarget<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
