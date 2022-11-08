use crate::error::UxdError;
use anchor_lang::{
    prelude::{Account, Clock, SolanaSysvar},
    Result,
};
use fixed::types::I80F48;
use mercurial_vault::state::Vault;

// Calculate how much collateral could be lost in possible LP token precision loss
pub fn calculate_possible_lp_token_precision_loss_collateral_value(
    mercurial_vault: &Account<Vault>,
    mercurial_vault_lp_mint_supply: u64,
) -> Result<u64> {
    let current_time = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or(UxdError::MathError)?;

    // Calculate the price of 1 native LP token
    // Do not use mercurial_vault.get_amount_by_share because it does not handle precision loss
    let total_amount = mercurial_vault
        .get_unlocked_amount(current_time)
        .ok_or(UxdError::MathError)?;

    let one_lp_token_collateral_value = I80F48::from_num(1)
        .checked_mul(I80F48::checked_from_num(total_amount).ok_or(UxdError::MathError)?)
        .ok_or(UxdError::MathError)?
        .checked_div(
            I80F48::checked_from_num(mercurial_vault_lp_mint_supply).ok_or(UxdError::MathError)?,
        )
        .ok_or(UxdError::MathError)?
        .checked_ceil()
        .ok_or(UxdError::MathError)?;

    Ok(one_lp_token_collateral_value
        .checked_to_num()
        .ok_or(UxdError::MathError)?)
}
