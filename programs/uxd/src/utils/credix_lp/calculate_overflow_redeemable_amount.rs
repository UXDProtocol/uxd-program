use crate::error::UxdError;
use anchor_lang::prelude::Result;

pub fn calculate_overflow_redeemable_amount(
    redeemable_amount_under_management_target_amount: u128,
    redeemable_amount_under_management: u128,
) -> Result<u64> {
    redeemable_amount_under_management_target_amount
        .checked_sub(checked_convert_u128_to_u64(
            redeemable_amount_under_management,
        )?)
        .ok_or(UxdError::MathError)?
}
