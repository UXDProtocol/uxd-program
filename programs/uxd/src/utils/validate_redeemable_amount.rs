use crate::error::UxdError;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub fn validate_redeemable_amount(
    user_redeemable: &Account<TokenAccount>,
    redeemable_amount: u64,
) -> Result<()> {
    // Check that there is some redeemable to be moved
    require!(redeemable_amount > 0, UxdError::InvalidRedeemableAmount);
    // Check that the user has enough redeemable
    require!(
        user_redeemable.amount >= redeemable_amount,
        UxdError::InsufficientRedeemableAmount
    );
    // Done
    Ok(())
}
