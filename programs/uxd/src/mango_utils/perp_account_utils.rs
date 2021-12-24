use mango::state::PerpAccount;

// Return the current uncommitted base position for a given PerpAccount
pub fn unprocessed_perp_base_position(perp_account: &PerpAccount) -> i64 {
    perp_account
        .base_position
        .checked_add(perp_account.taker_base)
        .unwrap()
}
