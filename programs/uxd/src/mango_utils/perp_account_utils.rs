use crate::error::UxdError;
use anchor_lang::prelude::*;
use fixed::types::I80F48;
use mango::state::PerpAccount;

// Return the base position + the amount that's on EventQueue waiting to be processed
pub fn total_perp_base_lot_position(perp_account: &PerpAccount) -> Result<I80F48> {
    Ok(I80F48::from_num(
        perp_account
            .base_position
            .checked_add(perp_account.taker_base)
            .ok_or_else(|| error!(UxdError::MathError))?,
    ))
}
