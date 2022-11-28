use crate::error::UxdError;
use anchor_lang::prelude::*;

// Precision loss may lower the returned value amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_value_for_shares_amount(
    shares_amount: u64,
    total_shares_amount: u64,
    total_shares_value: u64,
) -> Result<u64> {
    if shares_amount == 0 {
        return Ok(0);
    }
    require!(shares_amount <= total_shares_amount, UxdError::MathError);
    Ok(shares_amount
        .checked_mul(total_shares_value)
        .ok_or(UxdError::MathError)?
        .checked_div(total_shares_amount)
        .ok_or(UxdError::MathError)?)
}
