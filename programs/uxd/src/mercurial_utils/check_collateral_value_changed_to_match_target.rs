use anchor_lang::prelude::*;

use crate::{error::UxdError, utils::is_within_range_inclusive};

// Check that the collateral value changed matches the collateral amount we wanted the user to receive:
pub fn check_collateral_value_changed_to_match_target(
    collateral_value_changed: u64,
    target: u64,
    possible_lp_token_precision_loss_collateral_value: u64,
) -> Result<()> {
    // Lp token precision loss + withdraw collateral precision loss
    let maximum_allowed_precision_loss = possible_lp_token_precision_loss_collateral_value
        .checked_add(1)
        .ok_or(UxdError::MathError)?;

    let target_minimal_allowed_value = target
        .checked_sub(maximum_allowed_precision_loss)
        .ok_or(UxdError::MathError)?;

    require!(
        is_within_range_inclusive(
            collateral_value_changed,
            target_minimal_allowed_value,
            target
        ),
        UxdError::SlippageReached,
    );

    Ok(())
}
