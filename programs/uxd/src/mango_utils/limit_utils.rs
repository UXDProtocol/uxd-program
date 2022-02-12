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
    let slippage_amount = cal_slippage_amount(price, slippage).unwrap();
    return match side {
        Side::Bid => Ok(price.checked_add(slippage_amount).ok_or(math_err!())?),
        Side::Ask => Ok(price.checked_sub(slippage_amount).ok_or(math_err!())?),
    };
}

fn cal_slippage_amount(price: I80F48, slippage: u32) -> UxdResult<I80F48> {
    let slippage = I80F48::from_num(slippage);
    let slippage_basis = I80F48::from_num(SLIPPAGE_BASIS);
    let slippage_ratio = slippage.checked_div(slippage_basis).ok_or(math_err!())?;
    return price.checked_mul(slippage_ratio).ok_or(math_err!());
}

// Check if the provided order is valid given the slippage and side
pub fn check_effective_order_price_versus_limit_price(
    perp_info: &PerpInfo,
    order: &Order,
    slippage: u32,
) -> UxdResult {
    let market_price = perp_info.price;
    let limit_price = limit_price(market_price, slippage, order.side)?;
    let effective_order_price = limit_price
        .checked_mul(perp_info.base_lot_size)
        .ok_or(math_err!())?
        .checked_div(perp_info.quote_lot_size)
        .ok_or(math_err!())?;
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
    Err(throw_err!(
        UxdErrorCode::EffectiveOrderPriceBeyondLimitPrice
    ))
}

// test
#[cfg(test)]
mod test {
    use crate::mango_utils::{limit_price, limit_utils::cal_slippage_amount};
    use fixed::types::I80F48;
    use mango::matching::Side;
    use proptest::prelude::*;

    #[test]
    fn test_cal_slippage_amount() {
        // general param
        let lamport_basis = I80F48::from_num(10u32.pow(9));

        // given price is 24
        let ui_price = I80F48::from_num(24);
        let price = ui_price.checked_mul(lamport_basis).unwrap();

        // given slippage is 10%
        let slippage = 100u32;

        // expected slippage amount
        let expected = I80F48::from_num(2400000000u64);

        assert_eq!(
            cal_slippage_amount(price, slippage)
                .unwrap()
                .overflowing_round(),
            (expected, false)
        );
    }

    proptest! {
        #[test]
        fn test_limit_price_bid(price in 0..1000000000000i128, slippage in 0..u32::MAX) {
            // create random price in lamport range from 0 to 1000 equivalent uiAmount
            let fractional_price = I80F48::checked_from_num(price).unwrap();
            // println!("fractional_price = {}, slippage = {}", fractional_price, slippage);

            let limit_price = limit_price(fractional_price, slippage, Side::Bid).unwrap();

            let slippage_amount = cal_slippage_amount(fractional_price, slippage).unwrap();
            // expected limit price
            let price_plus_slippage = fractional_price.checked_add(slippage_amount).unwrap();

            prop_assert_eq!(limit_price, price_plus_slippage);
        }

    }

    proptest! {
        #[test]
        fn test_limit_price_ask(price in 0..1000000000000i128, slippage in 0..u32::MAX) {
            // create random price in lamport range from 0 to 1000 equivalent uiAmount
            let fractional_price = I80F48::checked_from_num(price).unwrap();
            // println!("fractional_price = {}, slippage = {}", fractional_price, slippage);

            let limit_price = limit_price(fractional_price, slippage, Side::Ask).unwrap();

            let slippage_amount = cal_slippage_amount(fractional_price, slippage).unwrap();
            // expected limit price
            let price_minus_slippage = fractional_price.checked_sub(slippage_amount).unwrap();

            prop_assert_eq!(limit_price, price_minus_slippage);
        }
    }
}
