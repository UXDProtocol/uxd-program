use crate::BPS_UNIT_CONVERSION;
use anchor_lang::prelude::*;

use super::compute_amount_less_fraction_floor;

pub fn calculate_amount_less_fees(amount: u64, fee_amount_in_bps: u8) -> Result<u64> {
    let fraction_numerator: u64 = fee_amount_in_bps.into();
    let fraction_denominator: u64 = BPS_UNIT_CONVERSION;
    compute_amount_less_fraction_floor(amount, fraction_numerator, fraction_denominator)
}
