#[cfg(test)]
mod test_limit_utils {

    use crate::mango_utils::{price_to_lot_price, PerpInfo};
    use fixed::types::I80F48;
    use mango::matching::Side;
    use proptest::prelude::*;

    mod test_limit_price {
        use crate::mango_utils::{calculate_slippage_amount, limit_price};

        use super::*;

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
                calculate_slippage_amount(price, slippage)
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

                let slippage_amount = calculate_slippage_amount(fractional_price, slippage).unwrap();
                // expected limit price
                let price_sub_slippage = fractional_price.checked_add(slippage_amount).unwrap();

                prop_assert_eq!(limit_price, price_sub_slippage);
            }
        }

        proptest! {
            #[test]
            fn test_limit_price_ask(price in 0..1000000000000i128, slippage in 0..u32::MAX) {
                // create random price in lamport range from 0 to 1000 equivalent uiAmount
                let fractional_price = I80F48::checked_from_num(price).unwrap();
                // println!("fractional_price = {}, slippage = {}", fractional_price, slippage);

                let limit_price = limit_price(fractional_price, slippage, Side::Ask).unwrap();

                let slippage_amount = calculate_slippage_amount(fractional_price, slippage).unwrap();
                // expected limit price
                let price_add_slippage = fractional_price.checked_sub(slippage_amount).unwrap();

                prop_assert_eq!(limit_price, price_add_slippage);
            }
        }
    }
}
