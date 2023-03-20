use crate::error::UxdError;
use crate::utils::checked_convert_u128_to_u64;
use anchor_lang::prelude::Result;

pub fn calculate_profits_collateral_amount(
    redeemable_amount_under_management: u128,
    owned_shares_value_before: u64,
) -> Result<u64> {
    // We assume that liabilities in redeemable are equivalent 1:1 to the same amount in collateral
    let liabilities_collateral_amount =
        checked_convert_u128_to_u64(redeemable_amount_under_management)?;
    // Compute the set of assets owned in the LP in collateral amount
    let assets_collateral_amount = owned_shares_value_before;
    // Compute the amount of profits that we can safely withdraw
    Ok(assets_collateral_amount
        .checked_sub(liabilities_collateral_amount)
        .ok_or(UxdError::MathError)?)
}
