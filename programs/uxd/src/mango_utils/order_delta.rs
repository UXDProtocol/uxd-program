use super::PerpInfo;
use crate::error::UxdError;
use crate::mango_utils::total_perp_base_lot_position;
use anchor_lang::prelude::*;
use fixed::types::I80F48;
use mango::state::PerpAccount;

/// In native units
#[derive(Debug)]
pub struct OrderDelta {
    pub base: I80F48,
    // Including fees
    pub quote: I80F48,
    pub fee: I80F48,
}

// Quote delta between two states of perp account
pub fn quote_delta(
    pre_pa: &PerpAccount,
    post_pa: &PerpAccount,
    quote_lot_size: I80F48,
) -> Result<I80F48> {
    let pre_taker_quote = I80F48::from_num(pre_pa.taker_quote);
    let post_taker_quote = I80F48::from_num(post_pa.taker_quote);
    let quote_lot_delta = pre_taker_quote
        .checked_dist(post_taker_quote)
        .ok_or_else(|| error!(UxdError::MathError))?;
    quote_lot_delta
        .checked_mul(quote_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))
}

// Quote delta between two states of perp account
pub fn base_delta(
    pre_pa: &PerpAccount,
    post_pa: &PerpAccount,
    base_lot_size: I80F48,
) -> Result<I80F48> {
    let pre_base_lot_position = total_perp_base_lot_position(pre_pa)?;
    let post_base_lot_position = total_perp_base_lot_position(post_pa)?;
    let base_lot_delta = pre_base_lot_position
        .checked_dist(post_base_lot_position)
        .ok_or_else(|| error!(UxdError::MathError))?;
    base_lot_delta
        .checked_mul(base_lot_size)
        .ok_or_else(|| error!(UxdError::MathError))
}

// returns the amount of taker_fee paid for trading raw_quote_amount (rounded up)
pub fn taker_fee_amount_ceil(raw_quote_amount: I80F48, taker_fee: I80F48) -> Result<I80F48> {
    raw_quote_amount
        .checked_mul(taker_fee)
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_ceil()
        .ok_or_else(|| error!(UxdError::MathError))
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
) -> Result<OrderDelta> {
    let quote_delta = quote_delta(pre_pa, post_pa, perp_info.quote_lot_size)?;
    // Quote amount from an order cannot be 0 at this stage
    if quote_delta.is_zero() {
        return Err(error!(UxdError::InvalidQuoteDelta));
    }
    let fee_delta = taker_fee_amount_ceil(quote_delta, perp_info.effective_fee)?;
    let base_delta = base_delta(pre_pa, post_pa, perp_info.base_lot_size)?;

    Ok(OrderDelta {
        base: base_delta,
        quote: quote_delta,
        fee: fee_delta,
    })
}
