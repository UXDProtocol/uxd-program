use anchor_lang::prelude::*;
use fixed::types::I80F48;
use mango::state::{MangoAccount, MangoCache, MangoGroup};

use crate::{error::UxdError, mango_utils::get_native_deposit};

#[derive(Debug)]
pub struct MsolInfo {
    pub native_mint_lamports: I80F48,
    pub msol_lamports: I80F48,
}

impl MsolInfo {
    pub fn new(
        mango_group_ai: &AccountInfo,
        mango_cache_ai: &AccountInfo,
        mango_account_ai: &AccountInfo,
        mango_group_key: &Pubkey,
        mango_program_key: &Pubkey,
        marinade_state: &Account<marinade_finance::state::State>,
        msol_mint: &Pubkey,
    ) -> Result<Self> {
        let mango_group = MangoGroup::load_checked(mango_group_ai, mango_program_key)
            .map_err(ProgramError::from)?;

        let mango_cache = MangoCache::load_checked(mango_cache_ai, mango_program_key, &mango_group)
            .map_err(ProgramError::from)?;

        let mango_account =
            MangoAccount::load_checked(mango_account_ai, mango_program_key, mango_group_key)
                .map_err(ProgramError::from)?;

        let depository_sol_lamports = get_native_deposit(
            &spl_token::native_mint::id(),
            &mango_group,
            &mango_cache,
            &mango_account,
        )
        .map_err(ProgramError::from)?;

        let depository_msol_amount =
            get_native_deposit(msol_mint, &mango_group, &mango_cache, &mango_account)
                .map_err(ProgramError::from)?
                .checked_to_num()
                .ok_or_else(|| error!(UxdError::MathError))?;

        let depository_msol_amount_lamports = I80F48::checked_from_num(
            // msol_amount * msol_price_in_sol
            marinade_state
                .calc_lamports_from_msol_amount(depository_msol_amount)
                .map_err(|me| ProgramError::from(me))?,
        )
        .ok_or_else(|| error!(UxdError::MathError))?;

        drop(mango_group);
        drop(mango_account);
        drop(mango_cache);

        Ok(MsolInfo {
            native_mint_lamports: depository_sol_lamports,
            msol_lamports: depository_msol_amount_lamports,
        })
    }
}

impl MsolInfo {
    pub fn total_depository_amount_lamports(&self) -> Result<I80F48> {
        self.native_mint_lamports
            .checked_add(self.msol_lamports)
            .ok_or_else(|| error!(UxdError::MathError))
    }

    // liquidity_ratio[t] = liquid_SOL[t]/(liquid_SOL[t] + marinade_SOL[t]*MSOL_underlying_SOL[t])
    pub fn liquidity_ratio(&self) -> Result<I80F48> {
        self.native_mint_lamports
            .checked_div(self.total_depository_amount_lamports()?)
            .ok_or_else(|| error!(UxdError::MathError))
    }
}
