use crate::error::UxdError;
use anchor_lang::prelude::*;
use mango::state::PerpAccount;

// Return the base position + the amount that's on EventQueue waiting to be processed
pub fn total_perp_base_lot_position(perp_account: &PerpAccount) -> Result<i64> {
    perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .ok_or(error!(UxdError::MathError))
}
