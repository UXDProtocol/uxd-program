// Unit Test
#[cfg(test)]
mod test_perp_account_utils {

    use crate::{mango_utils::total_perp_base_lot_position};

    use fixed::types::I80F48;
    use mango::state::PerpAccount;
    use proptest::prelude::*;

    fn mocked_perp_account(taker_base: i64, base_position: i64) -> PerpAccount {
        PerpAccount {
            base_position,
            quote_position: I80F48::ZERO,
            long_settled_funding: I80F48::ZERO,
            short_settled_funding: I80F48::ZERO,
            bids_quantity: 0,
            asks_quantity: 0,
            taker_base,
            taker_quote: 0,
            mngo_accrued: 0,
        }
    }

    proptest! {
        #[test]
        fn test_total_perp_base_lot_position(taker_base in i64::MIN..i64::MAX, base_position in i64::MIN..i64::MAX) {
            let perp_account = mocked_perp_account(taker_base, base_position);
            let res = total_perp_base_lot_position(&perp_account);

            match res {
                Ok(total) => {
                    prop_assert_eq!(total, taker_base + base_position);
                }
                Err(_error) => prop_assert!(false)
            };
        }
    }
}
