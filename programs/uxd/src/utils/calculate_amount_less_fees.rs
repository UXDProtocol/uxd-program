use crate::error::UxdError;
use crate::BPS_UNIT_CONVERSION;
use anchor_lang::prelude::*;
use fixed::types::I80F48;

pub fn calculate_amount_less_fees(amount: u64, fee_amount_in_bps: u8) -> Result<u64> {
    // Math: 5 bps fee would equate to bps_minted_to_user
    // being 9995 since 10000 - 5 = 9995
    let bps_less_fees: I80F48 = I80F48::checked_from_num(BPS_UNIT_CONVERSION)
        .ok_or(UxdError::MathError)?
        .checked_sub(fee_amount_in_bps.into())
        .ok_or(UxdError::MathError)?;

    // Math: Multiplies the base_amount by BPS less fees
    // then divide by the BPS_UNIT_CONVERSION to convert BPS to amount
    let amount_less_fees = bps_less_fees
        .checked_mul_int(amount.into())
        .ok_or(UxdError::MathError)?
        .checked_div_int(BPS_UNIT_CONVERSION.into())
        .ok_or(UxdError::MathError)?
        // Round down the number to attribute the precision loss to the user
        // It is redundant with u64 transformation, but we like it to be exhaustive
        .checked_floor()
        .ok_or(UxdError::MathError)?;

    Ok(amount_less_fees
        .checked_to_num::<u64>()
        .ok_or(UxdError::MathError)?)
}
