use crate::error::UxdError;
use anchor_lang::prelude::*;

use super::checked_u128_to_u64;

// Precision loss may lower the returned value amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_amount_fraction(
    amount: u64,
    fraction_numerator: u64,
    fraction_denominator: u64,
) -> Result<u64> {
    let amount: u128 = amount.into();
    let fraction_numerator: u128 = fraction_numerator.into();
    let fraction_denominator: u128 = fraction_denominator.into();
    let amount_fraction: u128 = amount
        .checked_mul(fraction_numerator)
        .ok_or(UxdError::MathError)?
        .checked_div(fraction_denominator)
        .ok_or(UxdError::MathError)?;
    Ok(checked_u128_to_u64(amount_fraction)?)
}
