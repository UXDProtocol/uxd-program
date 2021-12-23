use super::PerpInfo;
use fixed::types::I80F48;
use mango::state::PerpAccount;
use solana_program::msg;

pub struct OrderDelta {
    pub collateral: u64,
    pub redeemable: u64,
    pub fee: u64,
}

// Note : removes the taker fees from the redeemable_delta.
//  The fees are not reflected right away in the PerpAccount (uncommitted changes).
//  Mango system needs to call (after this ix, by the user or anyone) the consumeEvents ix, that will process the `fillEvent` in that case
//  and update all mango internals / resolve the uncommitted balance change, and process fees.
//  The amount minted/redeemed offsets accordingly to reflect that change that will be settled in the future.
pub fn derive_order_delta(perp_account: &PerpAccount, perp_info: &PerpInfo) -> OrderDelta {
    let order_value = I80F48::checked_from_num(perp_account.taker_quote)
        .unwrap()
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    // Rounded UP
    let fee_amount = order_value
        .checked_mul(perp_info.taker_fee)
        .unwrap()
        .checked_ceil()
        .unwrap()
        .checked_abs()
        .unwrap();
    let collateral_amount =
        I80F48::checked_from_num(perp_account.taker_base.checked_abs().unwrap())
            .unwrap()
            .checked_mul(perp_info.base_lot_size)
            .unwrap();
    let collateral_delta = collateral_amount.checked_to_num().unwrap();
    let redeemable_delta = order_value
        .checked_sub(fee_amount)
        .unwrap()
        .checked_abs()
        .unwrap()
        .checked_to_num()
        .unwrap();
    let fee_delta = fee_amount.checked_to_num().unwrap();

    msg!("collateral_delta {}", collateral_delta);
    msg!("redeemable_delta {}", redeemable_delta);
    msg!("fee_delta {}", fee_delta);

    OrderDelta {
        collateral: collateral_delta,
        redeemable: redeemable_delta,
        fee: fee_delta,
    }
}
