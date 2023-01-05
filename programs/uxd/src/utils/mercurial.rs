use crate::error::UxdError;
use crate::Clock;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use fixed::types::I80F48;

// Calculate how much collateral could be lost in possible LP token precision loss
pub fn calculate_possible_lp_token_precision_loss_collateral_value<'info>(
    mercurial_vault: &Account<'info, mercurial_vault::state::Vault>,
    mercurial_vault_lp_mint: &Account<'info, Mint>,
) -> Result<u64> {
    let current_time = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or_else(|| error!(UxdError::MathError))?;

    // Calculate the price of 1 native LP token
    // Do not use mercurial_vault.get_amount_by_share because it does not handle precision loss
    let total_amount = mercurial_vault
        .get_unlocked_amount(current_time)
        .ok_or_else(|| error!(UxdError::MathError))?;

    let one_lp_token_collateral_value = I80F48::from_num(1)
        .checked_mul(
            I80F48::checked_from_num(total_amount).ok_or_else(|| error!(UxdError::MathError))?,
        )
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_div(
            I80F48::checked_from_num(mercurial_vault_lp_mint.supply)
                .ok_or_else(|| error!(UxdError::MathError))?,
        )
        .ok_or_else(|| error!(UxdError::MathError))?
        .checked_ceil()
        .ok_or_else(|| error!(UxdError::MathError))?;

    one_lp_token_collateral_value
        .checked_to_num()
        .ok_or_else(|| error!(UxdError::MathError))
}
