use crate::error::UxdError;
use anchor_lang::prelude::*;

pub const SAFETY_VAULT_SPACE: usize = 8
    + 1
    + 1
    + 1
    + 32
    + 32
    + 32
    + 16
    + 16;

// This is the safety vault that will handle the result of kill-switch
// operations, holding the USDC from liquidating
#[account(zero_copy)]
pub struct SafetyVault {
    pub bump: u8,
    pub quote_vault_bump: u8,
    // Version used
    pub version: u8,
    // The account with authority over the controller
    pub authority: Pubkey,
    // The depository the SafetyVault is used for
    pub depository: Pubkey,
    // The token account of the SafetyVault to hold the quote
    pub quote_vault: Pubkey,
    //
    // Accounting -------------------------------
    //
    // The amount of depository.collateral_mint that has been liquidated
    pub collateral_liquidated: u128,
    // The amount of quote held in the quote_vault
    pub quote_vault_balance: u128,
}
