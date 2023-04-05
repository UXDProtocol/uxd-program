use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::UxdError;
use crate::utils::calculate_depositories_sum_value;

use super::checked_convert_u128_to_u64;
use super::compute_amount_less_fraction_floor;

pub struct DepositoriesRedeemRedeemableAmount {
    pub identity_depository_redeem_redeemable_amount: u64,
    pub mercurial_vault_depository_0_redeem_redeemable_amount: u64,
}

pub fn calculate_depositories_redeem_redeemable_amount(
    input_redeem_redeemable_amount: u64,
    identity_depository_target_redeemable_amount: u64,
    mercurial_vault_depository_0_target_redeemable_amount: u64,
    identity_depository_redeemable_amount_under_management: u128,
    mercurial_vault_depository_0_redeemable_amount_under_management: u128,
) -> Result<DepositoriesRedeemRedeemableAmount> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Calculate the ideal redeem redeemable amount for each depository
    // -- The ideal amount is how much we need to redeem to fully rebalance the depository
    // ---------------------------------------------------------------------

    let identity_depository_ideal_redeemable_amount = calculate_depository_ideal_redeemable_amount(
        identity_depository_redeemable_amount_under_management,
        identity_depository_target_redeemable_amount,
    )?;
    let mercurial_vault_depository_0_ideal_redeemable_amount =
        calculate_depository_ideal_redeemable_amount(
            mercurial_vault_depository_0_redeemable_amount_under_management,
            mercurial_vault_depository_0_target_redeemable_amount,
        )?;
    let credix_lp_depository_0_ideal_redeemable_amount = 0; // credix is not liquid

    let total_ideal_redeemable_amount = calculate_depositories_sum_value(
        identity_depository_ideal_redeemable_amount,
        mercurial_vault_depository_0_ideal_redeemable_amount,
        credix_lp_depository_0_ideal_redeemable_amount,
    )?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Calculate the maximum possible redeemable amount for each depository
    // -- We wont be able to redeem past this amount as the depositories would be empty
    // ---------------------------------------------------------------------

    let identity_depository_maximum_redeemable_amount =
        checked_convert_u128_to_u64(identity_depository_redeemable_amount_under_management)?;
    let mercurial_vault_depository_0_maximum_redeemable_amount = checked_convert_u128_to_u64(
        mercurial_vault_depository_0_redeemable_amount_under_management,
    )?;
    let credix_lp_depository_0_maximum_redeemable_amount = 0; // credix is not liquid

    let total_maximum_redeemable_amount = calculate_depositories_sum_value(
        identity_depository_maximum_redeemable_amount,
        mercurial_vault_depository_0_maximum_redeemable_amount,
        credix_lp_depository_0_maximum_redeemable_amount,
    )?;

    require!(
        total_maximum_redeemable_amount >= input_redeem_redeemable_amount,
        UxdError::InvalidRedeemableAmount
    );

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Compute the final amounts by trying as a best-effort to keep the balance
    // -- And when keeping the balance is not possible, try to consume the remaining
    // ---------------------------------------------------------------------

    let identity_depository_redeem_redeemable_amount =
        calculate_depository_redeem_redeemable_amount(
            input_redeem_redeemable_amount,
            identity_depository_ideal_redeemable_amount,
            identity_depository_maximum_redeemable_amount,
            total_ideal_redeemable_amount,
            total_maximum_redeemable_amount,
        )?;
    let mercurial_vault_depository_0_redeem_redeemable_amount =
        calculate_depository_redeem_redeemable_amount(
            input_redeem_redeemable_amount,
            mercurial_vault_depository_0_ideal_redeemable_amount,
            mercurial_vault_depository_0_maximum_redeemable_amount,
            total_ideal_redeemable_amount,
            total_maximum_redeemable_amount,
        )?;

    // Done
    Ok(DepositoriesRedeemRedeemableAmount {
        identity_depository_redeem_redeemable_amount,
        mercurial_vault_depository_0_redeem_redeemable_amount,
    })
}

/**
 * Compute how much we ideally can redeem to fulfil balancing requirements
 */
fn calculate_depository_ideal_redeemable_amount(
    depository_redeemable_amount_under_management: u128,
    depository_target_redeemable_amount: u64,
) -> Result<u64> {
    let depository_redeemable_amount_under_management =
        checked_convert_u128_to_u64(depository_redeemable_amount_under_management)?;
    if depository_redeemable_amount_under_management <= depository_target_redeemable_amount {
        return Ok(0);
    }
    Ok(depository_redeemable_amount_under_management
        .checked_sub(depository_target_redeemable_amount)
        .ok_or(UxdError::MathError)?)
}

/**
 * Compute the final redeemed amount.
 * There is 2 steps:
 * - 1) first we try to use the ideal amounts to improve blaancing
 * - 2) if the ideal amount is not enough, we have to tap into the rest and create an imbalance
 */
fn calculate_depository_redeem_redeemable_amount(
    input_redeem_redeemable_amount: u64,
    depository_ideal_redeemable_amount: u64,
    depository_maximum_redeemable_amount: u64,
    total_ideal_redeemable_amount: u64,
    total_maximum_redeemable_amount: u64,
) -> Result<u64> {
    // First phase, try to use the ideal amounts, weighted for each depository
    let total_first_redeem_redeemable_amount = std::cmp::min(
        input_redeem_redeemable_amount,
        total_ideal_redeemable_amount,
    );
    let other_depositories_ideal_redeemable_amount = total_ideal_redeemable_amount
        .checked_sub(depository_ideal_redeemable_amount)
        .ok_or(UxdError::MathError)?;
    let depository_first_redeem_redeemable_amount = compute_amount_less_fraction_floor(
        total_first_redeem_redeemable_amount,
        other_depositories_ideal_redeemable_amount,
        total_ideal_redeemable_amount,
    )?;

    // Second step, anything remaining must be taken from the rest to create an unbalance
    let total_second_redeem_redeemable_amount = input_redeem_redeemable_amount
        .checked_sub(total_first_redeem_redeemable_amount)
        .ok_or(UxdError::MathError)?;
    let other_depositories_maximum_redeemable_amount = total_maximum_redeemable_amount
        .checked_sub(depository_maximum_redeemable_amount)
        .ok_or(UxdError::MathError)?;
    let depository_second_redeem_redeemable_amount = compute_amount_less_fraction_floor(
        total_second_redeem_redeemable_amount,
        other_depositories_maximum_redeemable_amount,
        total_maximum_redeemable_amount,
    )?;

    // The combo of the two gives our depository amount
    Ok(depository_first_redeem_redeemable_amount
        .checked_add(depository_second_redeem_redeemable_amount)
        .ok_or(UxdError::MathError)?)
}
