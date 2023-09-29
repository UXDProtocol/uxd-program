use crate::error::UxdError;
use crate::utils::checked_ceil_div;
use anchor_lang::prelude::*;

pub fn compute_value_for_single_share_ceil(
    total_shares_value: u64,
    total_shares_supply: u64,
) -> Result<u64> {
    require!(total_shares_value > 0, UxdError::MathOverflow);
    require!(total_shares_supply > 0, UxdError::MathOverflow);
    checked_ceil_div(total_shares_value, total_shares_supply)
}
