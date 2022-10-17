use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn is_equal_with_precision_loss(
    amount_before_precision_loss: u64,
    amount_after_precision_loss: u64,
    allowed_precion_loss: u64,
) -> Result<bool> {
    let amount_max = amount_before_precision_loss;
    let amount_min = amount_before_precision_loss
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
