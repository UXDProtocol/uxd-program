use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::UxdResult;
use mango::state::PerpAccount;

declare_check_assert_macros!(SourceFileId::MangoUtilsPerpAccountUtils);

// Return the base position + the amount that's on EventQueue waiting to be processed
pub fn total_perp_base_lot_position(perp_account: &PerpAccount) -> UxdResult<i64> {
    perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .ok_or(math_err!())
}