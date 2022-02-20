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

#[derive(Debug)]
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
    I80F48::from_num(quote_lot_delta)
        .checked_mul(quote_lot_size)
        .ok_or(math_err!())
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
    base_lot_delta.checked_mul(base_lot_size).ok_or(math_err!())
}

// returns the amount of taker_fee paid for trading raw_quote_amount (rounded up)
pub fn taker_fee_amount_ceil(raw_quote_amount: I80F48, taker_fee: I80F48) -> UxdResult<I80F48> {
    raw_quote_amount
        .checked_mul(taker_fee)
        .ok_or(math_err!())?
        .checked_ceil()
        .ok_or(math_err!())
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
    let quote_delta = quote_delta(pre_pa, post_pa, perp_info.quote_lot_size)?;
    // Quote amount from an order cannot be 0 at this stage
    check!(!quote_delta.is_zero(), UxdErrorCode::InvalidQuoteDelta)?;
    // Note : Will keep the current way of calculating, but here quote_position delta would work
    let fee_delta = taker_fee_amount_ceil(quote_delta, perp_info.effective_fee)?;
    let base_delta = base_delta(pre_pa, post_pa, perp_info.base_lot_size)?;

    Ok(OrderDelta {
        collateral: base_delta.checked_to_num().ok_or(math_err!())?,
        quote: quote_delta.checked_to_num().ok_or(math_err!())?,
        fee: fee_delta.checked_to_num().ok_or(math_err!())?,
    })
}
