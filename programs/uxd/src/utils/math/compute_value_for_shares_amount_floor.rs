use crate::error::UxdError;
use crate::utils::checked_as_u64;
use crate::utils::checked_div;
use crate::utils::checked_mul;
use anchor_lang::prelude::*;

// Precision loss may lower the returned value amount.
// Precision loss of 1 native unit may be expected.
pub fn compute_value_for_shares_amount_floor(
    shares_amount: u64,
    total_shares_supply: u64,
    total_shares_value: u64,
) -> Result<u64> {
    if shares_amount == 0 {
        return Ok(0);
    }
    require!(total_shares_supply > 0, UxdError::MathOverflow);
    require!(total_shares_value > 0, UxdError::MathOverflow);
    let value: u128 = checked_div::<u128>(
        checked_mul::<u128>(u128::from(shares_amount), u128::from(total_shares_value))?,
        u128::from(total_shares_supply),
    )?;
    checked_as_u64(value)
}
