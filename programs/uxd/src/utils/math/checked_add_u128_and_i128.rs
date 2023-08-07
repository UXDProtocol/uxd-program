use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn checked_add_u128_and_i128(value_before: u128, change_delta: i128) -> Result<u128> {
    // In case of a simple positive change (increase), add the two positive values
    if change_delta >= 0 {
        let increase: u128 = u128::try_from(change_delta)
            .ok()
            .ok_or(UxdError::MathOverflow)?;
        return Ok(value_before
            .checked_add(increase)
            .ok_or(UxdError::MathOverflow)?);
    }
    // In case of a negative change, substract the absolute value of the delta (decrease)
    let decrease: u128 = if change_delta == i128::MIN {
        // special case: i128::MIN does not have an i128 absolute value
        u128::try_from(i128::MAX)
            .ok()
            .ok_or(UxdError::MathOverflow)?
            .checked_add(1)
            .ok_or(UxdError::MathOverflow)?
    } else {
        u128::try_from(change_delta.checked_abs().ok_or(UxdError::MathOverflow)?)
            .ok()
            .ok_or(UxdError::MathOverflow)?
    };
    Ok(value_before
        .checked_sub(decrease)
        .ok_or(UxdError::MathOverflow)?)
}
