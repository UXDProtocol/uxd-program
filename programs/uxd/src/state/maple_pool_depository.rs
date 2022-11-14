use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::error::UxdError;

pub const MAPLE_POOL_DEPOSITORY_SPACE: usize = 8 // anchor-pad
 + size_of::<u8>() // bump
 + size_of::<u8>() // version

 + size_of::<Pubkey>() // controller
 + size_of::<Pubkey>() // collateral_mint

 + size_of::<Pubkey>() // depository_collateral
 + size_of::<u8>() // depository_collateral_bump

 + size_of::<Pubkey>() // maple_pool
 + size_of::<Pubkey>() // maple_pool_locker
 + size_of::<Pubkey>() // maple_globals
 + size_of::<Pubkey>() // maple_lender
 + size_of::<Pubkey>() // maple_shares_mint
 + size_of::<Pubkey>() // maple_locked_shares
 + size_of::<Pubkey>() // maple_lender_shares

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
pub struct MaplePoolDepository {
    pub bump: u8,
    pub version: u8,

    // The Controller instance for which this Depository works for
    pub controller: Pubkey,

    // Token deposited in the vault
    pub collateral_mint: Pubkey,

    // The account owned by the despoitory containing the collateral during mint/redeem operations
    pub depository_collateral: Pubkey,
    pub depository_collateral_bump: u8,

    // Maple accounts enforced at registration
    pub maple_pool: Pubkey,
    pub maple_pool_locker: Pubkey,
    pub maple_globals: Pubkey,
    pub maple_lender: Pubkey,
    pub maple_shares_mint: Pubkey,
    pub maple_locked_shares: Pubkey,
    pub maple_lender_shares: Pubkey,

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

impl MaplePoolDepository {
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
            UxdError::RedeemableMaplePoolAmountUnderManagementCap
        );
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
}