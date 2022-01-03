use super::PerpInfo;
use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::UxdResult;
use fixed::types::I80F48;
use mango::state::PerpAccount;

declare_check_assert_macros!(SourceFileId::MangoUtilsPerpAccountUtils);

// Return the base position + the amount that's on EventQueue waiting to be processed
pub fn total_perp_base_lot_position(perp_account: &PerpAccount) -> UxdResult<i64> {
    Ok(perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .ok_or(math_err!())?)
}

// Return the quote position + the amount that's on EventQueue waiting to be processed (minus fees)
pub fn total_perp_quote_position(
    perp_account: &PerpAccount,
    perp_info: &PerpInfo,
) -> UxdResult<i64> {
    let taker_quote = I80F48::from_num(perp_account.taker_quote)
        .checked_mul(perp_info.quote_lot_size)
        .ok_or(math_err!())?;
    let fee_amount = taker_quote
        .abs()
        .checked_mul(perp_info.taker_fee)
        .ok_or(math_err!())?;
    let quote_change = taker_quote.checked_sub(fee_amount).ok_or(math_err!())?;
    let total_quote_position = perp_account
        .quote_position
        .checked_add(quote_change)
        .ok_or(math_err!())?;
    Ok(total_quote_position.checked_to_num().ok_or(math_err!())?)
}
