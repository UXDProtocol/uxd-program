use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::UxdError;
use crate::utils::calculate_depositories_sum_value;
use crate::BPS_UNIT_CONVERSION;
use crate::ROUTER_DEPOSITORIES_COUNT;

use super::checked_convert_u128_to_u64;
use super::compute_amount_fraction_ceil;

pub struct DepositoryWeightBpsAndRedeemableAmountUnderManagementCap {
    pub weight_bps: u16,
    pub redeemable_amount_under_management_cap: u128,
}

pub fn calculate_depositories_target_redeemable_amount(
    redeemable_circulating_supply: u128,
    depositories_weight_bps_and_redeemable_amount_under_management_cap: &Vec<
        DepositoryWeightBpsAndRedeemableAmountUnderManagementCap,
    >,
) -> Result<Vec<u64>> {
    require!(
        depositories_weight_bps_and_redeemable_amount_under_management_cap.len()
            == ROUTER_DEPOSITORIES_COUNT,
        UxdError::InvalidDepositoriesVector
    );

    let redeemable_circulating_supply = checked_convert_u128_to_u64(redeemable_circulating_supply)?;

    // Double check that the weights adds up to 100%
    let depositories_weights_bps =
        depositories_weight_bps_and_redeemable_amount_under_management_cap
            .iter()
            .map(|depository| u64::from(depository.weight_bps))
            .collect::<Vec<u64>>();
    let total_weight_bps = calculate_depositories_sum_value(&depositories_weights_bps)?;
    require!(
        total_weight_bps == BPS_UNIT_CONVERSION,
        UxdError::InvalidDepositoriesWeightBps,
    );

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Read the desired weights for each depository on chain
    // -- And generate a raw_target estimations that we can refine later
    // ---------------------------------------------------------------------

    let depositories_raw_target_redeemable_amount =
        depositories_weight_bps_and_redeemable_amount_under_management_cap
            .iter()
            .map(|depository| {
                calculate_depository_raw_target_redeemable_amount(
                    redeemable_circulating_supply,
                    depository.weight_bps,
                )
            })
            .collect::<Result<Vec<u64>>>()?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Using the raw_target and the depository cap:
    // -- Compute the overflow (raw target amount above the cap)
    // -- Compute the availability (raw target amount until the cap)
    // ---------------------------------------------------------------------

    // Read the minting caps of each depository
    let depositories_hard_cap_amount =
        depositories_weight_bps_and_redeemable_amount_under_management_cap
            .iter()
            .map(|depository| {
                checked_convert_u128_to_u64(depository.redeemable_amount_under_management_cap)
            })
            .collect::<Result<Vec<u64>>>()?;

    // Compute the depository_overflow amount of raw target that doesn't fit within the cap of each depository
    let depositories_overflow_amount = std::iter::zip(
        depositories_raw_target_redeemable_amount.iter(),
        depositories_hard_cap_amount.iter(),
    )
    .map(
        |(depository_raw_target_redeemable_amount, depository_hard_cap_amount)| {
            calculate_depository_overflow_amount(
                *depository_raw_target_redeemable_amount,
                *depository_hard_cap_amount,
            )
        },
    )
    .collect::<Result<Vec<u64>>>()?;

    // Compute the amount of space available under the cap in each depository
    let depositories_available_amount = std::iter::zip(
        depositories_raw_target_redeemable_amount.iter(),
        depositories_hard_cap_amount.iter(),
    )
    .map(
        |(depository_raw_target_redeemable_amount, depository_hard_cap_amount)| {
            calculate_depository_available_amount(
                *depository_raw_target_redeemable_amount,
                *depository_hard_cap_amount,
            )
        },
    )
    .collect::<Result<Vec<u64>>>()?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Compute the combined overflow of all depositories
    // -- Compute the combined availability of all depositories
    // ---------------------------------------------------------------------

    // Compute total amount that doesn't fit within depositories hard cap
    let total_overflow_amount = calculate_depositories_sum_value(&depositories_overflow_amount)?;
    // Compute total amount that doesn't fit within depositories hard cap
    let total_available_amount = calculate_depositories_sum_value(&depositories_available_amount)?;

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
    let depositories_target_redeemable_amount = std::iter::zip(
        depositories_raw_target_redeemable_amount.iter(),
        std::iter::zip(
            depositories_overflow_amount.iter(),
            depositories_available_amount.iter(),
        ),
    )
    .map(
        |(
            depository_raw_target_redeemable_amount,
            (depository_overflow_amount, depository_available_amount),
        )| {
            calculate_depository_target_redeemable_amount(
                *depository_raw_target_redeemable_amount,
                *depository_overflow_amount,
                *depository_available_amount,
                total_overflow_amount,
                total_available_amount,
            )
        },
    )
    .collect::<Result<Vec<u64>>>()?;

    // Done
    Ok(depositories_target_redeemable_amount)
}

/**
 * Initial depository target that doesnt take into account any hard caps
 */
fn calculate_depository_raw_target_redeemable_amount(
    redeemable_circulating_supply: u64,
    depository_weight_bps: u16,
) -> Result<u64> {
    let depository_raw_target_redeemable_amount = compute_amount_fraction_ceil(
        redeemable_circulating_supply,
        depository_weight_bps.into(),
        BPS_UNIT_CONVERSION,
    )?;
    Ok(depository_raw_target_redeemable_amount)
}

/**
 * Compute how much is the current depository overflowing compared to its hard cap
 */
fn calculate_depository_overflow_amount(
    depository_raw_target_redeemable_amount: u64,
    depository_hard_cap_amount: u64,
) -> Result<u64> {
    if depository_raw_target_redeemable_amount <= depository_hard_cap_amount {
        return Ok(0);
    }
    Ok(depository_raw_target_redeemable_amount
        .checked_sub(depository_hard_cap_amount)
        .ok_or(UxdError::MathError)?)
}

/**
 * Compute how much extra space the depository has within its hard cap
 */
fn calculate_depository_available_amount(
    depository_raw_target_redeemable_amount: u64,
    depository_hard_cap_amount: u64,
) -> Result<u64> {
    if depository_raw_target_redeemable_amount >= depository_hard_cap_amount {
        return Ok(0);
    }
    Ok(depository_hard_cap_amount
        .checked_sub(depository_raw_target_redeemable_amount)
        .ok_or(UxdError::MathError)?)
}

/**
 * Compute the final target amount based on circulating supply and all depository caps
 */
fn calculate_depository_target_redeemable_amount(
    depository_raw_target_redeemable_amount: u64,
    depository_overflow_amount: u64,
    depository_available_amount: u64,
    total_overflow_amount: u64,
    total_available_amount: u64,
) -> Result<u64> {
    let overflow_amount_reallocated_from_other_depositories: u64 = if total_available_amount > 0 {
        // We try to rellocate up to the maximum available total amount.
        // If the overflow amount is more than the available amount, there is nothing we can do
        let total_amount_reallocatable =
            std::cmp::min(total_overflow_amount, total_available_amount);
        compute_amount_fraction_ceil(
            total_amount_reallocatable,
            depository_available_amount,
            total_available_amount,
        )?
    } else {
        0
    };
    let final_target = depository_raw_target_redeemable_amount
        .checked_add(overflow_amount_reallocated_from_other_depositories)
        .ok_or(UxdError::MathError)?
        .checked_sub(depository_overflow_amount)
        .ok_or(UxdError::MathError)?;
    Ok(final_target)
}
