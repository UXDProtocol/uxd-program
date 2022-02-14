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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_check_effective_order_price_versus_limit_price_mint_valid_zslip() {
        // Order.price must be below the perpInfo.price within slippage
        let ret = check_effective_order_price_versus_limit_price(
            &PerpInfo {
                market_index: 3,
                price: I80F48::from_num(0.090000000000000), // 90.00$
                base_unit: I80F48::from_num(1000000000),    // SOL 9 decimals
                base_lot_size: I80F48::from_num(10000000),
                quote_unit: I80F48::from_num(1000000), // USD 6 decimals
                quote_lot_size: I80F48::from_num(100),
                taker_fee: I80F48::from_num(0.0005),
            },
            &Order {
                quantity: 0, // whatever not used
                price: 9000, // exact price
                size: 0,     // whatever not used
                side: Side::Bid,
            },
            1, // 0.1%
        );

        if ret.is_err() {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    pub fn test_check_effective_order_price_versus_limit_price_mint_valid() {
        let ret = check_effective_order_price_versus_limit_price(
            &PerpInfo {
                market_index: 3,
                price: I80F48::from_num(0.090000000000000), // 90.00$
                base_unit: I80F48::from_num(1000000000),    // SOL 9 decimals
                base_lot_size: I80F48::from_num(10000000),
                quote_unit: I80F48::from_num(1000000), // USD 6 decimals
                quote_lot_size: I80F48::from_num(100),
                taker_fee: I80F48::from_num(0.0005),
            },
            &Order {
                quantity: 0, // whatever not used
                price: 8911, // less than 1% below
                size: 0,     // whatever not used
                side: Side::Bid,
            },
            10, // 1%
        );

        if ret.is_err() {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    // REDEEM

    #[test]
    pub fn test_check_effective_order_price_versus_limit_price_redeem_valid_zslip() {
        let ret = check_effective_order_price_versus_limit_price(
            &PerpInfo {
                market_index: 3,
                price: I80F48::from_num(0.090000000000000), // 90.00$
                base_unit: I80F48::from_num(1000000000),    // SOL 9 decimals
                base_lot_size: I80F48::from_num(10000000),
                quote_unit: I80F48::from_num(1000000), // USD 6 decimals
                quote_lot_size: I80F48::from_num(100),
                taker_fee: I80F48::from_num(0.0005),
            },
            &Order {
                quantity: 0, // whatever not used
                price: 9000, // less than 1% above
                size: 0,     // whatever not used
                side: Side::Ask,
            },
            1, // 0.1%
        );

        if ret.is_err() {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    pub fn test_check_effective_order_price_versus_limit_price_mint_invalid() {
        let ret = check_effective_order_price_versus_limit_price(
            &PerpInfo {
                market_index: 3,
                price: I80F48::from_num(0.090000000000000), // 90.00$
                base_unit: I80F48::from_num(1000000000),    // SOL 9 decimals
                base_lot_size: I80F48::from_num(10000000),
                quote_unit: I80F48::from_num(1000000), // USD 6 decimals
                quote_lot_size: I80F48::from_num(100),
                taker_fee: I80F48::from_num(0.0005),
            },
            &Order {
                quantity: 0, // whatever not used
                price: 8909, // more than 1% below
                size: 0,     // whatever not used
                side: Side::Bid,
            },
            10, // 1%
        );

        if ret.is_err() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_check_effective_order_price_versus_limit_price_redeem_valid() {
        let ret = check_effective_order_price_versus_limit_price(
            &PerpInfo {
                market_index: 3,
                price: I80F48::from_num(0.090000000000000), // 90.00$
                base_unit: I80F48::from_num(1000000000),    // SOL 9 decimals
                base_lot_size: I80F48::from_num(10000000),
                quote_unit: I80F48::from_num(1000000), // USD 6 decimals
                quote_lot_size: I80F48::from_num(100),
                taker_fee: I80F48::from_num(0.0005),
            },
            &Order {
                quantity: 0, // whatever not used
                price: 9089, // less than 1% above
                size: 0,     // whatever not used
                side: Side::Ask,
            },
            10, // 1%
        );

        if ret.is_err() {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    pub fn test_check_effective_order_price_versus_limit_price_redeem_invalid() {
        let ret = check_effective_order_price_versus_limit_price(
            &PerpInfo {
                market_index: 3,
                price: I80F48::from_num(0.090000000000000), // 90.00$
                base_unit: I80F48::from_num(1000000000),    // SOL 9 decimals
                base_lot_size: I80F48::from_num(10000000),
                quote_unit: I80F48::from_num(1000000), // USD 6 decimals
                quote_lot_size: I80F48::from_num(100),
                taker_fee: I80F48::from_num(0.0005),
            },
            &Order {
                quantity: 0, // whatever not used
                price: 9091, // more than 1% above
                size: 0,     // whatever not used
                side: Side::Ask,
            },
            10, // 1%
        );

        if ret.is_err() {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}