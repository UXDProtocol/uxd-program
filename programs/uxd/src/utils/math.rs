use crate::error::UxdError;
use anchor_lang::prelude::*;
use fixed::types::I80F48;

pub fn math_checked_i64_to_u64(input: i64) -> Result<u64> {
    Ok(u64::try_from(input).ok().ok_or(UxdError::MathError)?)
}

pub fn math_compute_delta(amount_before: u64, amount_after: u64) -> Result<i64> {
    let amount_before_fixed = I80F48::checked_from_num(amount_before).ok_or(UxdError::MathError)?;
    let amount_after_fixed = I80F48::checked_from_num(amount_after).ok_or(UxdError::MathError)?;
    let delta_fixed = amount_after_fixed
        .checked_sub(amount_before_fixed)
        .ok_or(UxdError::MathError)?;
    Ok(delta_fixed
        .checked_to_num::<i64>()
        .ok_or(UxdError::MathError)?)
}

pub fn math_is_equal_with_precision_loss(
    amount_before_precision_loss: u64,
    amount_after_precision_loss: u64,
    allowed_precion_loss: u64,
) -> Result<bool> {
    let amount_max = amount_before_precision_loss;
    let amount_min = amount_after_precision_loss
        .checked_sub(allowed_precion_loss)
        .ok_or(UxdError::MathError)?;
    if amount_after_precision_loss > amount_max {
        return Ok(false);
    }
    if amount_after_precision_loss < amount_min {
        return Ok(false);
    }
    Ok(true)
}
