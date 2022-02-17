// Unit Test
#[cfg(test)]
mod test_order {

    use crate::{
        error::{SourceFileId, UxdError, UxdErrorCode},
        mango_utils::check_perp_order_fully_filled,
    };

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_check_perp_order_fully_filled(order_quantity in i64::MIN..i64::MAX, pre_position in i64::MIN..i64::MAX, post_position in i64::MIN..i64::MAX) {
            let res = check_perp_order_fully_filled(order_quantity, pre_position, post_position);
            // MangoMarket.place_perp_order take quantity as i64
            let order_quantity: u64 = order_quantity.abs().try_into().unwrap();
            match res {
                Ok(()) => {
                    prop_assert_eq!(order_quantity, pre_position.abs_diff(post_position));
                }
                Err(error) => {
                    match error {
                         UxdError::ProgramError(_) => prop_assert!(false),
                         UxdError::UxdErrorCode { uxd_error_code, line: _, source_file_id } => {
                            prop_assert_eq!(source_file_id, SourceFileId::MangoUtilsOrder);
                            match uxd_error_code {
                                UxdErrorCode::PerpOrderPartiallyFilled => prop_assert_ne!(order_quantity, pre_position.abs_diff(post_position)),
                                UxdErrorCode::MathError => prop_assert!(true),
                                _default => prop_assert!(false)
                            };
                         },
                    }
                }
            };
        }
    }
}
