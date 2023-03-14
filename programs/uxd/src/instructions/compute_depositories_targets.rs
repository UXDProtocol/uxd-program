use crate::error::UxdError;
use crate::state::CredixLpDepository;
use crate::state::MercurialVaultDepository;
use crate::utils::checked_convert_u128_to_u64;
use crate::utils::compute_amount_fraction_floor;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::BPS_UNIT_CONVERSION;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ComputeDepositoriesTargets<'info> {
    /// #1 Permissionless IX that can be called by anyone
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_mercurial_vault_depositories.contains(&mercurial_vault_depository.key()) @UxdError::InvalidDepository,
        constraint = controller.load()?.registered_credix_lp_depositories.contains(&credix_lp_depository.key()) @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3 The first known active mercurial vault depository
    #[account(
        mut,
        seeds = [
            MERCURIAL_VAULT_DEPOSITORY_NAMESPACE,
            mercurial_vault_depository.load()?.mercurial_vault.key().as_ref(),
            mercurial_vault_depository.load()?.collateral_mint.as_ref()
        ],
        bump = mercurial_vault_depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub mercurial_vault_depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #4 The first known active credix lp depository
    #[account(
        mut,
        seeds = [
            CREDIX_LP_DEPOSITORY_NAMESPACE,
            credix_lp_depository.load()?.credix_global_market_state.key().as_ref(),
            credix_lp_depository.load()?.collateral_mint.as_ref()
        ],
        bump = credix_lp_depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub credix_lp_depository: AccountLoader<'info, CredixLpDepository>,
}

pub(crate) fn handler(ctx: Context<ComputeDepositoriesTargets>) -> Result<()> {
    let controller = &ctx.accounts.controller.load()?;
    let mercurial_vault_depository = &mut ctx.accounts.mercurial_vault_depository.load_mut()?;
    let credix_lp_depository = &mut ctx.accounts.credix_lp_depository.load_mut()?;

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Read the desired weights for each depository on chain
    // -- And generate a raw_target estimations that we can refine later
    // ---------------------------------------------------------------------

    // We want to balance the supply on weighted portions of the circulating supply for now
    // This could be based on dynamic on-chain account values and liquidity later
    let mercurial_vault_depository_weight_bps =
        mercurial_vault_depository.redeemable_amount_under_management_weight_bps;
    let credix_lp_depository_weight_bps =
        credix_lp_depository.redeemable_amount_under_management_weight_bps;

    // Compute raw target values based on weighted portions of circulating supply
    let redeemable_circulating_supply =
        checked_convert_u128_to_u64(controller.redeemable_circulating_supply)?;

    let mercurial_vault_depository_raw_target = ctx.accounts.compute_raw_target(
        redeemable_circulating_supply,
        mercurial_vault_depository_weight_bps,
    )?;
    let credix_lp_depository_raw_target = ctx.accounts.compute_raw_target(
        redeemable_circulating_supply,
        credix_lp_depository_weight_bps,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Using the raw_target and the depository cap:
    // -- * Compute the overflow (raw target amount above the cap)
    // -- * Compute the availability (raw target amount until the cap)
    // ---------------------------------------------------------------------

    // Read the minting caps of each depository
    let mercurial_vault_depository_cap = checked_convert_u128_to_u64(
        mercurial_vault_depository.redeemable_amount_under_management_cap,
    )?;
    let credix_lp_depository_cap =
        checked_convert_u128_to_u64(credix_lp_depository.redeemable_amount_under_management_cap)?;

    // Compute the depository_overflow amount of raw target that doesn't fit within the cap of each depository
    let mercurial_vault_depository_overflow = ctx.accounts.compute_overflow(
        mercurial_vault_depository_raw_target,
        mercurial_vault_depository_cap,
    )?;
    let credix_lp_depository_overflow = ctx
        .accounts
        .compute_overflow(credix_lp_depository_raw_target, credix_lp_depository_cap)?;

    // Compute the amount of space available under the cap in each depository
    let mercurial_vault_depository_availability = ctx.accounts.compute_availability(
        mercurial_vault_depository_raw_target,
        mercurial_vault_depository_cap,
    )?;
    let credix_lp_depository_availability = ctx
        .accounts
        .compute_availability(credix_lp_depository_raw_target, credix_lp_depository_cap)?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Compute the combined overflow of all depositories
    // -- Compute the combined availability of all depositories
    // ---------------------------------------------------------------------

    // Compute total amount that doesn't fit within depositories cap
    let total_overflow = ctx.accounts.compute_total(
        mercurial_vault_depository_overflow,
        credix_lp_depository_overflow,
    )?;
    // Compute total amount that doesn't fit within depositories cap
    let total_availability = ctx.accounts.compute_total(
        mercurial_vault_depository_availability,
        credix_lp_depository_availability,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Compute the final target based off of the logic:
    // -- * FinalTarget = raw_target - overflow + ExtraBonus
    // -- * ExtraBonus = total_overflow * (availability / total_availability)
    // -- In other words:
    // -- * The final target is capped at the cap
    // -- * Any amount overflowing that is transfered to others depositories
    // -- * Depositories with available space will receive overflows
    // ---------------------------------------------------------------------

    // Compute the final targets for each depository
    let mercurial_vault_depository_final_target = ctx.accounts.compute_final_target(
        mercurial_vault_depository_raw_target,
        mercurial_vault_depository_overflow,
        mercurial_vault_depository_availability,
        total_overflow,
        total_availability,
    )?;
    let credix_lp_depository_final_target = ctx.accounts.compute_final_target(
        credix_lp_depository_raw_target,
        credix_lp_depository_overflow,
        credix_lp_depository_availability,
        total_overflow,
        total_availability,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 5
    // -- Update the on-chain depositories with computed results
    // ---------------------------------------------------------------------

    // Update onchain accounts
    mercurial_vault_depository.redeemable_amount_under_management_target_amount =
        mercurial_vault_depository_final_target;
    credix_lp_depository.redeemable_amount_under_management_target_amount =
        credix_lp_depository_final_target;

    // Success
    Ok(())
}

// Into functions
impl<'info> ComputeDepositoriesTargets<'info> {
    // Compute a simple raw target: raw_target = total_circulating_supply * weight
    pub fn compute_raw_target(
        &self,
        redeemable_circulating_supply: u64,
        depository_weight_bps: u16,
    ) -> Result<u64> {
        let depository_raw_target = compute_amount_fraction_floor(
            redeemable_circulating_supply,
            depository_weight_bps.into(),
            BPS_UNIT_CONVERSION,
        )?;
        Ok(depository_raw_target)
    }

    // Compute the overflow value: overflow = max(0, raw_target - cap)
    pub fn compute_overflow(&self, depository_raw_target: u64, depository_cap: u64) -> Result<u64> {
        if depository_raw_target <= depository_cap {
            return Ok(0);
        }
        Ok(depository_raw_target
            .checked_sub(depository_cap)
            .ok_or(UxdError::MathError)?)
    }

    // Compute the availability value: availability = max(0, cap - raw_target)
    pub fn compute_availability(
        &self,
        depository_raw_target: u64,
        depository_cap: u64,
    ) -> Result<u64> {
        if depository_raw_target >= depository_cap {
            return Ok(0);
        }
        Ok(depository_cap
            .checked_sub(depository_raw_target)
            .ok_or(UxdError::MathError)?)
    }

    // Compute the total of a value when adding all depositories
    pub fn compute_total(
        &self,
        mercurial_vault_depository_value: u64,
        credix_lp_depository_value: u64,
    ) -> Result<u64> {
        Ok(mercurial_vault_depository_value
            .checked_add(credix_lp_depository_value)
            .ok_or(UxdError::MathError)?)
    }

    // Compute the final target based of overflow and availabilities of all depositories
    pub fn compute_final_target(
        &self,
        depository_raw_target: u64,
        depository_overflow: u64,
        depository_availability: u64,
        total_overflow: u64,
        total_availability: u64,
    ) -> Result<u64> {
        let overflow_amount_recuperated_from_other_depositories: u64 =
            compute_amount_fraction_floor(
                total_overflow,
                depository_availability,
                total_availability,
            )?;

        let final_target = depository_raw_target
            .checked_sub(depository_overflow)
            .ok_or(UxdError::MathError)?
            .checked_add(overflow_amount_recuperated_from_other_depositories)
            .ok_or(UxdError::MathError)?;
        Ok(final_target)
    }
}

// Validate
impl<'info> ComputeDepositoriesTargets<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
