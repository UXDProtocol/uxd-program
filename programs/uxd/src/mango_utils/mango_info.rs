use anchor_lang::prelude::{AccountInfo, ProgramError, Pubkey};
use fixed::types::I80F48;
use mango::{
    error::MangoResult,
    state::{MangoAccount, MangoCache, MangoGroup},
};

pub struct MangoInfo {
    pub mango_group: MangoGroup,
    pub mango_account: MangoAccount,
    pub mango_cache: MangoCache,
}

impl MangoInfo {
    pub fn get_native_deposit_of_mint(&self, mint_pk: &Pubkey) -> MangoResult<I80F48> {
        let token_index = match self.mango_group.find_token_index(mint_pk) {
            None => return Ok(I80F48::ZERO),
            Some(i) => i,
        };
        let root_bank_cache = self.mango_cache.root_bank_cache[token_index];
        // If the user's token deposit is non-zero then the rootbank cache will be checked already in `place_perp_order`.
        // If it's zero then cache may be out of date, but it doesn't matter because 0 * index = 0
        self.mango_account
            .get_native_deposit(&root_bank_cache, token_index)
    }

    pub fn init(
        mango_group_ai: &AccountInfo,
        mango_cache_ai: &AccountInfo,
        mango_account_ai: &AccountInfo,
        mango_group_key: &Pubkey,
        mango_program_key: &Pubkey,
    ) -> Result<Self, ProgramError> {
        let mango_group = MangoGroup::load_checked(mango_group_ai, mango_program_key)
            .map_err(|me| ProgramError::from(me))?;
        let mango_cache = MangoCache::load_checked(mango_cache_ai, mango_program_key, &mango_group)
            .map_err(|me| ProgramError::from(me))?;
        let mango_account =
            MangoAccount::load_checked(mango_account_ai, mango_program_key, mango_group_key)
                .map_err(|me| ProgramError::from(me))?;
        Ok(MangoInfo {
            mango_group: *mango_group,
            mango_account: *mango_account,
            mango_cache: *mango_cache,
        })
    }
}
