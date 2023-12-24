use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::UxdError;
use crate::utils::calculate_depositories_sum_value;
use crate::utils::checked_add;
use crate::utils::checked_sub;
use crate::ROUTER_DEPOSITORIES_COUNT;

use super::compute_amount_less_fraction_floor;

pub struct DepositoryInfoForRedeemableAmount {
    pub directly_redeemable: bool,
    pub target_redeemable_amount: u64,
    pub redeemable_amount_under_management: u64,
}

pub fn calculate_depositories_redeemable_amount(
    requested_redeemable_amount: u64,
    depositories_info: &Vec<DepositoryInfoForRedeemableAmount>,
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
            if !depository.directly_redeemable {
                return Ok(0);
            }
            if depository.redeemable_amount_under_management <= depository.target_redeemable_amount
            {
                return Ok(0);
            }
            checked_sub(
                depository.redeemable_amount_under_management,
                depository.target_redeemable_amount,
            )
        })
        .collect::<Result<Vec<u64>>>()?;

    let total_over_target_redeemable_amount =
        calculate_depositories_sum_value(&depositories_over_target_redeemable_amount)?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Calculate the under_target redeemable amount for each depository
    // -- This amount is what remains redeemable after we redeemed the over_target amount
    // -- This amount will be used as a last-ditch effort to fullfill the redeem when needed
    // ---------------------------------------------------------------------

    let depositories_under_target_redeemable_amount = depositories_info
        .iter()
        .map(|depository| {
            if !depository.directly_redeemable {
                return Ok(0);
            }
            Ok(std::cmp::min(
                depository.redeemable_amount_under_management,
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

    let total_overall_redeemable_amount = checked_add(
        total_over_target_redeemable_amount,
        total_under_target_redeemable_amount,
    )?;
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
            // Step 1, try to use the over_target amounts, weighted for each depository
            let requested_primary_redeemable_amount = std::cmp::min(
                requested_redeemable_amount,
                total_over_target_redeemable_amount,
            );
            let depository_primary_redeemable_amount = if total_over_target_redeemable_amount > 0 {
                let other_depositories_over_target_redeemable_amount = checked_sub(
                    total_over_target_redeemable_amount,
                    *depository_over_target_redeemable_amount,
                )?;
                compute_amount_less_fraction_floor(
                    requested_primary_redeemable_amount,
                    other_depositories_over_target_redeemable_amount,
                    total_over_target_redeemable_amount,
                )?
            } else {
                0
            };
            // Step 2, anything under_target must be used as backup
            let requested_backup_redeemable_amount = checked_sub(
                requested_redeemable_amount,
                requested_primary_redeemable_amount,
            )?;
            let depository_backup_redeemable_amount = if total_under_target_redeemable_amount > 0 {
                let other_depositories_under_target_redeemable_amount = checked_sub(
                    total_under_target_redeemable_amount,
                    *depository_under_target_redeemable_amount,
                )?;
                compute_amount_less_fraction_floor(
                    requested_backup_redeemable_amount,
                    other_depositories_under_target_redeemable_amount,
                    total_under_target_redeemable_amount,
                )?
            } else {
                0
            };
            // The combo of the two gives our depository amount
            checked_add(
                depository_primary_redeemable_amount,
                depository_backup_redeemable_amount,
            )
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

    let mut rounding_errors = checked_sub(requested_redeemable_amount, total_redeemable_amount)?;

    for i in 0..depositories_info.len() {
        let depository = &depositories_info[i];
        if !depository.directly_redeemable {
            continue;
        }
        let depository_remaining_after_redeem = checked_sub(
            depository.redeemable_amount_under_management,
            depositories_redeemable_amount[i],
        )?;
        let depository_rounding_correction =
            std::cmp::min(depository_remaining_after_redeem, rounding_errors);
        depositories_redeemable_amount[i] = checked_add(
            depositories_redeemable_amount[i],
            depository_rounding_correction,
        )?;
        rounding_errors = checked_sub(rounding_errors, depository_rounding_correction)?;
    }

    // Done
    Ok(depositories_redeemable_amount)
}
