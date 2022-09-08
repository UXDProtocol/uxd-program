use fixed::types::I80F48;

use crate::error::UxdError;

// E.g: number is 2,000,000,000 (9 decimals), target is 6 decimals, result is 2,000,000
pub fn change_decimals_place(
    number: I80F48,
    decimals: u8,
    target_decimals: u8,
) -> Result<I80F48, UxdError> {
    let decimals_pow = I80F48::checked_from_num(
        10u64
            .checked_pow(decimals.into())
            .ok_or(UxdError::MathError)?,
    )
    .ok_or(UxdError::MathError)?;

    let target_decimals = I80F48::checked_from_num(
        10u64
            .checked_pow(target_decimals.into())
            .ok_or(UxdError::MathError)?,
    )
    .ok_or(UxdError::MathError)?;

    Ok(number
        .checked_div(decimals_pow)
        .ok_or(UxdError::MathError)?
        .checked_mul(target_decimals)
        .ok_or(UxdError::MathError)?)
}
