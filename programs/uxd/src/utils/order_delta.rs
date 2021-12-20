use super::PerpInfo;
use fixed::types::I80F48;
use mango::state::PerpAccount;
use solana_program::msg;

pub struct OrderDelta {
    pub collateral: u64,
    pub redeemable: u64,
    pub fee: u64,
}

// Note : remove the taker fees from the redeemable_delta.
//  They are not registered in mango unless the EventQueue fill event is consumed (later on)
//  The amount minted/redeemed is offseted accordingly to reflect that change that will be settled in the future.
pub fn derive_order_delta(perp_account: &PerpAccount, perp_info: &PerpInfo) -> OrderDelta {
    let collateral_delta = I80F48::checked_from_num(perp_account.taker_base.checked_abs().unwrap())
        .unwrap()
        .checked_mul(perp_info.base_lot_size)
        .unwrap()
        .checked_to_num()
        .unwrap();
    let order_amount = I80F48::checked_from_num(perp_account.taker_quote.checked_abs().unwrap())
        .unwrap()
        .checked_mul(perp_info.quote_lot_size)
        .unwrap();
    // Rounded UP
    let fee_amount = order_amount
        .checked_mul(perp_info.taker_fee)
        .unwrap()
        .checked_ceil()
        .unwrap();
    let redeemable_delta = order_amount
        .checked_sub(fee_amount)
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
