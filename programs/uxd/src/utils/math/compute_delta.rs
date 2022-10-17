use crate::error::UxdError;
use anchor_lang::prelude::*;
use fixed::types::I80F48;

pub fn compute_delta(amount_before: u64, amount_after: u64) -> Result<i64> {
    let amount_before_fixed = I80F48::checked_from_num(amount_before).ok_or(UxdError::MathError)?;
    let amount_after_fixed = I80F48::checked_from_num(amount_after).ok_or(UxdError::MathError)?;
    let delta_fixed = amount_after_fixed
        .checked_sub(amount_before_fixed)
        .ok_or(UxdError::MathError)?;
    Ok(delta_fixed
        .checked_to_num::<i64>()
        .ok_or(UxdError::MathError)?)
}
