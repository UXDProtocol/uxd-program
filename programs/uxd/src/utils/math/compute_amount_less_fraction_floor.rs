use crate::error::UxdError;
use anchor_lang::prelude::*;

// Precision loss may lower the returned amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_amount_less_fraction_floor(
    amount: u64,
    fraction_numerator: u64,
    fraction_denominator: u64,
) -> Result<u64> {
    require!(fraction_denominator > 0, UxdError::MathOverflow);
    let amount: u128 = amount.into();
    let fraction_numerator: u128 = fraction_numerator.into();
    let fraction_denominator: u128 = fraction_denominator.into();
    let amount_less_fraction: u128 = amount
        .checked_mul(
            fraction_denominator
                .checked_sub(fraction_numerator)
                .ok_or(UxdError::MathOverflow)?,
        )
        .ok_or(UxdError::MathOverflow)?
        .checked_div(fraction_denominator)
        .ok_or(UxdError::MathOverflow)?;
    Ok(u64::try_from(amount_less_fraction)
        .ok()
        .ok_or(UxdError::MathOverflow)?)
}
