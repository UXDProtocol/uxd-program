use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::UxdResult;
use mango::state::PerpAccount;

declare_check_assert_macros!(SourceFileId::MangoUtilsPerpAccountUtils);

// Return the base position + the amount that's on EventQueue waiting to be processed
pub fn total_perp_base_lot_position(perp_account: &PerpAccount) -> UxdResult<i64> {
    Ok(perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .ok_or(math_err!())?)
}

// Unit Test
#[cfg(test)]
mod tests {

    use super::*;
    use fixed::types::I80F48;
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
                Err(error) => {
                    match error {
                        UxdError::ProgramError(_) => prop_assert!(false),
                        UxdError::UxdErrorCode { uxd_error_code, line: _, source_file_id } => {
                            prop_assert_eq!(source_file_id, SourceFileId::MangoUtilsPerpAccountUtils);
                            match uxd_error_code {
                                UxdErrorCode::MathError => prop_assert!(true),
                                _default => prop_assert!(false)
                            }
                        }
                    }
                }
            };
        }
    }
}
