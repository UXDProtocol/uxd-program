use super::PerpInfo;
use crate::error::UxdError;
use crate::SLIPPAGE_BASIS;
use anchor_lang::prelude::*;
use fixed::types::I80F48;
use mango::matching::Side;

// Return the slippage amount, given a price and a slippage.
pub fn calculate_slippage_amount(price: I80F48, slippage: u16) -> Result<I80F48> {
    let slippage_ratio = I80F48::from_num(slippage) / I80F48::from_num(SLIPPAGE_BASIS);
    price
        .checked_mul(slippage_ratio)
        .ok_or_else(|| error!(UxdError::MathError))
}

// Worse execution price for a provided slippage and side.
// Keep in mind that you'r the Taker when you call this, and that the `matched_side` is the side your order will match against.
// Meaning that you'r willing to go as far as limit price.
//  If you are BID as the taker, matched_side is ASK, and you'll buy from price down to (price + slippage)
//  If you are ASK as the taker, matched_side is BID, and you'll sell from price down to (price - slippage)
pub fn limit_price(price: I80F48, slippage: u16, taker_side: Side) -> Result<I80F48> {
    let slippage_amount = calculate_slippage_amount(price, slippage)?;
    match taker_side {
        Side::Bid => price
            .checked_add(slippage_amount)
            .ok_or_else(|| error!(UxdError::MathError)),
        Side::Ask => price
            .checked_sub(slippage_amount)
            .ok_or_else(|| error!(UxdError::MathError)),
    }
}

// Convert price into a quote lot per base lot price.
// Price is the value of 1 native base unit expressed in native quote.
pub fn price_to_lot_price(price: I80F48, perp_info: &PerpInfo) -> Result<I80F48> {
    price
        .checked_mul(perp_info.base_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_div(perp_info.quote_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))
}
