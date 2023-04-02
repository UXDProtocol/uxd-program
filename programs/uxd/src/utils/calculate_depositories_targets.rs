use anchor_lang::prelude::Result;

use crate::error::UxdError;
use crate::BPS_UNIT_CONVERSION;

use super::checked_convert_u128_to_u64;
use super::compute_amount_fraction_floor;

pub struct DepositoriesTargets {
    pub identity_depository_target_amount: u64,
    pub mercurial_vault_depository_0_target_amount: u64,
    pub credix_lp_depository_0_target_amount: u64,
}

pub fn calculate_depositories_targets(
    redeemable_circulating_supply: u128,
    identity_depository_weight_bps: u16,
    mercurial_vault_depository_0_weight_bps: u16,
    credix_lp_depository_0_weight_bps: u16,
    identity_depository_redeemable_amount_under_management_cap: u128,
    mercurial_vault_depository_0_redeemable_amount_under_management_cap: u128,
    credix_lp_depository_0_redeemable_amount_under_management_cap: u128,
) -> Result<DepositoriesTargets> {
    let redeemable_circulating_supply = checked_convert_u128_to_u64(redeemable_circulating_supply)?;

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Read the desired weights for each depository on chain
    // -- And generate a raw_target estimations that we can refine later
    // ---------------------------------------------------------------------

    let identity_depository_raw_target_amount = calculate_depository_raw_target_amount(
        redeemable_circulating_supply,
        identity_depository_weight_bps,
    )?;
    let mercurial_vault_depository_0_raw_target_amount = calculate_depository_raw_target_amount(
        redeemable_circulating_supply,
        mercurial_vault_depository_0_weight_bps,
    )?;
    let credix_lp_depository_0_raw_target_amount = calculate_depository_raw_target_amount(
        redeemable_circulating_supply,
        credix_lp_depository_0_weight_bps,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Using the raw_target and the depository cap:
    // -- Compute the overflow (raw target amount above the cap)
    // -- Compute the availability (raw target amount until the cap)
    // ---------------------------------------------------------------------

    // Read the minting caps of each depository
    let identity_depository_hard_cap_amount =
        checked_convert_u128_to_u64(identity_depository_redeemable_amount_under_management_cap)?;
    let mercurial_vault_depository_0_hard_cap_amount = checked_convert_u128_to_u64(
        mercurial_vault_depository_0_redeemable_amount_under_management_cap,
    )?;
    let credix_lp_depository_0_hard_cap_amount =
        checked_convert_u128_to_u64(credix_lp_depository_0_redeemable_amount_under_management_cap)?;

    // Compute the depository_overflow amount of raw target that doesn't fit within the cap of each depository
    let identity_depository_overflow_amount = calculate_depository_overflow_amount(
        identity_depository_raw_target_amount,
        identity_depository_hard_cap_amount,
    )?;
    let mercurial_vault_depository_0_overflow_amount = calculate_depository_overflow_amount(
        mercurial_vault_depository_0_raw_target_amount,
        mercurial_vault_depository_0_hard_cap_amount,
    )?;
    let credix_lp_depository_0_overflow_amount = calculate_depository_overflow_amount(
        credix_lp_depository_0_raw_target_amount,
        credix_lp_depository_0_hard_cap_amount,
    )?;

    // Compute the amount of space available under the cap in each depository
    let identity_depository_available_amount = calculate_depository_available_amount(
        identity_depository_raw_target_amount,
        identity_depository_hard_cap_amount,
    )?;
    let mercurial_vault_depository_0_available_amount = calculate_depository_available_amount(
        mercurial_vault_depository_0_raw_target_amount,
        mercurial_vault_depository_0_hard_cap_amount,
    )?;
    let credix_lp_depository_0_available_amount = calculate_depository_available_amount(
        credix_lp_depository_0_raw_target_amount,
        credix_lp_depository_0_hard_cap_amount,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Compute the combined overflow of all depositories
    // -- Compute the combined availability of all depositories
    // ---------------------------------------------------------------------

    // Compute total amount that doesn't fit within depositories hard cap
    let total_overflow_amount = calculate_depositories_total_amount(
        identity_depository_overflow_amount,
        mercurial_vault_depository_0_overflow_amount,
        credix_lp_depository_0_overflow_amount,
    )?;
    // Compute total amount that doesn't fit within depositories hard cap
    let total_available_amount = calculate_depositories_total_amount(
        identity_depository_available_amount,
        mercurial_vault_depository_0_available_amount,
        credix_lp_depository_0_available_amount,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Compute the final target based off of the logic:
    // -- Target = raw_target - overflow_amount + Extras
    // -- Extras = total_overflow_amount * (available_amount / total_available_amount)
    // -- In other words:
    // -- The final target is capped at the depository hard cap
    // -- Any amount overflowing that, is allocated to others depositories
    // -- Depositories with available space will receive a portion of allocated overflows
    // ---------------------------------------------------------------------

    // Compute the final targets for each depository
    let identity_depository_target_amount = calculate_depository_target_amount(
        identity_depository_raw_target_amount,
        identity_depository_overflow_amount,
        identity_depository_available_amount,
        total_overflow_amount,
        total_available_amount,
    )?;
    let mercurial_vault_depository_0_target_amount = calculate_depository_target_amount(
        mercurial_vault_depository_0_raw_target_amount,
        mercurial_vault_depository_0_overflow_amount,
        mercurial_vault_depository_0_available_amount,
        total_overflow_amount,
        total_available_amount,
    )?;
    let credix_lp_depository_0_target_amount = calculate_depository_target_amount(
        credix_lp_depository_0_raw_target_amount,
        credix_lp_depository_0_overflow_amount,
        credix_lp_depository_0_available_amount,
        total_overflow_amount,
        total_available_amount,
    )?;

    // Done
    Ok(DepositoriesTargets {
        identity_depository_target_amount,
        mercurial_vault_depository_0_target_amount,
        credix_lp_depository_0_target_amount,
    })
}

/**
 * Initial depository target that doesnt take into account any hard caps
 */
fn calculate_depository_raw_target_amount(
    redeemable_circulating_supply: u64,
    depository_weight_bps: u16,
) -> Result<u64> {
    let depository_raw_target_amount = compute_amount_fraction_floor(
        redeemable_circulating_supply,
        depository_weight_bps.into(),
        BPS_UNIT_CONVERSION,
    )?;
    Ok(depository_raw_target_amount)
}

/**
 * Compute how much is the current depository overflowing compared to its hard cap
 */
fn calculate_depository_overflow_amount(
    depository_raw_target_amount: u64,
    depository_hard_cap_amount: u64,
) -> Result<u64> {
    if depository_raw_target_amount <= depository_hard_cap_amount {
        return Ok(0);
    }
    Ok(depository_raw_target_amount
        .checked_sub(depository_hard_cap_amount)
        .ok_or(UxdError::MathError)?)
}

/**
 * Compute how much extra space the depository has within its hard cap
 */
fn calculate_depository_available_amount(
    depository_raw_target_amount: u64,
    depository_hard_cap_amount: u64,
) -> Result<u64> {
    if depository_raw_target_amount >= depository_hard_cap_amount {
        return Ok(0);
    }
    Ok(depository_hard_cap_amount
        .checked_sub(depository_raw_target_amount)
        .ok_or(UxdError::MathError)?)
}

/**
 * Compute the final target amount based on circulating supply and all depository caps
 */
fn calculate_depository_target_amount(
    depository_raw_target_amount: u64,
    depository_overflow_amount: u64,
    depository_available_amount: u64,
    total_overflow_amount: u64,
    total_available_amount: u64,
) -> Result<u64> {
    let overflow_amount_reallocated_from_other_depositories: u64 = if total_available_amount > 0 {
        compute_amount_fraction_floor(
            total_overflow_amount,
            depository_available_amount,
            total_available_amount,
        )?
    } else {
        0
    };
    let final_target = depository_raw_target_amount
        .checked_sub(depository_overflow_amount)
        .ok_or(UxdError::MathError)?
        .checked_add(overflow_amount_reallocated_from_other_depositories)
        .ok_or(UxdError::MathError)?;
    Ok(final_target)
}

/**
 * Compute the sum of an amount for all known depositories
 */
fn calculate_depositories_total_amount(
    identity_depository_amount: u64,
    mercurial_vault_depository_0_amount: u64,
    credix_lp_depository_0_amount: u64,
) -> Result<u64> {
    Ok(identity_depository_amount
        .checked_add(mercurial_vault_depository_0_amount)
        .ok_or(UxdError::MathError)?
        .checked_add(credix_lp_depository_0_amount)
        .ok_or(UxdError::MathError)?)
}
