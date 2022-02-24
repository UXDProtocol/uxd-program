use super::Order;
use super::PerpInfo;
use crate::error::UxdError;
use crate::SLIPPAGE_BASIS;
use anchor_lang::prelude::*;
use fixed::types::I80F48;
use mango::matching::Side;

// Return the slippage amount, given a price and a slippage.
pub fn calculate_slippage_amount(price: I80F48, slippage: u32) -> Result<I80F48> {
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage
        .checked_div(slippage_basis)
        .ok_or(error!(UxdError::MathError))?;
    price
        .checked_mul(slippage_ratio)
        .ok_or(error!(UxdError::MathError))
}

// Worse execution price for a provided slippage and side.
// Keep in mind that you'r the Taker when you call this, and that the `matched_side` is the side your order will match against.
// Meaning that you'r willing to go as far as limit price.
//  If you are BID as the taker, matched_side is ASK, and you'll buy from price down to (price + slippage)
//  If you are ASK as the taker, matched_side is BID, and you'll sell from price down to (price - slippage)
pub fn limit_price(price: I80F48, slippage: u32, taker_side: Side) -> Result<I80F48> {
    let slippage_amount = calculate_slippage_amount(price, slippage)?;
    match taker_side {
        Side::Bid => price
            .checked_add(slippage_amount)
            .ok_or(error!(UxdError::MathError)),
        Side::Ask => price
            .checked_sub(slippage_amount)
            .ok_or(error!(UxdError::MathError)),
    }
}

// Convert price into a quote lot per base lot price.
// Price is the value of 1 native base unit expressed in native quote.
pub fn price_to_lot_price(price: I80F48, perp_info: &PerpInfo) -> Result<I80F48> {
    price
        .checked_mul(perp_info.base_lot_size)
        .ok_or(error!(UxdError::MathError))?
        .checked_div(perp_info.quote_lot_size)
        .ok_or(error!(UxdError::MathError))
}

// Check if the provided order is valid given the slippage point and side
// TODO: Doesn't handle 0 slippage - Currently `validate` checks for slippage != 0
pub fn check_effective_order_price_versus_limit_price(
    perp_info: &PerpInfo,
    order: &Order,
    slippage: u32,
) -> Result<()> {
    let market_price = perp_info.price;
    let limit_price = limit_price(market_price, slippage, order.taker_side)?;
    let limit_price_lot = price_to_lot_price(limit_price, perp_info)?;
    match order.taker_side {
        Side::Bid => {
            // Bid up to limit price
            if order.price <= limit_price_lot {
                return Ok(());
            }
        }
        Side::Ask => {
            // Ask for at least limit price
            if order.price >= limit_price_lot {
                return Ok(());
            }
        }
    };
    Err(error!(UxdError::SlippageReached))
}
