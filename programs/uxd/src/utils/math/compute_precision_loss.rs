use anchor_lang::prelude::*;

use crate::error::UxdError;

pub fn compute_precision_loss(total_shares_amount: u64, total_shares_value: u64) -> Result<u64> {
    // ceil ( total_shares_amount / total_shares_value )
    // is equivalent to (sum - 1) / divisor + 1
    if total_shares_value == 0 {
        return Ok(0);
    }
    Ok(total_shares_amount
        .checked_sub(1)
        .ok_or(UxdError::MathError)?
        .checked_div(total_shares_value)
        .ok_or(UxdError::MathError)?
        .checked_add(1)
        .ok_or(UxdError::MathError)?)
}
