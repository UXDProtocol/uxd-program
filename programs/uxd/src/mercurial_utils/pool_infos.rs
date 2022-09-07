use crate::{error::UxdError, utils};
use anchor_lang::prelude::{Account, Clock, SolanaSysvar};
use anchor_spl::token::{Mint, TokenAccount};
use fixed::types::I80F48;

pub struct MercurialPoolInfos {
    pub pool_token_a_base_amount: I80F48,
    pub pool_token_b_base_amount: I80F48,
    pub pool_base_value: I80F48,
    pub pool_lp_mint_base_supply: I80F48,
    pub one_lp_token_base_value: I80F48,
}

impl std::fmt::Display for MercurialPoolInfos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Mercurial pool info {{pool_token_a_base_amount: {}, pool_token_b_base_amount: {}, pool_base_value: {}, pool_lp_mint_base_supply: {}, one_lp_token_base_value: {}}}",
            self.pool_token_a_base_amount,
            self.pool_token_b_base_amount,
            self.pool_base_value,
            self.pool_lp_mint_base_supply,
            self.one_lp_token_base_value,
        ))
    }
}

// Returns the number of token a and token b owned by the pool in base unit
fn get_pool_token_base_amounts(
    vault_a: Account<mercurial_vault::state::Vault>,
    vault_b: Account<mercurial_vault::state::Vault>,
    vault_a_lp: Account<TokenAccount>,
    vault_a_lp_mint: Account<Mint>,
    vault_b_lp: Account<TokenAccount>,
    vault_b_lp_mint: Account<Mint>,
    token_a_mint: Account<Mint>,
    token_b_mint: Account<Mint>,
) -> Result<(I80F48, I80F48), UxdError> {
    let clock = Clock::get().ok().ok_or(UxdError::ClockError)?;

    let current_time = u64::try_from(clock.unix_timestamp)
        .ok()
        .ok_or(UxdError::MathError)?;

    let token_a_amount = vault_a
        .get_amount_by_share(current_time, vault_a_lp.amount, vault_a_lp_mint.supply)
        .ok_or(UxdError::MathError)?;

    let token_b_amount = vault_b
        .get_amount_by_share(current_time, vault_b_lp.amount, vault_b_lp_mint.supply)
        .ok_or(UxdError::MathError)?;

    let token_a_base_amount = utils::native_to_base(
        I80F48::checked_from_num(
            I80F48::checked_from_num(token_a_amount).ok_or_else(|| UxdError::MathError)?,
        )
        .ok_or_else(|| UxdError::MathError)?,
        token_a_mint.decimals,
    )?;

    let token_b_base_amount = utils::native_to_base(
        I80F48::checked_from_num(
            I80F48::checked_from_num(token_b_amount).ok_or_else(|| UxdError::MathError)?,
        )
        .ok_or_else(|| UxdError::MathError)?,
        token_b_mint.decimals,
    )?;

    Ok((token_a_base_amount, token_b_base_amount))
}

// Calculate the dollar value of the pool in base units
// We consider the pool to be constituted of stablecoin only
fn calculate_pool_base_value(
    base_pool_token_a_amount: I80F48,
    base_pool_token_b_amount: I80F48,
) -> Result<I80F48, UxdError> {
    let base_pool_value = base_pool_token_a_amount
        .checked_add(base_pool_token_b_amount)
        .ok_or_else(|| UxdError::MathError)?;

    Ok(base_pool_value)
}

// Generates mercurial pool infos based on Account states
impl MercurialPoolInfos {
    pub fn new(
        vault_a: Account<mercurial_vault::state::Vault>,
        vault_b: Account<mercurial_vault::state::Vault>,
        vault_a_lp: Account<TokenAccount>,
        vault_a_lp_mint: Account<Mint>,
        vault_b_lp: Account<TokenAccount>,
        vault_b_lp_mint: Account<Mint>,
        lp_mint: Account<Mint>,
        token_a_mint: Account<Mint>,
        token_b_mint: Account<Mint>,
    ) -> Result<Self, UxdError> {
        let (pool_token_a_base_amount, pool_token_b_base_amount) = get_pool_token_base_amounts(
            vault_a,
            vault_b,
            vault_a_lp,
            vault_a_lp_mint,
            vault_b_lp,
            vault_b_lp_mint,
            token_a_mint,
            token_b_mint,
        )?;

        let pool_base_value =
            calculate_pool_base_value(pool_token_a_base_amount, pool_token_b_base_amount)?;

        let pool_lp_mint_base_supply = utils::native_to_base(
            I80F48::checked_from_num(lp_mint.supply).ok_or_else(|| UxdError::MathError)?,
            lp_mint.decimals,
        )?;

        let one_lp_token_base_value = pool_base_value
            .checked_div(pool_lp_mint_base_supply)
            .ok_or_else(|| UxdError::MathError)?;

        Ok(MercurialPoolInfos {
            pool_token_a_base_amount,
            pool_token_b_base_amount,
            pool_base_value,
            pool_lp_mint_base_supply,
            one_lp_token_base_value,
        })
    }

    // Take in an LP token amount and return its value in base unit
    pub fn calculate_pool_lp_token_base_value(
        &self,
        lp_token_amount: I80F48,
    ) -> Result<I80F48, UxdError> {
        let lp_token_base_value = lp_token_amount
            .checked_mul(self.one_lp_token_base_value)
            .ok_or_else(|| UxdError::MathError)?;

        Ok(lp_token_base_value)
    }
}
