use crate::error::UxdError;
use anchor_lang::prelude::*;

// Precision loss may lower the returned value amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_amount_less_fraction(
    amount: u64,
    fraction_numerator: u64,
    fraction_denominator: u64,
) -> Result<u64> {
    Ok(amount
        .checked_mul(
            fraction_denominator
                .checked_sub(fraction_numerator)
                .ok_or(UxdError::MathError)?,
        )
        .ok_or(UxdError::MathError)?
        .checked_div(fraction_denominator)
        .ok_or(UxdError::MathError)?)
}
