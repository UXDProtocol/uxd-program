use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn checked_i64_to_u64(input: i64) -> Result<u64> {
    Ok(u64::try_from(input).ok().ok_or(UxdError::MathError)?)
}
