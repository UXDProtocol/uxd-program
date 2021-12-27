use std::ops::{Mul, Sub};

use fixed::types::I80F48;
use mango::state::PerpAccount;

use super::PerpInfo;

pub fn unsettled_base_amount(perp_account: &PerpAccount) -> i64 {
    perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .unwrap()
}

// Return the base position + the amount that's on EventQueue waiting to be processed
pub fn total_perp_base_lot_position(perp_account: &PerpAccount) -> i64 {
    perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .unwrap()
}

// Return the quote position + the amount that's on EventQueue waiting to be processed (minus fees)
pub fn total_perp_quote_position(perp_account: &PerpAccount, perp_info: &PerpInfo) -> i64 {
    let taker_quote = I80F48::from_num(perp_account.taker_quote)
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    let fee_amount = taker_quote.abs().mul(perp_info.taker_fee);
    let quote_change = taker_quote.sub(fee_amount);
    let total_quote_position = perp_account
        .quote_position
        .checked_add(quote_change)
        .unwrap();
    total_quote_position.checked_to_num().unwrap()
}
