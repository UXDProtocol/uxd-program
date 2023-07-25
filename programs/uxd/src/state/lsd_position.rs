use anchor_lang::prelude::*;
use std::mem::size_of;

pub const LSD_POSITION_RESERVED_SPACE: usize = 64;
pub const LSD_POSITION_SPACE: usize = 8
    + size_of::<u8>() // bump
    + size_of::<u8>() // liquidation_thread_authority_bump
    + size_of::<Pubkey>() // depository
    + size_of::<u64>() // collateral_amount
    + size_of::<u64>() // redeemable_amount
    + size_of::<u64>() // liquidation_price

    + size_of::<bool>() // is_initialized
    + LSD_POSITION_RESERVED_SPACE;

/// Represents a user position tied to a specific LSD Depository.
/// It tracks the total collateral deposits and UXD borrowed.

#[account(zero_copy)]
#[repr(packed)]
pub struct LsdPosition {
    pub bump: u8,
    pub user_liquidation_thread_authority_bump: u8,
    pub depository: Pubkey,
    pub collateral_amount: u64,
    pub redeemable_amount: u64,
    pub liquidation_price: u64,

    pub is_initialized: bool,
    pub _reserved: [u8; LSD_POSITION_RESERVED_SPACE],
}
