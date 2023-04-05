use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::UxdError;
use crate::utils::calculate_depositories_sum_value;

use super::checked_convert_u128_to_u64;
use super::compute_amount_less_fraction_floor;
use super::DepositoriesTargetRedeemableAmount;

pub struct DepositoriesMintCollateralAmount {
    pub identity_depository_mint_collateral_amount: u64,
    pub mercurial_vault_depository_0_mint_collateral_amount: u64,
    pub credix_lp_depository_0_mint_collateral_amount: u64,
}

pub fn calculate_depositories_mint_collateral_amount(
    input_mint_collateral_amount: u64,
    depositories_target_redeemable_amount: &DepositoriesTargetRedeemableAmount,
    identity_depository_redeemable_amount_under_management: u128,
    mercurial_vault_depository_0_redeemable_amount_under_management: u128,
    credix_lp_depository_0_redeemable_amount_under_management: u128,
) -> Result<DepositoriesMintCollateralAmount> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Calculate the maximum mintable collateral amount for each depository
    // ---------------------------------------------------------------------

    let identity_depository_mintable_collateral_amount =
        calculate_depository_mintable_collateral_amount(
            identity_depository_redeemable_amount_under_management,
            depositories_target_redeemable_amount.identity_depository_target_redeemable_amount,
        )?;
    let mercurial_vault_depository_0_mintable_collateral_amount =
        calculate_depository_mintable_collateral_amount(
            mercurial_vault_depository_0_redeemable_amount_under_management,
            depositories_target_redeemable_amount
                .mercurial_vault_depository_0_target_redeemable_amount,
        )?;
    let credix_lp_depository_0_mintable_collateral_amount =
        calculate_depository_mintable_collateral_amount(
            credix_lp_depository_0_redeemable_amount_under_management,
            depositories_target_redeemable_amount.credix_lp_depository_0_target_redeemable_amount,
        )?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Calculate the total amount we could possibly mint
    // -- If this total is not enough, we abort
    // ---------------------------------------------------------------------

    let total_mintable_collateral_amount = calculate_depositories_sum_value(
        identity_depository_mintable_collateral_amount,
        mercurial_vault_depository_0_mintable_collateral_amount,
        credix_lp_depository_0_mintable_collateral_amount,
    )?;
    require!(
        total_mintable_collateral_amount >= input_mint_collateral_amount,
        UxdError::RedeemableGlobalSupplyCapReached
    );

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Calculate the actual minted amount,
    // -- it is a weighted slice of the total mintable amount
    // ---------------------------------------------------------------------

    let identity_depository_mint_collateral_amount = calculate_depository_mint_collateral_amount(
        input_mint_collateral_amount,
        identity_depository_mintable_collateral_amount,
        total_mintable_collateral_amount,
    )?;
    let mercurial_vault_depository_0_mint_collateral_amount =
        calculate_depository_mint_collateral_amount(
            input_mint_collateral_amount,
            mercurial_vault_depository_0_mintable_collateral_amount,
            total_mintable_collateral_amount,
        )?;
    let credix_lp_depository_0_mint_collateral_amount =
        calculate_depository_mint_collateral_amount(
            input_mint_collateral_amount,
            credix_lp_depository_0_mintable_collateral_amount,
            total_mintable_collateral_amount,
        )?;

    // Done
    Ok(DepositoriesMintCollateralAmount {
        identity_depository_mint_collateral_amount,
        mercurial_vault_depository_0_mint_collateral_amount,
        credix_lp_depository_0_mint_collateral_amount,
    })
}

/**
 * Compute how much we can mint before we go over the depository's target
 */
fn calculate_depository_mintable_collateral_amount(
    depository_redeemable_amount_under_management: u128,
    depository_target_redeemable_amount: u64,
) -> Result<u64> {
    let depository_redeemable_amount_under_management =
        checked_convert_u128_to_u64(depository_redeemable_amount_under_management)?;
    if depository_target_redeemable_amount <= depository_redeemable_amount_under_management {
        return Ok(0);
    }
    Ok(depository_target_redeemable_amount
        .checked_sub(depository_redeemable_amount_under_management)
        .ok_or(UxdError::MathError)?)
}

/**
 * Compute the fraction of the total_mint_collateral_amount that can be mint in this depository
 */
fn calculate_depository_mint_collateral_amount(
    total_mint_collateral_amount: u64,
    depository_mintable_collateral_amount: u64,
    total_mintable_collateral_amount: u64,
) -> Result<u64> {
    let other_depositories_mintable_collateral_amount = total_mintable_collateral_amount
        .checked_sub(depository_mintable_collateral_amount)
        .ok_or(UxdError::MathError)?;
    Ok(compute_amount_less_fraction_floor(
        total_mint_collateral_amount,
        other_depositories_mintable_collateral_amount,
        total_mintable_collateral_amount,
    )?)
}
