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

// Note : removes the taker fees from the redeemable_delta.
//  The fees are not reflected right away in the PerpAccount (uncommitted changes), so we do it manually.
//  Mango system needs to call (after this ix, by the user or anyone) the consumeEvents ix, that will process the `fillEvent` in that case
//  and update all mango internals / resolve the unsettled balance change, and process fees.
//  The amount minted/redeemed offsets accordingly to reflect that change that will be settled in the future.
pub fn derive_order_delta(
    pre_pa: &PerpAccount,
    post_pa: &PerpAccount,
    perp_info: &PerpInfo,
) -> UxdResult<OrderDelta> {
    // [QUOTE]
    // The order delta in quote lot
    let pre_taker_quote = I80F48::from_num(pre_pa.taker_quote);
    let post_taker_quote = I80F48::from_num(post_pa.taker_quote);
    let quote_lot_delta = pre_taker_quote.dist(post_taker_quote);

    check!(
        !quote_lot_delta.is_zero(),
        UxdErrorCode::InvalidQuoteLotDelta
    )?;
    let quote_delta = I80F48::from_num(quote_lot_delta)
        .checked_mul(perp_info.quote_lot_size)
        .ok_or(math_err!())?;

    // [QUOTE FEES] (Rounded UP)
    let fee_amount = quote_delta
        .checked_mul(perp_info.taker_fee)
        .ok_or(math_err!())?
        .checked_ceil()
        .ok_or(math_err!())?;

    let fee_delta = fee_amount.checked_to_num().ok_or(math_err!())?;

    // [BASE]
    let pre_base_lot_position = I80F48::from_num(total_perp_base_lot_position(pre_pa)?);
    let post_base_lot_position = I80F48::from_num(total_perp_base_lot_position(post_pa)?);
    let base_lot_delta = pre_base_lot_position.dist(post_base_lot_position);

    let collateral_delta = base_lot_delta
        .checked_mul(perp_info.base_lot_size)
        .ok_or(math_err!())?
        .checked_to_num()
        .ok_or(math_err!())?;

    Ok(OrderDelta {
        collateral: collateral_delta,
        quote: quote_delta.checked_to_num().ok_or(math_err!())?,
        fee: fee_delta,
    })
}

// Todo tests