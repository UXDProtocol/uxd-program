use anchor_lang::prelude::*;

// Total account size target: 900
pub const MERCURIAL_VAULT_DEPOSITORY_RESERVED_SPACE: usize = 479;

pub const MERCURIAL_VAULT_DEPOSITORY_SPACE: usize = 8
    + 1
    + 1
    + 32
    + 1
    + 32
    + 128
    + 128
    + 32
    + 32
    + 1
    + 32
    + 1
    + 1
    + MERCURIAL_VAULT_DEPOSITORY_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct MercurialVaultDepository {
    pub bump: u8,

    // Version used
    pub version: u8,

    // Token deposited in the vault
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,

    // The Controller instance for which this Depository works for
    pub controller: Pubkey,

    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    pub collateral_amount_deposited: u128,

    // The amount of minted redeemable using this repository
    // Equals to collateral_amount_deposited, minus precision loss
    pub minted_redeemable_amount: u128,

    // mercurial_vault linked to the depository
    pub mercurial_vault: Pubkey,

    // LP tokens received in exchange for depositing collateral
    pub mercurial_vault_lp_mint: Pubkey,
    pub mercurial_vault_lp_mint_decimals: u8,

    // Token account holding the LP tokens minted by depositing collateral on mercurial vault
    pub lp_token_vault: Pubkey,
    pub lp_token_vault_bump: u8,
}
