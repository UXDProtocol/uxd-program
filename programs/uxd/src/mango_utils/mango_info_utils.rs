use anchor_lang::prelude::{ProgramError, Pubkey};
use fixed::types::I80F48;
use mango::state::CENTIBPS_PER_UNIT;
use mango::{
    error::MangoResult,
    ids::mngo_token,
    state::{MangoAccount, MangoCache, MangoGroup},
};

pub fn get_native_deposit(
    mint_pk: &Pubkey,
    mango_group: &MangoGroup,
    mango_cache: &MangoCache,
    mango_account: &MangoAccount,
) -> MangoResult<I80F48> {
    let token_index = match mango_group.find_token_index(mint_pk) {
        None => return Ok(I80F48::ZERO),
        Some(i) => i,
    };
    let root_bank_cache = mango_cache.root_bank_cache[token_index];
    // If the user's token deposit is non-zero then the rootbank cache will be checked already in `place_perp_order`.
    // If it's zero then cache may be out of date, but it doesn't matter because 0 * index = 0
    mango_account.get_native_deposit(&root_bank_cache, token_index)
}

// Check for Ref fees (fees added on mango v3.3.5)
// Referral fees
// If you hold 10k MNGO or more in your MangoAccount as of 02/20/2022, you skip this fee.
pub fn determine_ref_fee(
    mango_group: &MangoGroup,
    mango_cache: &MangoCache,
    mango_account: &MangoAccount,
) -> Result<I80F48, ProgramError> {
    let mngo_deposits =
        get_native_deposit(&mngo_token::id(), mango_group, mango_cache, mango_account)
            .map_err(|me| ProgramError::from(me))?;
    let ref_mngo_req = I80F48::from_num(mango_group.ref_mngo_required);
    if mngo_deposits >= ref_mngo_req {
        return Ok(I80F48::ZERO);
    }
    Ok(I80F48::from_num(mango_group.ref_share_centibps) / CENTIBPS_PER_UNIT)
}
