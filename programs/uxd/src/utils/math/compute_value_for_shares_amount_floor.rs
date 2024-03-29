use crate::error::UxdError;
use anchor_lang::prelude::*;

// Precision loss may lower the returned value amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_value_for_shares_amount_floor(
    shares_amount: u64,
    total_shares_supply: u64,
    total_shares_value: u64,
) -> Result<u64> {
    if shares_amount == 0 {
        return Ok(0);
    }
    require!(total_shares_supply > 0, UxdError::MathOverflow);
    require!(total_shares_value > 0, UxdError::MathOverflow);
    let shares_amount: u128 = shares_amount.into();
    let total_shares_supply: u128 = total_shares_supply.into();
    let total_shares_value: u128 = total_shares_value.into();
    let value: u128 = shares_amount
        .checked_mul(total_shares_value)
        .ok_or(UxdError::MathOverflow)?
        .checked_div(total_shares_supply)
        .ok_or(UxdError::MathOverflow)?;
    Ok(u64::try_from(value).ok().ok_or(UxdError::MathOverflow)?)
}
