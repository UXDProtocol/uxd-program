use crate::error::UxdError;
use crate::utils::checked_as_u64;
use anchor_lang::{
    prelude::{Account, Clock, SolanaSysvar},
    Result,
};
use mercurial_vault::state::Vault;

pub fn calculate_lp_tokens_value(
    mercurial_vault: &Account<Vault>,
    mercurial_vault_lp_mint_supply: u64,
    lp_token_amount: u64,
) -> Result<u64> {
    let current_time = checked_as_u64(Clock::get()?.unix_timestamp)?;

    Ok(mercurial_vault
        .get_amount_by_share(
            current_time,
            lp_token_amount,
            mercurial_vault_lp_mint_supply,
        )
        .ok_or(UxdError::MathOverflow)?)
}
