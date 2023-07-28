use crate::error::UxdError;
use anchor_lang::prelude::*;

// Rounding error may increase the returned amount.
// Rounding error of 1 native unit may be expected.
pub fn compute_amount_fraction_ceil(
    amount: u64,
    fraction_numerator: u64,
    fraction_denominator: u64,
) -> Result<u64> {
    require!(fraction_denominator > 0, UxdError::MathOverflow);
    if fraction_numerator == 0 || amount == 0 {
        return Ok(0);
    }
    let amount: u128 = amount.into();
    let fraction_numerator: u128 = fraction_numerator.into();
    let fraction_denominator: u128 = fraction_denominator.into();
    let amount_fraction_ceil: u128 = amount
        .checked_mul(fraction_numerator)
        .ok_or(UxdError::MathOverflow)?
        .checked_sub(1)
        .ok_or(UxdError::MathOverflow)?
        .checked_div(fraction_denominator)
        .ok_or(UxdError::MathOverflow)?
        .checked_add(1)
        .ok_or(UxdError::MathOverflow)?;
    Ok(u64::try_from(amount_fraction_ceil)
        .ok()
        .ok_or(UxdError::MathOverflow)?)
}
