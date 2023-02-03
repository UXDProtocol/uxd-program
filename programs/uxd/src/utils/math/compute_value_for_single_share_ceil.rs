use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn compute_value_for_single_share_ceil(
    total_shares_value: u64,
    total_shares_supply: u64,
) -> Result<u64> {
    // ceil ( total_shares_value / total_shares_supply )
    // is equivalent to (total_shares_value - 1) / total_shares_supply + 1
    require!(total_shares_value > 0, UxdError::MathError);
    require!(total_shares_supply > 0, UxdError::MathError);
    Ok(total_shares_value
        .checked_sub(1)
        .ok_or(UxdError::MathError)?
        .checked_div(total_shares_supply)
        .ok_or(UxdError::MathError)?
        .checked_add(1)
        .ok_or(UxdError::MathError)?)
}
