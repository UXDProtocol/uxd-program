use crate::error::UxdError;
use crate::state::CredixLpDepository;
use crate::utils::checked_convert_u128_to_u64;
use anchor_lang::prelude::AccountLoader;
use anchor_lang::prelude::Result;

pub fn calculate_profits_collateral_amount(
    depository: &AccountLoader<CredixLpDepository>,
    owned_shares_value_before: u64,
) -> Result<u64> {
    // Read the redeemable amount that is supposed to be in this depository
    let redeemable_amount_under_management =
        checked_convert_u128_to_u64(depository.load()?.redeemable_amount_under_management)?;
    // For each redeemable amount, we must have at least the same amount of collateral in the depository
    let liabilities_collateral_amount = redeemable_amount_under_management;
    // The depository's owned LP value may have grown over time to be worth more collateral then we need
    let assets_collateral_amount = owned_shares_value_before;
    // To find the profits we can safely withdraw, we find the current value of asset minus the minimum liabilities
    Ok(assets_collateral_amount
        .checked_sub(liabilities_collateral_amount)
        .ok_or(UxdError::MathError)?)
}
