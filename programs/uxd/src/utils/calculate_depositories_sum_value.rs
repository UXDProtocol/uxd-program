use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::UxdError;
use crate::utils::checked_add;
use crate::ROUTER_DEPOSITORIES_COUNT;

/**
 * Compute the sum of one value for each known depositories
 */
pub fn calculate_depositories_sum_value(depositories_values: &Vec<u64>) -> Result<u64> {
    require!(
        depositories_values.len() == ROUTER_DEPOSITORIES_COUNT,
        UxdError::InvalidDepositoriesVector
    );
    let sum = depositories_values
        .iter()
        .try_fold(0u64, |accumulator: u64, value: &u64| {
            checked_add(accumulator, *value)
        })?;
    Ok(sum)
}
