use crate::error::UxdError;
use fixed::types::I80F48;

// The purpose of the base unit is to set a common ground for numbers with different decimals.
// E.g consider a TokenA/TokenB pool, TokenA is 6 decimals, TokenB is 9 decimals, both tokens worth $1
// There are 1,000,000 TokenA in the pool and 2,000,000,000 TokenB in the pool
// How much is worth the pool?

// Naming: native unit
// e.g 1 USDC (6 decimals) is 1,000,000 in native value

// Naming: base unit
// e.g 1 USDC (6 decimals) is 1,000,000,000 in base unit

pub const BASE_DECIMALS: u8 = 9;

// Convert a number from native unit to base unit
pub fn native_to_base(number: I80F48, decimals: u8) -> Result<I80F48, UxdError> {
    let decimals_power: u64 = 10u64
        .checked_pow(decimals.into())
        .ok_or_else(|| UxdError::MathError)?;

    let base_decimals_power: u64 = 10u64
        .checked_pow(BASE_DECIMALS.into())
        .ok_or_else(|| UxdError::MathError)?;

    number
        .checked_mul(I80F48::from_num(base_decimals_power))
        .ok_or_else(|| UxdError::MathError)?
        .checked_div(I80F48::from_num(decimals_power))
        .ok_or_else(|| UxdError::MathError)
}

// Convert a number from base unit to native unit
pub fn base_to_native(number: I80F48, decimals: u8) -> Result<I80F48, UxdError> {
    let decimals_power: u64 = 10u64
        .checked_pow(decimals.into())
        .ok_or_else(|| UxdError::MathError)?;

    let base_decimals_power: u64 = 10u64
        .checked_pow(BASE_DECIMALS.into())
        .ok_or_else(|| UxdError::MathError)?;

    number
        .checked_mul(I80F48::from_num(decimals_power))
        .ok_or_else(|| UxdError::MathError)?
        .checked_div(I80F48::from_num(base_decimals_power))
        .ok_or_else(|| UxdError::MathError)
}
