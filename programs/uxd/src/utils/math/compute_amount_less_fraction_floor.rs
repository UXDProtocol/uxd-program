use crate::error::UxdError;
use crate::utils::checked_as_u64;
use crate::utils::checked_div;
use crate::utils::checked_mul;
use crate::utils::checked_sub;
use anchor_lang::prelude::*;

// Precision loss may lower the returned amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_amount_less_fraction_floor(
    amount: u64,
    fraction_numerator: u64,
    fraction_denominator: u64,
) -> Result<u64> {
    require!(fraction_denominator > 0, UxdError::MathOverflow);
    require!(
        fraction_denominator >= fraction_numerator,
        UxdError::MathOverflow
    );
    let amount_less_fraction: u128 = checked_div::<u128>(
        checked_mul::<u128>(
            u128::from(amount),
            checked_sub::<u128>(
                u128::from(fraction_denominator),
                u128::from(fraction_numerator),
            )?,
        )?,
        u128::from(fraction_denominator),
    )?;
    Ok(checked_as_u64(amount_less_fraction)?)
}
