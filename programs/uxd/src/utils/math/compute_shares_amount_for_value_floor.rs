use crate::error::UxdError;
use crate::utils::checked_as_u64;
use crate::utils::checked_div;
use crate::utils::checked_mul;
use anchor_lang::prelude::*;

// Precision loss may lower the returned shares amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_shares_amount_for_value_floor(
    value: u64,
    total_shares_supply: u64,
    total_shares_value: u64,
) -> Result<u64> {
    if value == 0 {
        return Ok(0);
    }
    require!(total_shares_supply > 0, UxdError::MathOverflow);
    require!(total_shares_value > 0, UxdError::MathOverflow);
    let shares_amount: u128 = checked_div::<u128>(
        checked_mul::<u128>(u128::from(value), u128::from(total_shares_supply))?,
        u128::from(total_shares_value),
    )?;
    checked_as_u64(shares_amount)
}
