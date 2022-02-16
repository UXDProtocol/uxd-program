use super::PerpInfo;
use crate::declare_check_assert_macros;
use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::mango_utils::total_perp_base_lot_position;
use crate::UxdResult;
use fixed::types::I80F48;
use mango::state::PerpAccount;

declare_check_assert_macros!(SourceFileId::MangoUtilsOrderDelta);

pub struct OrderDelta {
    pub collateral: u64,
    pub quote: u64,
    pub fee: u64,
}

// Quote delta between two states of perp account
pub fn quote_delta(
    pre_pa: &PerpAccount,
    post_pa: &PerpAccount,
    quote_lot_size: I80F48,
) -> UxdResult<I80F48> {
    let pre_taker_quote = I80F48::from_num(pre_pa.taker_quote);
    let post_taker_quote = I80F48::from_num(post_pa.taker_quote);
    let quote_lot_delta = pre_taker_quote.dist(post_taker_quote);
    Ok(I80F48::from_num(quote_lot_delta)
        .checked_mul(quote_lot_size)
        .ok_or(math_err!())?)
}

// Quote delta between two states of perp account
pub fn base_delta(
    pre_pa: &PerpAccount,
    post_pa: &PerpAccount,
    base_lot_size: I80F48,
) -> UxdResult<I80F48> {
    let pre_base_lot_position = I80F48::from_num(total_perp_base_lot_position(pre_pa)?);
    let post_base_lot_position = I80F48::from_num(total_perp_base_lot_position(post_pa)?);
    let base_lot_delta = pre_base_lot_position.dist(post_base_lot_position);
    Ok(base_lot_delta
        .checked_mul(base_lot_size)
        .ok_or(math_err!())?)
}

// returns the amount of taker_fee paid for trading raw_quote_amount (rounded up)
pub fn taker_fee_amount_ceil(raw_quote_amount: I80F48, taker_fee: I80F48) -> UxdResult<I80F48> {
    Ok(raw_quote_amount
        .checked_mul(taker_fee)
        .ok_or(math_err!())?
        .checked_ceil()
        .ok_or(math_err!())?)
}

// Note : removes the taker fees from the redeemable_delta.
//  The fees are not reflected right away in the PerpAccount (uncommitted changes), so we do it manually.
//  Mango system needs to call (after this ix, by the user or anyone) the consumeEvents ix, that will process the `fillEvent` in that case
//  and update all mango internals / resolve the unsettled balance change, and process fees.
//  The amount minted/redeemed offsets accordingly to reflect that change that will be settled in the future.
// MangoMarkets v3.3.5 : Fees are not reflected directly in the quote_position, still not in the taker_quote
pub fn derive_order_delta(
    pre_pa: &PerpAccount,
    post_pa: &PerpAccount,
    perp_info: &PerpInfo,
) -> UxdResult<OrderDelta> {
    let quote_delta = quote_delta(&pre_pa, &post_pa, perp_info.quote_lot_size)?;
    // Quote amount from an order cannot be 0 at this stage
    check!(!quote_delta.is_zero(), UxdErrorCode::InvalidQuoteDelta)?;
    // Note : Will keep the current way of calculating, but here quote_position delta would work
    let fee_delta = taker_fee_amount_ceil(quote_delta, perp_info.taker_fee)?;
    let base_delta = base_delta(&pre_pa, &post_pa, perp_info.base_lot_size)?;

    Ok(OrderDelta {
        collateral: base_delta.checked_to_num().ok_or(math_err!())?,
        quote: quote_delta.checked_to_num().ok_or(math_err!())?,
        fee: fee_delta.checked_to_num().ok_or(math_err!())?,
    })
}

// Non regression tests
#[cfg(test)]
mod tests {

    use super::*;
    use fixed::types::I80F48;
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
                    let expected_quote_delta = I80F48::from_num(taker_quote).dist(I80F48::from_num(taker_quote_post)).checked_mul(quote_lot_size).unwrap();
                    prop_assert_eq!(quote_delta, expected_quote_delta)
                },
                Err(error) => {
                match error {
                        UxdError::ProgramError(_) => todo!(),
                        UxdError::UxdErrorCode { uxd_error_code, line: _, source_file_id } => {
                            prop_assert_eq!(source_file_id, SourceFileId::MangoUtilsLimitUtils);
                            match uxd_error_code {
                                UxdErrorCode::MathError => prop_assert!(true),
                                _default => prop_assert!(false)
                            }
                        }
                    }
                }
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
                    let expected_base_delta = total_base_pre.dist(I80F48::from_num(total_base_post)).checked_mul(base_lot_size).unwrap();
                    prop_assert_eq!(base_delta, expected_base_delta)
                },
                Err(error) => {
                match error {
                        UxdError::ProgramError(_) => todo!(),
                        UxdError::UxdErrorCode { uxd_error_code, line: _, source_file_id } => {
                            prop_assert_eq!(source_file_id, SourceFileId::MangoUtilsLimitUtils);
                            match uxd_error_code {
                                UxdErrorCode::MathError => prop_assert!(true),
                                _default => prop_assert!(false)
                            }
                        }
                    }
                }
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
                  Ok(taker_fee_amount) => prop_assert_eq!(taker_fee_amount, I80F48::from_num(raw_quote_amount).checked_mul(I80F48::from_num(taker_fee)).unwrap().ceil()),
                  Err(_) => todo!(),
                }
            }
    }
}
