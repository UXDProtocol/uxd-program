use super::Order;
use super::PerpInfo;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::UxdErrorCode;
use crate::UxdResult;
use crate::SLIPPAGE_BASIS;
use fixed::types::I80F48;
use mango::matching::Side;

declare_check_assert_macros!(SourceFileId::MangoUtilsLimitUtils);

// Return the slippage amount, given a price and a slippage.
pub fn calculate_slippage_amount(price: I80F48, slippage: u32) -> UxdResult<I80F48> {
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).ok_or(math_err!())?;
    return price.checked_mul(slippage_ratio).ok_or(math_err!());
}

// Worse execution price for a provided slippage and side.
// Keep in mind that you'r the Taker when you call this, and that the `matched_side` is the side your order will match against.
// Meaning that you'r willing to go as far as limit price.
//  If you'r buying, matched_side is ASK, and you'll buy from price up to (price + slippage)
//  If you'r selling, matched_side is BID, and you'll sell from price down to (price - slippage)
pub fn limit_price(price: I80F48, slippage: u32, matched_side: Side) -> UxdResult<I80F48> {
    let slippage_amount = calculate_slippage_amount(price, slippage).unwrap();
    return match matched_side {
        Side::Bid => price.checked_sub(slippage_amount).ok_or(math_err!()),
        Side::Ask => price.checked_add(slippage_amount).ok_or(math_err!()),
    };
}

// Convert into a base lot price in quote lot.
// Price is the value of 1 native base unit expressed in native quote.
pub fn price_to_lot_price(price: I80F48, perp_info: &PerpInfo) -> UxdResult<I80F48> {
    price
        .checked_mul(perp_info.base_lot_size)
        .ok_or(math_err!())?
        .checked_div(perp_info.quote_lot_size)
        .ok_or(math_err!())
}

// Check if the provided order is valid given the slippage point and side
pub fn check_effective_order_price_versus_limit_price(
    perp_info: &PerpInfo,
    order: &Order,
    slippage: u32,
) -> UxdResult {
    let market_price = perp_info.price;
    let limit_price = limit_price(market_price, slippage, order.side)?;
    let limit_price_lot = price_to_lot_price(limit_price, &perp_info)?;
    match order.side {
        Side::Bid => {
            if order.price >= limit_price_lot {
                return Ok(());
            }
        }
        Side::Ask => {
            if order.price <= limit_price_lot {
                return Ok(());
            }
        }
    };
    Err(throw_err!(UxdErrorCode::SlippageReached))
}