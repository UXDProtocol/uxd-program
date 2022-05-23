
#[cfg(test)]
mod test_quote_amounts {
    use std::ops::Neg;

    use fixed::types::I80F48;
    use proptest::prelude::*;

    use crate::instructions::{calculate_quote_mintable, calculate_quote_redeemable};

    proptest! {
        #[test]
        fn test_quote_mintable(
            perp_unrealized_pnl in (i128::from(u64::MAX) / 2).neg()..-1i128,
            quote_minted in (i128::from(u64::MAX) / 2).neg()..(i128::from(u64::MAX) / 2)
        ) {
            let perp_unrealized_pnl = I80F48::from_num(perp_unrealized_pnl);
            match calculate_quote_mintable(perp_unrealized_pnl, quote_minted) {
                Ok(quote_mintable) => {
                    let expected_quote_mintable = perp_unrealized_pnl.checked_sub(I80F48::from_num(quote_minted)).unwrap().checked_abs().unwrap().checked_to_num::<u64>().unwrap();
                    prop_assert_eq!(quote_mintable, expected_quote_mintable)
                },
                Err(_error) => prop_assert!(false)
            }
        }
    }

    proptest! {
        #[test]
        fn test_quote_redeemable(
            perp_unrealized_pnl in 1i128..(i128::from(u64::MAX) / 2),
            quote_minted in 1i128..(i128::from(u64::MAX) / 2),
        ) {
            let perp_unrealized_pnl = I80F48::from_num(perp_unrealized_pnl);
            match calculate_quote_redeemable(perp_unrealized_pnl, quote_minted) {
                Ok(quote_redeemable) => {
                    let expected_quote_redeemable = perp_unrealized_pnl.checked_abs().unwrap().checked_add(I80F48::from_num(quote_minted)).unwrap().checked_to_num::<u64>().unwrap();
                    prop_assert_eq!(quote_redeemable, expected_quote_redeemable)
                },
                Err(_error) => prop_assert!(false)
            }
        }
    }
}
