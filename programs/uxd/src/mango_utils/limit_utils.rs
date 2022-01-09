use super::Order;
use super::PerpInfo;
use crate::ErrorCode;
use crate::UxdResult;
use crate::SLIPPAGE_BASIS;
use fixed::types::I80F48;
use mango::matching::Side;

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

// test
#[cfg(test)]
mod test {
    use crate::{mango_utils::limit_price, SLIPPAGE_BASIS};
    use fixed::types::I80F48;
    use mango::matching::Side;
    use proptest::prelude::*;

    fn cal_slippage(price: I80F48, slippage: u32) -> I80F48 {
        let slippage = I80F48::checked_from_num(slippage).unwrap();
        let slippage_basis = I80F48::checked_from_num(SLIPPAGE_BASIS).unwrap();
        let slippage_ratio = slippage.checked_div(slippage_basis).unwrap();
        return price.checked_mul(slippage_ratio).unwrap();
    }

    proptest! {
        #[test]
        fn test_limit_price_bid(price in 0..1000000i128, slippage in 0..u32::MAX) {
            // create random fraction with 9 decimals
            let fractional_price = I80F48::from_bits(price << 48-9);
            // println!("fractional_price = {}, slippage = {}", fractional_price, slippage);

            let limit_price = limit_price(fractional_price, slippage, Side::Bid);

            let slippage_amount = cal_slippage(fractional_price, slippage);
            // expected limit price
            let price_plus_slippage = fractional_price.checked_add(slippage_amount).unwrap();

            prop_assert_eq!(limit_price, price_plus_slippage);
        }

        #[test]
        fn test_limit_price_ask(price in 0..1000000i128, slippage in 0..u32::MAX) {
            // create random fraction with 9 decimals
            let fractional_price = I80F48::from_bits(price << 48-9);
            // println!("fractional_price = {}, slippage = {}", fractional_price, slippage);

            let limit_price = limit_price(fractional_price, slippage, Side::Ask);

            let slippage_amount = cal_slippage(fractional_price, slippage);
            // expected limit price
            let price_minus_slippage = fractional_price.checked_sub(slippage_amount).unwrap();
            
            prop_assert_eq!(limit_price, price_minus_slippage);
        }
    }
}
