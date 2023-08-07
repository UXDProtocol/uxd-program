use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn compute_increase(before: u64, after: u64) -> Result<u64> {
    Ok(after.checked_sub(before).ok_or(UxdError::MathOverflow)?)
}
