use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn checked_u128_to_u64(value: u128) -> Result<u64> {
    Ok(u64::try_from(value).ok().ok_or(UxdError::MathError)?)
}
