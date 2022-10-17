use std::mem::size_of;

use anchor_lang::prelude::*;

use super::{depository_accounting::DepositoryAccounting, DepositoryConfiguration};

pub const MAPLE_POOL_DEPOSITORY_SPACE: usize = 8 // anchor-pad
 + size_of::<u8>() // bump
 + size_of::<u8>() // version

 + size_of::<Pubkey>() // controller
 + size_of::<Pubkey>() // collateral_mint

 + size_of::<Pubkey>() // depository_collateral
 + size_of::<u8>() // depository_collateral_bump

 + size_of::<Pubkey>() // maple_pool
 + size_of::<Pubkey>() // maple_lender
 + size_of::<Pubkey>() // maple_shares_mint
 + size_of::<Pubkey>() // maple_locked_shares
 + size_of::<Pubkey>() // maple_lender_shares

 + size_of::<u128>() // redeemable_amount_under_management_cap
 + size_of::<u8>() // minting_fee_in_bps
 + size_of::<u8>() // redeeming_fee_in_bps

 + size_of::<u128>() // collateral_amount_deposited
 + size_of::<u128>() // redeemable_amount_under_management
 + size_of::<u128>() // total_paid_minting_fees
 + size_of::<u128>() // total_paid_redeeming_fee

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

    // Maple accounts
    pub maple_pool: Pubkey,
    pub maple_lender: Pubkey,
    pub maple_shares_mint: Pubkey,
    pub maple_locked_shares: Pubkey,
    pub maple_lender_shares: Pubkey,

    // Depository configuration
    pub redeemable_amount_under_management_cap: u128,
    pub minting_fee_in_bps: u8,
    pub redeeming_fee_in_bps: u8,

    // Depository accouting
    pub collateral_amount_deposited: u128,
    pub redeemable_amount_under_management: u128,
    pub total_paid_minting_fees: u128,
    pub total_paid_redeeming_fee: u128,
}

impl DepositoryConfiguration for MaplePoolDepository {
    fn get_redeemable_amount_under_management_cap(&self) -> u128 {
        self.redeemable_amount_under_management_cap
    }
    fn get_minting_fee_in_bps(&self) -> u8 {
        self.minting_fee_in_bps
    }
    fn get_redeeming_fee_in_bps(&self) -> u8 {
        self.redeeming_fee_in_bps
    }
}

impl DepositoryAccounting for MaplePoolDepository {
    fn get_collateral_amount_deposited(&self) -> u128 {
        self.collateral_amount_deposited
    }
    fn set_collateral_amount_deposited(&mut self, value: u128) {
        self.collateral_amount_deposited = value;
    }
    fn get_redeemable_amount_under_management(&self) -> u128 {
        self.redeemable_amount_under_management
    }
    fn set_redeemable_amount_under_management(&mut self, value: u128) {
        self.redeemable_amount_under_management = value;
    }
    fn get_total_paid_minting_fees(&self) -> u128 {
        self.total_paid_minting_fees
    }
    fn set_total_paid_minting_fees(&mut self, value: u128) {
        self.total_paid_minting_fees = value;
    }
    fn get_total_paid_redeeming_fee(&self) -> u128 {
        self.total_paid_redeeming_fee
    }
    fn set_total_paid_redeeming_fee(&mut self, value: u128) {
        self.total_paid_redeeming_fee = value;
    }
}
