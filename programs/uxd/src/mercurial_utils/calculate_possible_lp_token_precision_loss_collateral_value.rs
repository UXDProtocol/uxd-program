use crate::{error::UxdError, utils::compute_value_for_single_share_ceil};
use anchor_lang::{
    prelude::{Account, Clock, SolanaSysvar},
    Result,
};

use mercurial_vault::state::Vault;

// Calculate how much collateral could be lost in possible LP token precision loss
pub fn calculate_possible_lp_token_precision_loss_collateral_value(
    mercurial_vault: &Account<Vault>,
    mercurial_vault_lp_mint_supply: u64,
) -> Result<u64> {
    let current_time = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or(UxdError::MathOverflow)?;

    // Calculate the price of 1 native LP token
    // Do not use mercurial_vault.get_amount_by_share because it does not handle precision loss
    let total_unlocked_lp_token_value = mercurial_vault
        .get_unlocked_amount(current_time)
        .ok_or(UxdError::MathOverflow)?;

    compute_value_for_single_share_ceil(
        total_unlocked_lp_token_value,
        mercurial_vault_lp_mint_supply,
    )
}
