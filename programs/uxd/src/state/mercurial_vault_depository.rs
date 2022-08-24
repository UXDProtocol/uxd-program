use anchor_lang::prelude::*;

pub const MERCURIAL_VAULT_DEPOSITORY_RESERVED_SPACE: usize = 616;

pub const MERCURIAL_VAULT_DEPOSITORY_SPACE: usize =
    8 + 1 + 1 + 32 + 1 + 32 + 128 + 32 + 1 + 32 + 1 + MERCURIAL_VAULT_DEPOSITORY_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct MercurialVaultDepository {
    pub bump: u8,

    // Version used
    pub version: u8,

    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,

    // The Controller instance for which this Depository works for
    pub controller: Pubkey,

    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    pub collateral_amount_deposited: u128,

    // Tokens received in exchange for depositing collateral
    pub lp_token_mint: Pubkey,
    pub lp_token_decimals: u8,

    // Keep the mercurial vault tokens
    pub lp_tokens_vault: Pubkey,
    pub lp_tokens_vault_bump: u8,
}
