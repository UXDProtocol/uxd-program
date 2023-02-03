use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn compute_amount_after_change(amount_before_change: u128, change: i128) -> Result<u128> {
    // In case of a simple positive change (increase), add the two amount
    if change >= 0 {
        let increase: u128 = u128::try_from(change).ok().ok_or(UxdError::MathError)?;
        return Ok(amount_before_change
            .checked_add(increase)
            .ok_or(UxdError::MathError)?);
    }
    // In case of a negative change, substract using the absolute value (decrease)
    let decrease: u128 = if change == i128::MIN {
        // special case: i128::MIN does not have an i128 absolute value
        u128::try_from(i128::MAX)
            .ok()
            .ok_or(UxdError::MathError)?
            .checked_add(1)
            .ok_or(UxdError::MathError)?
    } else {
        u128::try_from(change.checked_abs().ok_or(UxdError::MathError)?)
            .ok()
            .ok_or(UxdError::MathError)?
    };
    Ok(amount_before_change
        .checked_sub(decrease)
        .ok_or(UxdError::MathError)?)
}
