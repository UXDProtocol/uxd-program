use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::UxdError;
use crate::utils::calculate_depositories_sum_value;
use crate::ROUTER_DEPOSITORIES_COUNT;

use super::compute_amount_less_fraction_floor;

pub struct DepositoryInfoForMintCollateralAmount {
    pub directly_mintable: bool,
    pub target_redeemable_amount: u64,
    pub redeemable_amount_under_management: u64,
    pub redeemable_amount_under_management_cap: u64,
}

pub fn calculate_depositories_mint_collateral_amount(
    requested_collateral_amount: u64,
    depositories_info: &Vec<DepositoryInfoForMintCollateralAmount>,
) -> Result<Vec<u64>> {
    require!(
        depositories_info.len() == ROUTER_DEPOSITORIES_COUNT,
        UxdError::InvalidDepositoriesVector
    );

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Calculate the under_target redeemable amount for each depository
    // -- This amount is what we can mint into first, so that all depositories remain balanced
    // ---------------------------------------------------------------------

    let depositories_under_target_redeemable_amount = depositories_info
        .iter()
        .map(|depository| {
            if !depository.directly_mintable {
                return Ok(0);
            }
            if depository.target_redeemable_amount <= depository.redeemable_amount_under_management
            {
                return Ok(0);
            }
            Ok(depository
                .target_redeemable_amount
                .checked_sub(depository.redeemable_amount_under_management)
                .ok_or(UxdError::MathOverflow)?)
        })
        .collect::<Result<Vec<u64>>>()?;

    let total_under_target_redeemable_amount =
        calculate_depositories_sum_value(&depositories_under_target_redeemable_amount)?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Calculate the under cap redeemable amount for each depository
    // -- This is the amount of redeemable past the target and under the hard-cap of the depository
    // -- This amount will be used as a last-ditch effort to fullfull the mint
    // -- In case all mintable depositories are filled up to their target
    // ---------------------------------------------------------------------

    let depositories_under_cap_redeemable_amount = depositories_info
        .iter()
        .map(|depository| {
            if !depository.directly_mintable {
                return Ok(0);
            }
            if depository.redeemable_amount_under_management_cap
                <= depository.target_redeemable_amount
            {
                return Ok(0);
            }
            Ok(depository
                .redeemable_amount_under_management_cap
                .checked_sub(depository.target_redeemable_amount)
                .ok_or(UxdError::MathOverflow)?)
        })
        .collect::<Result<Vec<u64>>>()?;

    let total_under_cap_redeemable_amount =
        calculate_depositories_sum_value(&depositories_under_cap_redeemable_amount)?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Check that we have enough available space in our depositories
    // -- To be able to fullfill the user's mint requested amount
    // ---------------------------------------------------------------------

    let total_overall_mintable_amount = total_under_target_redeemable_amount
        .checked_add(total_under_cap_redeemable_amount)
        .ok_or(UxdError::MathOverflow)?;
    require!(
        total_overall_mintable_amount >= requested_collateral_amount,
        UxdError::DepositoriesTargerRedeemableAmountReached
    );

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Compute the final amounts by:
    // -- trying as a best-effort to keep the balance (step1)
    // -- And when keeping the perfect balance is not possible,
    // -- try to consume the under_cap amount fairly (step2)
    // ---------------------------------------------------------------------

    let depositories_mint_collateral_amount = std::iter::zip(
        depositories_under_target_redeemable_amount.iter(),
        depositories_under_cap_redeemable_amount.iter(),
    )
    .map(
        |(depository_under_target_redeemable_amount, depository_under_cap_redeemable_amount)| {
            // Step 1, try to mint any under_target amount to fill the depository to its target
            let requested_primary_collateral_amount = std::cmp::min(
                requested_collateral_amount,
                total_under_target_redeemable_amount,
            );
            let depository_primary_collateral_amount = if total_under_target_redeemable_amount > 0 {
                let other_depositories_under_target_redeemable_amount =
                    total_under_target_redeemable_amount
                        .checked_sub(*depository_under_target_redeemable_amount)
                        .ok_or(UxdError::MathOverflow)?;
                compute_amount_less_fraction_floor(
                    requested_primary_collateral_amount,
                    other_depositories_under_target_redeemable_amount,
                    total_under_target_redeemable_amount,
                )?
            } else {
                0
            };
            // Step 2, when all depositories are to their targets, try to fill up to their caps fairly
            let requested_backup_collateral_amount = requested_collateral_amount
                .checked_sub(requested_primary_collateral_amount)
                .ok_or(UxdError::MathOverflow)?;
            let depository_backup_collateral_amount = if total_under_cap_redeemable_amount > 0 {
                let other_depositories_under_cap_redeemable_amount =
                    total_under_cap_redeemable_amount
                        .checked_sub(*depository_under_cap_redeemable_amount)
                        .ok_or(UxdError::MathOverflow)?;
                compute_amount_less_fraction_floor(
                    requested_backup_collateral_amount,
                    other_depositories_under_cap_redeemable_amount,
                    total_under_cap_redeemable_amount,
                )?
            } else {
                0
            };
            // The combo of the two gives our depository amount
            Ok(depository_primary_collateral_amount
                .checked_add(depository_backup_collateral_amount)
                .ok_or(UxdError::MathOverflow)?)
        },
    )
    .collect::<Result<Vec<u64>>>()?;

    // Done
    Ok(depositories_mint_collateral_amount)
}
