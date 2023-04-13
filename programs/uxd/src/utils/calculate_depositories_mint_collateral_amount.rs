use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::UxdError;
use crate::utils::calculate_depositories_sum_value;
use crate::ROUTER_DEPOSITORIES_COUNT;

use super::checked_convert_u128_to_u64;
use super::compute_amount_less_fraction_floor;

pub struct DepositoryInfoForMintCollateralAmount {
    pub target_redeemable_amount: u64,
    pub redeemable_amount_under_management: u128,
}

pub fn calculate_depositories_mint_collateral_amount(
    requested_mint_collateral_amount: u64,
    depositories_info: &Vec<DepositoryInfoForMintCollateralAmount>,
) -> Result<Vec<u64>> {
    require!(
        depositories_info.len() == ROUTER_DEPOSITORIES_COUNT,
        UxdError::InvalidDepositoriesVector
    );

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Calculate the maximum mintable collateral amount for each depository
    // ---------------------------------------------------------------------

    let depositories_mintable_collateral_amount = depositories_info
        .iter()
        .map(|depository| {
            let depository_redeemable_amount_under_management =
                checked_convert_u128_to_u64(depository.redeemable_amount_under_management)?;
            if depository.target_redeemable_amount <= depository_redeemable_amount_under_management
            {
                return Ok(0);
            }
            Ok(depository
                .target_redeemable_amount
                .checked_sub(depository_redeemable_amount_under_management)
                .ok_or(UxdError::MathError)?)
        })
        .collect::<Result<Vec<u64>>>()?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Calculate the total amount we could possibly mint
    // -- If this total is not enough, we abort
    // ---------------------------------------------------------------------

    let total_mintable_collateral_amount =
        calculate_depositories_sum_value(&depositories_mintable_collateral_amount)?;
    require!(
        total_mintable_collateral_amount >= requested_mint_collateral_amount,
        UxdError::DepositoriesTargerRedeemableAmountReached
    );

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Calculate the actual minted amount per depository for the requested mint amount,
    // -- it is a weighted slice of the total mintable amount, scaled by the requested mint amount
    // ---------------------------------------------------------------------

    let depositories_mint_collateral_amount = depositories_mintable_collateral_amount
        .iter()
        .map(|depository_mintable_collateral_amount| {
            let other_depositories_mintable_collateral_amount = total_mintable_collateral_amount
                .checked_sub(*depository_mintable_collateral_amount)
                .ok_or(UxdError::MathError)?;
            compute_amount_less_fraction_floor(
                requested_mint_collateral_amount,
                other_depositories_mintable_collateral_amount,
                total_mintable_collateral_amount,
            )
        })
        .collect::<Result<Vec<u64>>>()?;

    // Done
    Ok(depositories_mint_collateral_amount)
}
