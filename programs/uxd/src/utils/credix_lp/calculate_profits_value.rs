use crate::error::UxdError;
use crate::state::CredixLpDepository;
use crate::utils::checked_convert_u128_to_u64;
use anchor_lang::prelude::AccountLoader;
use anchor_lang::prelude::Result;

pub fn calculate_profits_value(
    depository: &AccountLoader<CredixLpDepository>,
    owned_shares_value: u64,
) -> Result<u64> {
    let redeemable_amount_under_management =
        checked_convert_u128_to_u64(depository.load()?.redeemable_amount_under_management)?;
    // To find the profits we can safely withdraw
    // we find the current value of asset (the lp tokens)
    // minus the minimum liabilities (the outstanding redeemable tokens)
    Ok(owned_shares_value
        .checked_sub(redeemable_amount_under_management)
        .ok_or(UxdError::MathError)?)
}
