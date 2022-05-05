use anchor_lang::prelude::Pubkey;
use fixed::types::I80F48;
use mango::{state::{MangoGroup, MangoCache, MangoAccount}, error::MangoResult};

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