use crate::error::UxdError;
use anchor_lang::prelude::*;
use fixed::types::I80F48;

pub fn compute_delta(before: u64, after: u64) -> Result<i64> {
    let before_fixed = I80F48::checked_from_num(before).ok_or(UxdError::MathError)?;
    let after_fixed = I80F48::checked_from_num(after).ok_or(UxdError::MathError)?;
    let delta_fixed = after_fixed
        .checked_sub(before_fixed)
        .ok_or(UxdError::MathError)?;
    Ok(delta_fixed
        .checked_to_num::<i64>()
        .ok_or(UxdError::MathError)?)
}
