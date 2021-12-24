// use super::PerpInfo;
// use fixed::types::I80F48;
use mango::state::PerpAccount;

pub fn unsettled_base_amount(perp_account: &PerpAccount) -> i64 {
    perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .unwrap()
}

// pub fn unconsumed_quote_amount(perp_account: &PerpAccount, perp_info: &PerpInfo) -> i64 {
//     // Over checked, OPT later
//     let taker_quote = I80F48::from_num(perp_account.taker_quote);
//     let quote_position = I80F48::from_num(perp_account.quote_position);
//     let taker_quote_pending_fees = taker_quote.checked_mul(perp_info.taker_fee).unwrap();
//     let taker_quote_minus_fees = taker_quote.checked_sub(taker_quote_pending_fees).unwrap();
//     return quote_position
//         .checked_add(taker_quote_minus_fees)
//         .unwrap()
//         .checked_to_num()
//         .unwrap();
// }
