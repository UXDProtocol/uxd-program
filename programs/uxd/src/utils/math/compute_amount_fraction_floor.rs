use crate::error::UxdError;
use crate::utils::checked_as_u64;
use crate::utils::checked_div;
use crate::utils::checked_mul;
use anchor_lang::prelude::*;

// Rounding error may decrease the returned amount.
// Rounding error of 1 native unit may be expected.
pub fn compute_amount_fraction_floor(
    amount: u64,
    fraction_numerator: u64,
    fraction_denominator: u64,
) -> Result<u64> {
    require!(fraction_denominator > 0, UxdError::MathOverflow);
    if fraction_numerator == 0 || amount == 0 {
        return Ok(0);
    }
    let amount_fraction_floor: u128 = checked_div::<u128>(
        checked_mul::<u128>(u128::from(amount), u128::from(fraction_numerator))?,
        u128::from(fraction_denominator),
    )?;
    checked_as_u64(amount_fraction_floor)
}
