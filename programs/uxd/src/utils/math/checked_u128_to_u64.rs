use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn checked_u128_to_u64(amount: u128) -> Result<u64> {
    Ok(u64::try_from(amount).ok().ok_or(UxdError::MathError)?)
}
