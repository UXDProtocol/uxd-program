use crate::error::UxdError;
use crate::utils::checked_add;
use crate::utils::checked_sub;
use crate::utils::checked_as_u128;
use anchor_lang::prelude::*;

pub fn checked_add_u128_and_i128(value_before: u128, change_delta: i128) -> Result<u128> {
    // In case of a simple positive change (increase), add the two positive values
    if change_delta >= 0 {
        let increase: u128 = checked_as_u128(change_delta)?;
        return Ok(checked_add(value_before, increase)?);
    }
    // In case of a negative change, substract the absolute value of the delta (decrease)
    let decrease: u128 = if change_delta == i128::MIN {
        // special case: i128::MIN does not have an i128 absolute value
        checked_add(checked_as_u128(i128::MAX)?, 1)?
    } else {
        checked_as_u128(change_delta.checked_abs().ok_or(UxdError::MathOverflow)?)?
    };
    Ok(checked_sub(value_before, decrease)?)
}
