use crate::error::UxdError;
use anchor_lang::prelude::*;

use super::is_within_range_inclusive;

pub fn is_equal_with_precision_loss(
    amount_before_precision_loss: u64,
    amount_after_precision_loss: u64,
    allowed_precion_loss: u64,
) -> Result<bool> {
    let amount_max = amount_before_precision_loss;
    let amount_min = amount_before_precision_loss
        .checked_sub(allowed_precion_loss)
        .ok_or(UxdError::MathError)?;
    return Ok(is_within_range_inclusive(
        amount_after_precision_loss,
        amount_min,
        amount_max,
    ));
}
