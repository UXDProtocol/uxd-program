use crate::error::UxdError;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub fn validate_collateral_amount(
    user_collateral: &Account<TokenAccount>,
    collateral_amount: u64,
) -> Result<()> {
    // Check that there is some collateral to be moved
    require!(collateral_amount > 0, UxdError::InvalidCollateralAmount);
    // Check that the user has enough collateral
    require!(
        user_collateral.amount >= collateral_amount,
        UxdError::InsufficientCollateralAmount
    );
    // Done
    Ok(())
}
