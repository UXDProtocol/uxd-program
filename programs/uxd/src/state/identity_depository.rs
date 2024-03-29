use std::mem::size_of;

use anchor_lang::prelude::*;

pub const IDENTITY_DEPOSITORY_RESERVED_SPACE: usize = 512;
pub const IDENTITY_DEPOSITORY_SPACE: usize = 8
    + size_of::<u8>() // bump
    + size_of::<u8>() // version
    + size_of::<Pubkey>() // collateral_mint
    + size_of::<u8>() // collateral_mint_decimal
    + size_of::<Pubkey>() // collateral_vault
    + size_of::<u8>() // collateral_vault_bump
    + size_of::<u128>() // collateral_amount_deposited
    + size_of::<u128>() // redeemable_amount_under_management
    + size_of::<u128>() // redeemable_amount_under_management_cap
    + size_of::<bool>() // minting_disabled
    + size_of::<bool>() // mango_collateral_reinjected_wsol
    + size_of::<bool>() // mango_collateral_reinjected_btc
    + size_of::<bool>() // mango_collateral_reinjected_eth
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
    pub mango_collateral_reinjected_wsol: bool,
    pub mango_collateral_reinjected_btc: bool,
    pub mango_collateral_reinjected_eth: bool,
    // For future usage
    pub _reserved: [u8; IDENTITY_DEPOSITORY_RESERVED_SPACE],
}
