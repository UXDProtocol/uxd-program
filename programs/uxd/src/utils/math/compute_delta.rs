use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn compute_delta(before: u64, after: u64) -> Result<i64> {
    let before_signed = i64::try_from(before).ok().ok_or(UxdError::MathError)?;
    let after_signed = i64::try_from(after).ok().ok_or(UxdError::MathError)?;
    Ok(after_signed
        .checked_sub(before_signed)
        .ok_or(UxdError::MathError)?)
}
