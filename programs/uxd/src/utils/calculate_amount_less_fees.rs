use crate::BPS_POWER;
use anchor_lang::prelude::*;

use super::compute_amount_fraction_floor;

pub fn calculate_amount_less_fees(amount: u64, fee_amount_in_bps: u8) -> Result<u64> {
    let fraction_numerator: u64 = BPS_POWER.saturating_sub(fee_amount_in_bps.into());
    let fraction_denominator: u64 = BPS_POWER;
    compute_amount_fraction_floor(amount, fraction_numerator, fraction_denominator)
}
