use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::error::UxdError;
use crate::utils::checked_add_u128_and_i128;

pub const MERCURIAL_VAULT_RESERVED_SPACE: usize = 572;
pub const MERCURIAL_VAULT_DEPOSITORY_SPACE: usize = 8
    + size_of::<u8>()     // bump
    + size_of::<u8>()     // version
    + size_of::<Pubkey>() // collateral_mint
    + size_of::<u8>()     // collateral_mint_decimals
    + size_of::<Pubkey>() // controller
    + size_of::<u128>()   // collateral_amount_deposited
    + size_of::<u128>()   // redeemable_amount_under_management
    + size_of::<Pubkey>() // mercurial_vault
    + size_of::<Pubkey>() // mercurial_vault_lp_mint
    + size_of::<u8>()     // mercurial_vault_lp_mint_decimals
    + size_of::<Pubkey>() // lp_token_vault
    + size_of::<u8>()     // lp_token_vault_bump
    + size_of::<u8>()     // minting_fee_in_bps
    + size_of::<u8>()     // redeeming_fee_in_bps
    + size_of::<u128>()   // minting_fee_total_accrued
    + size_of::<u128>()   // redeeming_fee_total_accrued
    + size_of::<u128>()   // redeemable_amount_under_management_cap
    + size_of::<bool>()   // minting_disabled
    + size_of::<u128>()   // profits_total_collected
    + size_of::<u64>()    // last_profits_collection_unix_timestamp
    + size_of::<Pubkey>() // profits_beneficiary_collateral
    + size_of::<u64>() // redeemable_amount_under_management_weight
    + size_of::<u64>() // redeemable_amount_under_management_target
    + MERCURIAL_VAULT_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct MercurialVaultDepository {
    pub bump: u8,
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
    pub redeemable_amount_under_management: u128,

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
    pub minting_fee_total_accrued: u128,

    // The amount of fees accrued from redeeming
    // Expressed in redeemable mint decimals (6)
    pub redeeming_fee_total_accrued: u128,

    // The total amount of circulating UXD originating from that depository
    pub redeemable_amount_under_management_cap: u128,

    pub minting_disabled: bool,

    // Total amount of interests collected by interests_and_fees_redeem_authority
    pub profits_total_collected: u128,

    // Worth 0 if interests and fees never got collected
    pub last_profits_collection_unix_timestamp: u64,

    // Receiver of the depository's profits
    pub profits_beneficiary_collateral: Pubkey,

    // Redeemable amount targets used for rebalancing
    pub redeemable_amount_under_management_weight: u64,
    pub redeemable_amount_under_management_target: u64,

    // For future usage
    pub _reserved: [u8; MERCURIAL_VAULT_RESERVED_SPACE],
}

impl MercurialVaultDepository {
    pub fn update_onchain_accounting_following_profits_collection(
        &mut self,
        collected_profits: u64,
        current_time_as_unix_timestamp: u64,
    ) -> Result<()> {
        self.profits_total_collected = self
            .profits_total_collected
            .checked_add(collected_profits.into())
            .ok_or(UxdError::MathError)?;

        self.last_profits_collection_unix_timestamp = current_time_as_unix_timestamp;

        Ok(())
    }

    // provides numbers + or - depending on the change
    pub fn update_onchain_accounting_following_mint_or_redeem(
        &mut self,
        collateral_amount_deposited_change: i128,
        redeemable_amount_change: i128,
        paid_minting_fees_change: i128,
        paid_redeeming_fees_change: i128,
    ) -> Result<()> {
        self.collateral_amount_deposited = checked_add_u128_and_i128(
            self.collateral_amount_deposited,
            collateral_amount_deposited_change,
        )?;

        self.redeemable_amount_under_management = checked_add_u128_and_i128(
            self.redeemable_amount_under_management,
            redeemable_amount_change,
        )?;

        self.minting_fee_total_accrued =
            checked_add_u128_and_i128(self.minting_fee_total_accrued, paid_minting_fees_change)?;

        self.redeeming_fee_total_accrued = checked_add_u128_and_i128(
            self.redeeming_fee_total_accrued,
            paid_redeeming_fees_change,
        )?;

        Ok(())
    }
}
