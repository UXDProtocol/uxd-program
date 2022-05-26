// Non regression tests
#[cfg(test)]
mod test_order {

    use fixed::types::I80F48;
    use mango::state::PerpAccount;
    use proptest::prelude::*;

    fn mocked_perp_account(
        taker_base: i64,
        taker_quote: i64,
        base_position: i64,
        quote_position: i64,
    ) -> PerpAccount {
        PerpAccount {
            base_position,
            quote_position: I80F48::from(quote_position),
            long_settled_funding: I80F48::ZERO,
            short_settled_funding: I80F48::ZERO,
            bids_quantity: 0,
            asks_quantity: 0,
            taker_base,
            taker_quote,
            mngo_accrued: 0,
        }
    }

    use crate::mango_utils::{base_delta, quote_delta, taker_fee_amount_ceil};

    proptest! {
        #[test]
        fn test_quote_delta(
            taker_base in i32::MIN..i32::MAX,
            taker_quote in i32::MIN..i32::MAX,
            base_position in i32::MIN..i32::MAX,
            quote_position in i32::MIN..i32::MAX,
            taker_base_post in i32::MIN..i32::MAX,
            taker_quote_post in i32::MIN..i32::MAX,
            base_position_post in i32::MIN..i32::MAX,
            quote_position_post in i32::MIN..i32::MAX,
            quote_lot_size in 1..1_000_000_000i32
        ) {
            let quote_lot_size = I80F48::from_num(quote_lot_size);
            let pre_pa = mocked_perp_account(taker_base.into(), taker_quote.into(), base_position.into(), quote_position.into());
            let post_pa = mocked_perp_account(taker_base_post.into(), taker_quote_post.into(), base_position_post.into(), quote_position_post.into());
            match quote_delta(&pre_pa, &post_pa, quote_lot_size) {
                Ok(quote_delta) => {
                    let expected_quote_delta = I80F48::from_num(taker_quote_post).checked_sub(I80F48::from_num(taker_quote)).unwrap().checked_mul(quote_lot_size).unwrap();
                    prop_assert_eq!(quote_delta, expected_quote_delta)
                },
                Err(_error) => prop_assert!(false)
            }
        }
    }

    proptest! {
        #[test]
        fn test_base_delta(
            taker_base in i32::MIN..i32::MAX,
            taker_quote in i32::MIN..i32::MAX,
            base_position in i32::MIN..i32::MAX,
            quote_position in i32::MIN..i32::MAX,
            taker_base_post in i32::MIN..i32::MAX,
            taker_quote_post in i32::MIN..i32::MAX,
            base_position_post in i32::MIN..i32::MAX,
            quote_position_post in i32::MIN..i32::MAX,
            base_lot_size in 1..1_000_000_000i32
        ) {
            let base_lot_size = I80F48::from_num(base_lot_size);
            let pre_pa = mocked_perp_account(taker_base.into(), taker_quote.into(), base_position.into(), quote_position.into());
            let post_pa = mocked_perp_account(taker_base_post.into(), taker_quote_post.into(), base_position_post.into(), quote_position_post.into());
            match base_delta(&pre_pa, &post_pa, base_lot_size) {
                Ok(base_delta) => {
                    let total_base_pre = I80F48::from_num(taker_base) + I80F48::from_num(base_position);
                    let total_base_post = I80F48::from_num(taker_base_post) + I80F48::from_num(base_position_post);
                    let expected_base_delta = total_base_post.checked_sub(I80F48::from_num(total_base_pre)).unwrap().checked_mul(base_lot_size).unwrap();
                    prop_assert_eq!(base_delta, expected_base_delta)
                },
                Err(_error) => prop_assert!(false)
            }
        }
    }

    proptest! {
        #[test]
        fn test_taker_fee_amount_ceil(
            raw_quote_amount in i64::MIN..i64::MAX,
            taker_fee in 0.0000f64..0.001f64, // 0 bps to 10 bps
        ) {
            match taker_fee_amount_ceil(I80F48::from_num(raw_quote_amount), I80F48::from_num(taker_fee)) {
              Ok(taker_fee_amount) => prop_assert_eq!(taker_fee_amount, I80F48::from_num(raw_quote_amount).checked_mul(I80F48::from_num(taker_fee)).unwrap().ceil()),                  Err(_) => todo!(),
            }
        }
    }
}
