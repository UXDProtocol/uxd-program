use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::UxdError;
use crate::utils::calculate_depositories_sum_value;
use crate::utils::checked_as_u64;
use crate::ROUTER_DEPOSITORIES_COUNT;

use super::compute_amount_less_fraction_floor;

pub struct DepositoryInfoForRedeemableAmount {
    pub is_liquid: bool,
    pub target_redeemable_amount: u64,
    pub redeemable_amount_under_management: u128,
}

pub fn calculate_depositories_redeemable_amount(
    requested_redeemable_amount: u64,
    depositories_info: &[DepositoryInfoForRedeemableAmount],
) -> Result<Vec<u64>> {
    require!(
        depositories_info.len() == ROUTER_DEPOSITORIES_COUNT,
        UxdError::InvalidDepositoriesVector
    );

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Calculate the over_target redeem redeemable amount for each depository
    // -- The over_target amount is how much we need to redeem to fully rebalance the depository
    // -- It is equivalent to the anount of collateral above the target
    // ---------------------------------------------------------------------

    let depositories_over_target_redeemable_amount = depositories_info
        .iter()
        .map(|depository| {
            if !depository.is_liquid {
                return Ok(0);
            }
            let depository_redeemable_amount_under_management =
                checked_as_u64(depository.redeemable_amount_under_management)?;
            if depository_redeemable_amount_under_management <= depository.target_redeemable_amount
            {
                return Ok(0);
            }
            Ok(depository_redeemable_amount_under_management
                .checked_sub(depository.target_redeemable_amount)
                .ok_or(UxdError::MathOverflow)?)
        })
        .collect::<Result<Vec<u64>>>()?;

    let total_over_target_redeemable_amount =
        calculate_depositories_sum_value(&depositories_over_target_redeemable_amount)?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Calculate the under_target redeemable amount for each depository
    // -- This amount is what remains redeemable after we redeemed the over_target amount
    // -- This amount will be used as a last-ditch effort to fullfull the redeem when needed
    // ---------------------------------------------------------------------

    let depositories_under_target_redeemable_amount = depositories_info
        .iter()
        .map(|depository| {
            if !depository.is_liquid {
                return Ok(0);
            }
            let depository_redeemable_amount_under_management =
                checked_as_u64(depository.redeemable_amount_under_management)?;
            Ok(std::cmp::min(
                depository_redeemable_amount_under_management,
                depository.target_redeemable_amount,
            ))
        })
        .collect::<Result<Vec<u64>>>()?;

    let total_under_target_redeemable_amount =
        calculate_depositories_sum_value(&depositories_under_target_redeemable_amount)?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Check that we have enough redeemable accross all our available methods
    // -- to be able to fullfill the user's redeemable requested amount
    // ---------------------------------------------------------------------

    let total_overall_redeemable_amount = total_over_target_redeemable_amount
        .checked_add(total_under_target_redeemable_amount)
        .ok_or(UxdError::MathOverflow)?;
    require!(
        total_overall_redeemable_amount >= requested_redeemable_amount,
        UxdError::InvalidRedeemableAmount
    );

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Compute the final amounts by:
    // -- trying as a best-effort to keep the balance (step1)
    // -- And when keeping the perfect balance is not possible,
    // -- try to consume the under_target redeemable amount fairly (step2)
    // ---------------------------------------------------------------------

    let mut depositories_redeemable_amount = std::iter::zip(
        depositories_over_target_redeemable_amount.iter(),
        depositories_under_target_redeemable_amount.iter(),
    )
    .map(
        |(depository_over_target_redeemable_amount, depository_under_target_redeemable_amount)| {
            // Total possible redeemable amounts for both steps
            let requested_first_redeemable_amount = std::cmp::min(
                requested_redeemable_amount,
                total_over_target_redeemable_amount,
            );
            let requested_second_redeemable_amount = requested_redeemable_amount
                .checked_sub(requested_first_redeemable_amount)
                .ok_or(UxdError::MathOverflow)?;
            // First step, try to use the over_target amounts, weighted for each depository
            let depository_first_redeemable_amount = if total_over_target_redeemable_amount > 0 {
                let other_depositories_over_target_redeemable_amount =
                    total_over_target_redeemable_amount
                        .checked_sub(*depository_over_target_redeemable_amount)
                        .ok_or(UxdError::MathOverflow)?;
                compute_amount_less_fraction_floor(
                    requested_first_redeemable_amount,
                    other_depositories_over_target_redeemable_amount,
                    total_over_target_redeemable_amount,
                )?
            } else {
                0
            };
            // Second step, anything under_target must be taken as backup
            let depository_second_redeemable_amount = if total_under_target_redeemable_amount > 0 {
                let other_depositories_under_target_redeemable_amount =
                    total_under_target_redeemable_amount
                        .checked_sub(*depository_under_target_redeemable_amount)
                        .ok_or(UxdError::MathOverflow)?;
                compute_amount_less_fraction_floor(
                    requested_second_redeemable_amount,
                    other_depositories_under_target_redeemable_amount,
                    total_under_target_redeemable_amount,
                )?
            } else {
                0
            };
            // The combo of the two gives our depository amount
            Ok(depository_first_redeemable_amount
                .checked_add(depository_second_redeemable_amount)
                .ok_or(UxdError::MathOverflow)?)
        },
    )
    .collect::<Result<Vec<u64>>>()?;

    // ---------------------------------------------------------------------
    // -- Phase 5
    // -- Correct for precision loss rounding errors
    // -- We compute the difference between the requested amount and the total computed amount
    // -- And add any difference back to the first depository with remaining redeemable
    // ---------------------------------------------------------------------

    let total_redeemable_amount =
        calculate_depositories_sum_value(&depositories_redeemable_amount)?;

    let mut rounding_errors = requested_redeemable_amount
        .checked_sub(total_redeemable_amount)
        .ok_or(UxdError::MathOverflow)?;

    for i in 0..depositories_info.len() {
        let depository = &depositories_info[i];
        if !depository.is_liquid {
            continue;
        }
        let depository_remaining_after_redeem =
            checked_as_u64(depository.redeemable_amount_under_management)?
                .checked_sub(depositories_redeemable_amount[i])
                .ok_or(UxdError::MathOverflow)?;
        let depository_rounding_correction =
            std::cmp::min(depository_remaining_after_redeem, rounding_errors);
        depositories_redeemable_amount[i] = depositories_redeemable_amount[i]
            .checked_add(depository_rounding_correction)
            .ok_or(UxdError::MathOverflow)?;
        rounding_errors = rounding_errors
            .checked_sub(depository_rounding_correction)
            .ok_or(UxdError::MathOverflow)?;
    }

    // Done
    Ok(depositories_redeemable_amount)
}
