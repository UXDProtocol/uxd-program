use anchor_lang::prelude::*;
use std::mem::size_of;

use super::OracleParams;

pub const LSD_DEPOSITORY_RESERVED_SPACE: usize = 256;
pub const LSD_DEPOSITORY_SPACE: usize = 8
    + size_of::<u8>() // bump
    + size_of::<u8>() // version
    + size_of::<Pubkey>() // collateral_mint
    + size_of::<u8>() // collateral_mint_decimal
    + size_of::<Pubkey>() // liquidation_mint
    + size_of::<u8>() // liquidation_mint_decimal
    + size_of::<Pubkey>() // profits_token_account
    + size_of::<u8>() // profits_token_account_bump

    + size_of::<bool>() // borrowing_disabled
    + size_of::<u64>() // redeemable_amount_under_management_cap
    + size_of::<u8>() // borrowing_fee_bps
    + size_of::<u8>() // repay_fee_bps
    + size_of::<u8>() // loan_to_value_bps
    + size_of::<u16>() // max_loan_to_value_bps
    + size_of::<u16>() // liquidation_loan_to_value_threshold_bps
    + size_of::<u16>() // liquidation_fee_bps
    + size_of::<Pubkey>() // profits_beneficiary

    + size_of::<u64>() // collateral_amount_deposits
    + size_of::<u64>() // redeemable_amount_under_management

    + size_of::<u128>() // collateral_amount_liquidated
    + size_of::<u128>() // borrow_fee_accrued
    + size_of::<u128>() // repay_fee_accrued
    + size_of::<u128>() // liquidation_fee_total_accrued
    + size_of::<u128>() // profits_collected

    + LSD_DEPOSITORY_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct LsdDepository {
    pub bump: u8,
    pub version: u8,
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,
    pub liquidation_mint: Pubkey, // when liquidation occurs, this is the target mint (should )
    pub liquidation_mint_decimals: u8,
    pub profits_token_account: Pubkey, // using liquidation_mint. Contains liquidation profits + borrowing fees if any
    pub profits_token_account_bump: u8,
    pub collateral_oracle_params: OracleParams,

    // Configuration
    pub borrowing_disabled: bool,
    pub redeemable_amount_under_management_cap: u64,
    pub borrowing_fee_bps: u8,
    pub repay_fee_bps: u8,
    pub max_loan_to_value_bps: u16,
    pub liquidation_loan_to_value_threshold_bps: u16,
    pub liquidation_fee_bps: u16,
    pub profits_beneficiary: Pubkey,

    // Accouting
    pub collateral_amount_deposits: u64,
    pub redeemable_amount_under_management: u64,

    // Stats
    pub collateral_amount_liquidated: u128,
    pub borrow_fee_accrued: u128,
    pub repay_fee_accrued: u128,
    pub liquidation_fee_accrued: u128,
    pub profits_collected: u128,

    pub _reserved: [u8; LSD_DEPOSITORY_RESERVED_SPACE],
}
