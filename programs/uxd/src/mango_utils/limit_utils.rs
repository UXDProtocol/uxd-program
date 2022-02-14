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

// Worse execution price for a provided slippage and side
pub fn limit_price(price: I80F48, slippage: u32, side: Side) -> UxdResult<I80F48> {
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).ok_or(math_err!())?;
    let slippage_amount = price.checked_mul(slippage_ratio).ok_or(math_err!())?;
    let limit_price = match side {
        Side::Bid => price.checked_sub(slippage_amount).ok_or(math_err!()),
        Side::Ask => price.checked_add(slippage_amount).ok_or(math_err!()),
    }?;
    Ok(limit_price)
}

// Check if the provided order is valid given the slippage and side
pub fn check_effective_order_price_versus_limit_price(
    perp_info: &PerpInfo,
    order: &Order,
    slippage: u32,
) -> UxdResult {
    let market_price = perp_info.price;
    let limit_price = limit_price(market_price, slippage, order.side)?;
    let limit_price_lot = limit_price
        .checked_mul(perp_info.base_lot_size)
        .ok_or(math_err!())?
        .checked_div(perp_info.quote_lot_size)
        .ok_or(math_err!())?;
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
