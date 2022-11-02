use anchor_lang::prelude::*;

pub const IDENTITY_DEPOSITORY_RESERVED_SPACE: usize = 512;
pub const IDENTITY_DEPOSITORY_SPACE: usize = 8
    + 1 // bump
    + 1 // version
    + 32 // collateral_mint
    + 1 // collateral_mint_decimal
    + 32 // collateral_vault
    + 1 // collateral_vault_bump
    + 32 // controller
    + 16 // collateral_amount_deposited
    + 16 // redeemable_under_management
    + 16 // redeemable_under_management_cap
    + 1 // regular_minting_disabled
    + 1 // mango_collateral_reinjected
    + IDENTITY_DEPOSITORY_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct IdentityDepository {
    pub bump: u8,
    // Version used
    pub version: u8,
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,
    //
    // The depository TokenAccount that store the collateral
    pub collateral_vault: Pubkey,
    pub collateral_vault_bump: u8,
    //
    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    pub collateral_amount_deposited: u128,
    //
    // The amount of redeemable managed by this depository.
    // Updated after each mint/redeem
    // In Redeemable native units
    pub redeemable_amount_under_management: u128,
    pub redeemable_amount_under_management_cap: u128,
    // Flag to indicate whether minting through collateral deposits is allowed
    pub minting_disabled: bool,
    // has the collateral originally on mango reinjected to this depository
    pub mango_collateral_reinjected: bool,
}
