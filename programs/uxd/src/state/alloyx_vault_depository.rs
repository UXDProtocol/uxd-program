use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::error::UxdError;
use crate::utils::checked_add;
use crate::utils::checked_sub;

pub const ALLOYX_VAULT_DEPOSITORY_RESERVED_SPACE: usize = 800;
pub const ALLOYX_VAULT_DEPOSITORY_SPACE: usize = 8 // anchor-pad
 + size_of::<u8>() // bump
 + size_of::<u8>() // version

 + size_of::<Pubkey>() // controller
 + size_of::<Pubkey>() // collateral_mint

 + size_of::<Pubkey>() // depository_collateral
 + size_of::<Pubkey>() // depository_shares

 + size_of::<Pubkey>() // alloyx_vault_info
 + size_of::<Pubkey>() // alloyx_vault_collateral
 + size_of::<Pubkey>() // alloyx_vault_shares
 + size_of::<Pubkey>() // alloyx_vault_mint

 + size_of::<u64>() // redeemable_amount_under_management_cap
 + size_of::<u8>() // minting_fee_in_bps
 + size_of::<u8>() // redeeming_fee_in_bps
 + size_of::<bool>() // minting_disabled

 + size_of::<u64>() // collateral_amount_deposited
 + size_of::<u64>() // redeemable_amount_under_management
 + size_of::<u64>() // minting_fee_total_accrued
 + size_of::<u64>() // redeeming_fee_total_accrued

 + size_of::<u64>() // profits_total_collected
 + size_of::<Pubkey>() // profits_beneficiary_collateral

 + ALLOYX_VAULT_DEPOSITORY_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct AlloyxVaultDepository {
    pub bump: u8,
    pub version: u8,

    // The Controller instance for which this Depository works for
    pub controller: Pubkey,

    // Token deposited in the vault
    pub collateral_mint: Pubkey,

    // The account owned by the despoitory containing the collateral during mint/redeem operations
    pub depository_collateral: Pubkey,

    // The account owned by the despoitory containing the shares owned
    pub depository_shares: Pubkey,

    // Alloyx vault accounts enforced at registration
    pub alloyx_vault_info: Pubkey,
    pub alloyx_vault_collateral: Pubkey,
    pub alloyx_vault_shares: Pubkey,
    pub alloyx_vault_mint: Pubkey,

    // Depository configuration
    pub redeemable_amount_under_management_cap: u64,
    pub minting_fee_in_bps: u8,
    pub redeeming_fee_in_bps: u8,
    pub minting_disabled: bool,

    // Depository accouting
    pub collateral_amount_deposited: u64,
    pub redeemable_amount_under_management: u64,
    pub minting_fee_total_accrued: u64,
    pub redeeming_fee_total_accrued: u64,

    // Collection of the depository's profits
    pub profits_total_collected: u64,
    pub profits_beneficiary_collateral: Pubkey,

    // For future usage
    pub _reserved: [u8; ALLOYX_VAULT_DEPOSITORY_RESERVED_SPACE],
}

impl AlloyxVaultDepository {
    // When we mint, we need to increment the supply counters
    pub fn collateral_deposited_and_redeemable_minted(
        &mut self,
        collateral_amount_added: u64,
        redeemable_amount_added: u64,
    ) -> Result<()> {
        // Check that there was some successful minting
        require!(
            collateral_amount_added > 0,
            UxdError::InvalidCollateralAmount,
        );
        require!(
            redeemable_amount_added > 0,
            UxdError::MinimumMintedRedeemableAmountError
        );
        // Actually add the recent change
        self.collateral_amount_deposited =
            checked_add(self.collateral_amount_deposited, collateral_amount_added)?;
        self.redeemable_amount_under_management = checked_add(
            self.redeemable_amount_under_management,
            redeemable_amount_added,
        )?;
        Ok(())
    }

    // When we redeem, we need to decrement the supply counters
    pub fn collateral_withdrawn_and_redeemable_burned(
        &mut self,
        collateral_amount_removed: u64,
        redeemable_amount_removed: u64,
    ) -> Result<()> {
        // Check that there was some successful redeeming
        require!(
            collateral_amount_removed > 0,
            UxdError::InvalidCollateralAmount,
        );
        require!(
            redeemable_amount_removed > 0,
            UxdError::InvalidRedeemableAmount
        );
        // Actually add the recent change
        self.collateral_amount_deposited =
            checked_sub(self.collateral_amount_deposited, collateral_amount_removed)?;
        self.redeemable_amount_under_management = checked_sub(
            self.redeemable_amount_under_management,
            redeemable_amount_removed,
        )?;
        Ok(())
    }

    // When minting fee was paid, we need to add it to the total
    pub fn minting_fee_accrued(&mut self, minting_fee_paid: u64) -> Result<()> {
        self.minting_fee_total_accrued =
            checked_add(self.minting_fee_total_accrued, minting_fee_paid)?;
        Ok(())
    }

    // When redeeming fee was paid, we need to add it to the total
    pub fn redeeming_fee_accrued(&mut self, redeeming_fee_paid: u64) -> Result<()> {
        self.redeeming_fee_total_accrued =
            checked_add(self.redeeming_fee_total_accrued, redeeming_fee_paid)?;
        Ok(())
    }

    // When collecting profits, we need to add it to the total
    pub fn update_onchain_accounting_following_profits_collection(
        &mut self,
        profits_collected: u64,
    ) -> Result<()> {
        self.profits_total_collected =
            checked_add(self.profits_total_collected, profits_collected)?;
        Ok(())
    }
}
