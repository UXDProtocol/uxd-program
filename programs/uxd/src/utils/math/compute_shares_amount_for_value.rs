use crate::error::UxdError;
use anchor_lang::prelude::*;
use fixed::types::I80F48;

// Precision loss may lower the returned shares amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_shares_amount_for_value(
    value: u64,
    total_shares_amount: u64,
    total_shares_value: u64,
) -> Result<u64> {
    if value == 0 {
        return Ok(0);
    }
    require!(value <= total_shares_value, UxdError::MathError);
    let value_fixed = I80F48::checked_from_num(value).ok_or(UxdError::MathError)?;
    let total_shares_amount_fixed =
        I80F48::checked_from_num(total_shares_amount).ok_or(UxdError::MathError)?;
    let total_shares_value_fixed =
        I80F48::checked_from_num(total_shares_value).ok_or(UxdError::MathError)?;
    let shares_amount_fixed = value_fixed
        .checked_mul(total_shares_amount_fixed)
        .ok_or(UxdError::MathError)?
        .checked_div(total_shares_value_fixed)
        .ok_or(UxdError::MathError)?;
    Ok(shares_amount_fixed
        .checked_to_num::<u64>()
        .ok_or(UxdError::MathError)?)
}
