use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::error::UxdError;

pub const CREDIX_LP_DEPOSITORY_SPACE: usize = 8 // anchor-pad
 + size_of::<u8>() // bump
 + size_of::<u8>() // version

 + size_of::<Pubkey>() // controller
 + size_of::<Pubkey>() // collateral_mint

 + size_of::<Pubkey>() // depository_collateral
 + size_of::<Pubkey>() // depository_shares

 + size_of::<Pubkey>() // credix_program_state
 + size_of::<Pubkey>() // credix_global_market_state
 + size_of::<Pubkey>() // credix_signing_authority
 + size_of::<Pubkey>() // credix_liquidity_collateral
 + size_of::<Pubkey>() // credix_shares_mint

 + size_of::<u128>() // redeemable_amount_under_management_cap
 + size_of::<u8>() // minting_fee_in_bps
 + size_of::<u8>() // redeeming_fee_in_bps
 + size_of::<bool>() // minting_disabled

 + size_of::<u128>() // collateral_amount_deposited
 + size_of::<u128>() // redeemable_amount_under_management
 + size_of::<u128>() // minting_fee_total_accrued
 + size_of::<u128>() // redeeming_fee_total_accrued

 + 800; // reserved space

#[account(zero_copy)]
#[repr(packed)]
pub struct CredixLpDepository {
    pub bump: u8,
    pub version: u8,

    // The Controller instance for which this Depository works for
    pub controller: Pubkey,

    // Token deposited in the vault
    pub collateral_mint: Pubkey,

    // The account owned by the despoitory containing the collateral during mint/redeem operations
    pub depository_collateral: Pubkey,

    // The account owned by the despoitory containing the lp shares owned
    pub depository_shares: Pubkey,

    // Credix accounts enforced- at registration
    pub credix_program_state: Pubkey,
    pub credix_global_market_state: Pubkey,
    pub credix_signing_authority: Pubkey,
    pub credix_liquidity_collateral: Pubkey,
    pub credix_shares_mint: Pubkey,

    // Depository configuration
    pub redeemable_amount_under_management_cap: u128,
    pub minting_fee_in_bps: u8,
    pub redeeming_fee_in_bps: u8,
    pub minting_disabled: bool,

    // Depository accouting
    pub collateral_amount_deposited: u128,
    pub redeemable_amount_under_management: u128,
    pub minting_fee_total_accrued: u128,
    pub redeeming_fee_total_accrued: u128,
}

impl CredixLpDepository {
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
        self.collateral_amount_deposited = self
            .collateral_amount_deposited
            .checked_add(collateral_amount_added.into())
            .ok_or(UxdError::MathError)?;
        self.redeemable_amount_under_management = self
            .redeemable_amount_under_management
            .checked_add(redeemable_amount_added.into())
            .ok_or(UxdError::MathError)?;
        // Check that we're not minting past the cap
        require!(
            self.redeemable_amount_under_management <= self.redeemable_amount_under_management_cap,
            UxdError::RedeemableCredixLpAmountUnderManagementCap
        );
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
        self.collateral_amount_deposited = self
            .collateral_amount_deposited
            .checked_sub(collateral_amount_removed.into())
            .ok_or(UxdError::MathError)?;
        self.redeemable_amount_under_management = self
            .redeemable_amount_under_management
            .checked_sub(redeemable_amount_removed.into())
            .ok_or(UxdError::MathError)?;
        Ok(())
    }

    // When minting fee was paid, we need to add it to the total
    pub fn minting_fee_accrued(&mut self, minting_fee_paid: u64) -> Result<()> {
        self.minting_fee_total_accrued = self
            .minting_fee_total_accrued
            .checked_add(minting_fee_paid.into())
            .ok_or(UxdError::MathError)?;
        Ok(())
    }

    // When redeeming fee was paid, we need to add it to the total
    pub fn redeeming_fee_accrued(&mut self, redeeming_fee_paid: u64) -> Result<()> {
        self.redeeming_fee_total_accrued = self
            .redeeming_fee_total_accrued
            .checked_add(redeeming_fee_paid.into())
            .ok_or(UxdError::MathError)?;
        Ok(())
    }
}
