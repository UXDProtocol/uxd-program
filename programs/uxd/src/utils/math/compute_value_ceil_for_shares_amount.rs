use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn compute_value_ceil_for_shares_amount(
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
        .checked_sub(1)
        .ok_or(UxdError::MathError)?
        .checked_div(total_shares_amount)
        .ok_or(UxdError::MathError)?
        .checked_add(1)
        .ok_or(UxdError::MathError)?)
}
