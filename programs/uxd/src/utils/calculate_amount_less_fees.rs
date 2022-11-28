use crate::BPS_UNIT_CONVERSION;
use anchor_lang::prelude::*;

use super::compute_amount_less_fraction;

pub fn calculate_amount_less_fees(amount: u64, fee_amount_in_bps: u8) -> Result<u64> {
    Ok(compute_amount_less_fraction(
        amount,
        fee_amount_in_bps.into(),
        BPS_UNIT_CONVERSION,
    )?)
}
