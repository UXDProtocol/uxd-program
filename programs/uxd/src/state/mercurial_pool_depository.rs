use anchor_lang::prelude::*;

pub const MERCURIAL_POOL_DEPOSITORY_RESERVED_SPACE: usize = 583;

pub const MERCURIAL_POOL_DEPOSITORY_SPACE: usize = 8
    + 1
    + 1
    + 32
    + 1
    + 32
    + 128
    + 32
    + 32
    + 1
    + 32
    + 1
    + 1
    + 1
    + MERCURIAL_POOL_DEPOSITORY_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct MercurialPoolDepository {
    pub bump: u8,

    // Version used
    pub version: u8,

    // Token deposited to the pool
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,

    // The Controller instance for which this Depository works for
    pub controller: Pubkey,

    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    pub collateral_amount_deposited: u128,

    // Mercurial pool account
    pub mercurial_pool: Pubkey,

    // Tokens received in exchange for depositing collateral
    pub pool_lp_mint: Pubkey,
    pub pool_lp_mint_decimals: u8,

    // Keep the mercurial vault tokens
    pub pool_lp_token_vault: Pubkey,
    pub pool_lp_token_vault_bump: u8,

    // TODO: change to enum
    pub collateral_is_mercurial_pool_token_a: bool,
    pub collateral_is_mercurial_pool_token_b: bool,
}
