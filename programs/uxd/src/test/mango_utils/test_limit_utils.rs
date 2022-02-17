#[cfg(test)]
mod test_limit_utils {

    use crate::{
        mango_utils::{price_to_lot_price, Order, PerpInfo},
        UxdResult,
    };
    use fixed::types::I80F48;
    use mango::matching::Side;
    use proptest::prelude::*;

    // price expressed in native quote per native base
    fn mocked_perp_info(price: f64) -> PerpInfo {
        PerpInfo {
            market_index: 3,
            // Price is the price of 1 native unit of BASE expressed in native unit of QUOTE
            price: I80F48::from_num(price),
            base_unit: I80F48::from_num(1_000_000_000), // SOL 9 decimals
            base_lot_size: I80F48::from_num(10_000_000),
            quote_unit: I80F48::from_num(1_000_000), // USD 6 decimals
            quote_lot_size: I80F48::from_num(100),
            taker_fee: I80F48::from_num(0.000_5),
        }
    }

    fn mocked_order(perp_info: &PerpInfo, price: f64, side: Side) -> UxdResult<Order> {
        let price_lot = price_to_lot_price(I80F48::from_num(price), perp_info)?;
        Ok(Order {
            quantity: 0,               // whatever not used
            price: price_lot.to_num(), // exact price
            size: 0,                   // whatever not used
            side,
        })
    }

    mod check_effective_order_price_versus_limit_price_suite {
        use super::*;
        use crate::SLIPPAGE_BASIS;

        mod mint_suite {
            use crate::{
                error::{SourceFileId, UxdError, UxdErrorCode},
                mango_utils::{check_effective_order_price_versus_limit_price, limit_price},
            };

            use super::*;

            proptest! {
                /// Tests the price check after placing a Perp order for Minting UXD (Selling Perp to open the Short position)
                /// combinations with :
                ///      perp_price per base unit between 0$ and 100_000
                ///      order_price per base unit between 0$ and 100_000
                ///      slippage between 0.1% and 100%
                #[test]
                fn test_check_effective_order_price_versus_limit_price_bid(perp_price in 0.0f64..10f64, order_price in 0.0f64..10f64, slippage in 1..SLIPPAGE_BASIS) {
                    // Order.price must be below the perpInfo.price within slippage
                    let side = Side::Bid;
                    let perp_info = mocked_perp_info(perp_price);
                    let order = mocked_order(&perp_info, order_price, side).unwrap();

                    let limit_price: f64 = limit_price(I80F48::from_num(perp_price), slippage, side)?.to_num();
                    match check_effective_order_price_versus_limit_price(
                        &perp_info,
                        &order,
                        slippage,
                    ) {
                        Ok(_) => {
                            prop_assert!(order_price >= limit_price);
                        },
                        Err(error) => {
                            match error {
                                UxdError::ProgramError(_) => prop_assert!(false),
                                UxdError::UxdErrorCode { uxd_error_code, line: _, source_file_id } => {
                                    prop_assert_eq!(source_file_id, SourceFileId::MangoUtilsLimitUtils);
                                    match uxd_error_code {
                                        UxdErrorCode::MathError => prop_assert!(false),
                                        UxdErrorCode::SlippageReached => {
                                            prop_assert!(order_price < limit_price);
                                        },
                                        _default => prop_assert!(false)
                                    }
                                }
                            }
                        }
                    }
                }
            }

            mod non_regression {
                use crate::mango_utils::check_effective_order_price_versus_limit_price;

                use super::*;

                #[test]
                pub fn test_valid_mint_small_slippage() {
                    // Order.price must be below the perpInfo.price within slippage
                    let perp_info = mocked_perp_info(0.09000);
                    let order = mocked_order(&perp_info, 0.09000, Side::Bid).unwrap();
                    let ret = check_effective_order_price_versus_limit_price(
                        &perp_info, &order, 1, // 0.1%
                    );
                    assert!(ret.is_ok());
                }

                #[test]
                pub fn test_valid_mint() {
                    let perp_info = mocked_perp_info(0.09000);
                    let order = mocked_order(&perp_info, 0.08911, Side::Bid).unwrap();
                    let ret = check_effective_order_price_versus_limit_price(
                        &perp_info, &order, 10, // 1%
                    );
                    assert!(ret.is_ok());
                }

                #[test]
                pub fn test_invalid_mint() {
                    let perp_info = mocked_perp_info(0.09000);
                    let order = mocked_order(&perp_info, 0.08909, Side::Bid).unwrap();
                    let ret = check_effective_order_price_versus_limit_price(
                        &perp_info, &order, 10, // 1%
                    );
                    assert!(ret.is_err());
                }
            }
        }

        mod redeem_suite {
            use crate::{
                error::{SourceFileId, UxdError, UxdErrorCode},
                mango_utils::{check_effective_order_price_versus_limit_price, limit_price},
            };

            use super::*;

            proptest! {
                /// Tests the price check after placing a Perp order for Redeeming UXD (Buying Perp to close the outstanding Short position)
                /// combinations with :
                ///      perp_price per base unit between 0$ and 100_000
                ///      order_price per base unit between 0$ and 100_000
                ///      slippage between 0.1% and 100%
                #[test]
                fn test_check_effective_order_price_versus_limit_price_ask(perp_price in 0.0f64..10f64, order_price in 0.0f64..10f64, slippage in 1..SLIPPAGE_BASIS) {
                    let side = Side::Ask;
                    let perp_info = mocked_perp_info(perp_price);
                    let order = mocked_order(&perp_info, order_price, side).unwrap();

                    let limit_price: f64 = limit_price(I80F48::from_num(perp_price), slippage, side)?.to_num();
                    match check_effective_order_price_versus_limit_price(
                        &perp_info,
                        &order,
                        slippage,
                    ) {
                        Ok(_) => {
                            prop_assert!(order_price <= limit_price);
                        },
                        Err(error) => {
                            match error {
                                UxdError::ProgramError(_) => prop_assert!(false),
                                UxdError::UxdErrorCode { uxd_error_code, line: _, source_file_id } => {
                                    prop_assert_eq!(source_file_id, SourceFileId::MangoUtilsLimitUtils);
                                    match uxd_error_code {
                                        UxdErrorCode::MathError => prop_assert!(false),
                                        UxdErrorCode::SlippageReached => {
                                            prop_assert!(order_price > limit_price);
                                        },
                                        _default => prop_assert!(false)
                                    }
                                }
                            }
                        }
                    }
                }
            }

            mod non_regression {
                use crate::mango_utils::check_effective_order_price_versus_limit_price;

                use super::*;

                #[test]
                pub fn test_valid_redeem_small_slippage() {
                    let perp_info = mocked_perp_info(0.09000);
                    let order = mocked_order(&perp_info, 0.09000, Side::Ask).unwrap();
                    let ret = check_effective_order_price_versus_limit_price(
                        &perp_info, &order, 1, // 0.1%
                    );
                    assert!(ret.is_ok());
                }

                #[test]
                pub fn test_valid_redeem() {
                    let perp_info = mocked_perp_info(0.09000);
                    let order = mocked_order(&perp_info, 0.09089, Side::Ask).unwrap();
                    let ret = check_effective_order_price_versus_limit_price(
                        &perp_info, &order, 10, // 1%
                    );
                    assert!(ret.is_ok());
                }

                #[test]
                pub fn test_invalid_redeem() {
                    let perp_info = mocked_perp_info(0.09000);
                    let order = mocked_order(&perp_info, 0.09091, Side::Ask).unwrap();
                    let ret = check_effective_order_price_versus_limit_price(
                        &perp_info, &order, 10, // 1%
                    );
                    assert!(ret.is_err());
                }
            }
        }
    }

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
                let price_sub_slippage = fractional_price.checked_sub(slippage_amount).unwrap();

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
                let price_add_slippage = fractional_price.checked_add(slippage_amount).unwrap();

                prop_assert_eq!(limit_price, price_add_slippage);
            }
        }
    }
}
