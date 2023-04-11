use anchor_lang::prelude::Result;

use crate::error::UxdError;

/**
 * Compute the sum of one value for each known depositories
 */
pub fn calculate_depositories_sum_value(
    identity_depository_value: u64,
    mercurial_vault_depository_0_value: u64,
    credix_lp_depository_0_value: u64,
) -> Result<u64> {
    Ok(identity_depository_value
        .checked_add(mercurial_vault_depository_0_value)
        .ok_or(UxdError::MathError)?
        .checked_add(credix_lp_depository_0_value)
        .ok_or(UxdError::MathError)?)
}
