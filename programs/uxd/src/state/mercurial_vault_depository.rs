use anchor_lang::prelude::*;

// Total should be 900 bytes
pub const MERCURIAL_VAULT_RESERVED_SPACE: usize = 661;
pub const MERCURIAL_VAULT_DEPOSITORY_SPACE: usize = 8
    + 1
    + 1
    + 32
    + 1
    + 32
    + 16
    + 16
    + 32
    + 32
    + 1
    + 32
    + 1
    + 1
    + 1
    + 16
    + 16
    + MERCURIAL_VAULT_RESERVED_SPACE;

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

    // Fee applied at minting, expressed in basis point (bps) and taken by minting less redeemable for the user.
    // E.g, with a minting fee of 5 bps, if the user mint for 1_000_000 USDC (6 decimals), it should receive 999_500 UXD (6 decimals)
    // Calculation: (10_000 - 5) * 1_000_000 / 10_000
    pub minting_fee_in_bps: u8,

    // Fee applied at redeeming, expressed in basis point (bps) and taken by redeeming less lp token from the mercurial vault
    // thus sending less collateral to the user.
    // E.g, with a redeeming fee of 5 bps, if the user redeem for 1_000_000 UXD (6 decimals), it should receive 999_500 USDC (6 decimals)
    // Calculation: (10_000 - 5) * 1_000_000 / 10_000
    pub redeeming_fee_in_bps: u8,

    // The amount of fees accrued from minting
    // Expressed in redeemable mint decimals (6)
    pub total_paid_mint_fees: u128,

    // The amount of fees accrued from redeeming
    // Expressed in redeemable mint decimals (6)
    pub total_paid_redeem_fees: u128,
}
