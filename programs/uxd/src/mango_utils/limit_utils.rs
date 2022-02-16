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
fn calculate_slippage_amount(price: I80F48, slippage: u32) -> UxdResult<I80F48> {
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

#[cfg(test)]
mod tests {

    use super::*;
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

        mod mint_suite {
            use super::*;

            proptest! {
                /// Tests the price check after placing a Perp order for Minting UXD (Selling Perp to open the Short position)
                /// combinations with :
                ///      perp_price between 0$ and 100_000
                ///      order_price between 0$ and 100_000
                ///      slippage between 0.01% and 100%
                #[test]
                fn proptest(perp_price in 0.0f64..10f64, order_price in 0.0f64..10f64, slippage in 1..SLIPPAGE_BASIS) {
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
                                        UxdErrorCode::MathError => prop_assert!(true),
                                        UxdErrorCode::SlippageReached => {
                                            prop_assert!(order_price <= limit_price);
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
            use super::*;

            proptest! {
                /// Tests the price check after placing a Perp order for Redeeming UXD (Buying Perp to close the outstanding Short position)
                /// combinations with :
                ///      perp_price between 0$ and 100_000
                ///      order_price between 0$ and 100_000
                ///      slippage between 0.01% and 100%
                #[test]
                fn proptest_redeem(perp_price in 0.0f64..10f64, order_price in 0.0f64..10f64, slippage in 1..SLIPPAGE_BASIS) {
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
                                        UxdErrorCode::MathError => prop_assert!(true),
                                        UxdErrorCode::SlippageReached => {
                                            prop_assert!(order_price >= limit_price);
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

    mod unit_tests {
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
