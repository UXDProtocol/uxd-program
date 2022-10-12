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

 + size_of::<u128>() // minted_redeemable_soft_cap
 + size_of::<u8>() // minting_fees_in_bps
 + size_of::<u8>() // redeeming_fees_in_bps

 + size_of::<u128>() // deposited_collateral_amount
 + size_of::<u128>() // minted_redeemable_amount
 + size_of::<u128>() // minting_fees_total_paid
 + size_of::<u128>() // redeeming_fees_total_paid

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
    pub minted_redeemable_soft_cap: u128,
    pub minting_fees_in_bps: u8,
    pub redeeming_fees_in_bps: u8,

    // Depository accouting
    pub deposited_collateral_amount: u128,
    pub minted_redeemable_amount: u128,
    pub minting_fees_total_paid: u128,
    pub redeeming_fees_total_paid: u128,
}

impl DepositoryConfiguration for MaplePoolDepository {
    fn get_minted_redeemable_soft_cap(&self) -> u128 {
        self.minted_redeemable_soft_cap
    }
    fn get_minting_fees_in_bps(&self) -> u8 {
        self.minting_fees_in_bps
    }
    fn get_redeeming_fees_in_bps(&self) -> u8 {
        self.redeeming_fees_in_bps
    }
}

impl DepositoryAccounting for MaplePoolDepository {
    fn get_deposited_collateral_amount(&self) -> u128 {
        self.deposited_collateral_amount
    }
    fn set_deposited_collateral_amount(&mut self, value: u128) {
        self.deposited_collateral_amount = value;
    }
    fn get_minted_redeemable_amount(&self) -> u128 {
        self.minted_redeemable_amount
    }
    fn set_minted_redeemable_amount(&mut self, value: u128) {
        self.minted_redeemable_amount = value;
    }
    fn get_minting_fees_total_paid(&self) -> u128 {
        self.minting_fees_total_paid
    }
    fn set_minting_fees_total_paid(&mut self, value: u128) {
        self.minting_fees_total_paid = value;
    }
    fn get_redeeming_fees_total_paid(&self) -> u128 {
        self.redeeming_fees_total_paid
    }
    fn set_redeeming_fees_total_paid(&mut self, value: u128) {
        self.redeeming_fees_total_paid = value;
    }
}
