use std::ops::Mul;
use std::ops::Sub;

use super::Order;
use super::PerpInfo;
use crate::ErrorCode;
use crate::UxdResult;
use crate::SLIPPAGE_BASIS;
use fixed::types::I80F48;
use mango::matching::Side;
use mango::state::PerpAccount;

// Worse execution price for a provided slippage and side
pub fn limit_price(price: I80F48, slippage: u32, side: Side) -> I80F48 {
    let slippage = I80F48::checked_from_num(slippage).unwrap();
    let slippage_basis = I80F48::checked_from_num(SLIPPAGE_BASIS).unwrap();
    let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
    let slippage_amount = price.checked_mul(slippage_ratio).unwrap();
    return match side {
        Side::Bid => price.checked_add(slippage_amount).unwrap(),
        Side::Ask => price.checked_sub(slippage_amount).unwrap(),
    };
}

// Check if the provided order is valid given the slippage and side
pub fn check_effective_order_price_versus_limit_price(
    perp_info: &PerpInfo,
    order: &Order,
    slippage: u32,
) -> UxdResult {
    let market_price = perp_info.price;
    let limit_price = limit_price(market_price, slippage, order.side);
    let effective_order_price = limit_price
        .checked_mul(perp_info.base_lot_size)
        .unwrap()
        .checked_div(perp_info.quote_lot_size)
        .unwrap();
    match order.side {
        Side::Bid => {
            if order.price < effective_order_price {
                return Ok(());
            }
        }
        Side::Ask => {
            if order.price > effective_order_price {
                return Ok(());
            }
        }
    };
    Err(ErrorCode::InvalidSlippage)
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
