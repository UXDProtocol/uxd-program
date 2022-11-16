use crate::error::UxdError;
use anchor_lang::prelude::*;
use fixed::types::I80F48;

// Precision loss may lower the returned owner value amount.
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
    let shares_amount_fixed = I80F48::checked_from_num(shares_amount).ok_or(UxdError::MathError)?;
    let total_shares_amount_fixed =
        I80F48::checked_from_num(total_shares_amount).ok_or(UxdError::MathError)?;
    let total_shares_value_fixed =
        I80F48::checked_from_num(total_shares_value).ok_or(UxdError::MathError)?;
    let shares_value_fixed = shares_amount_fixed
        .checked_mul(total_shares_value_fixed)
        .ok_or(UxdError::MathError)?
        .checked_div(total_shares_amount_fixed)
        .ok_or(UxdError::MathError)?;
    Ok(shares_value_fixed
        .checked_to_num::<u64>()
        .ok_or(UxdError::MathError)?)
}
