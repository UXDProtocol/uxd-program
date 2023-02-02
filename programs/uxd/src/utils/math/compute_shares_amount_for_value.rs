use crate::{error::UxdError, utils::checked_u128_to_u64};
use anchor_lang::prelude::*;

// Precision loss may lower the returned shares amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_shares_amount_for_value(
    value: u64,
    total_shares_supply: u64,
    total_shares_value: u64,
) -> Result<u64> {
    if value == 0 {
        return Ok(0);
    }
    require!(total_shares_supply > 0, UxdError::MathError);
    require!(total_shares_value > 0, UxdError::MathError);
    let value: u128 = value.into();
    let total_shares_supply: u128 = total_shares_supply.into();
    let total_shares_value: u128 = total_shares_value.into();
    let shares_amount: u128 = value
        .checked_mul(total_shares_supply)
        .ok_or(UxdError::MathError)?
        .checked_div(total_shares_value)
        .ok_or(UxdError::MathError)?;
    checked_u128_to_u64(shares_amount)
}
