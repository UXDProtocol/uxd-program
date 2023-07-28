use anchor_lang::prelude::*;
use std::mem::size_of;

pub const LSD_POSITION_RESERVED_SPACE: usize = 64;
pub const LSD_POSITION_SPACE: usize = 8
    + size_of::<u8>() // bump
    + size_of::<Pubkey>() // depository
    + size_of::<u64>() // collateral_amount
    + size_of::<u64>() // redeemable_amount
    + size_of::<u8>() // effective_ltv_bps
    + size_of::<u64>() // liquidation_price

    + LSD_POSITION_RESERVED_SPACE;

/// Represents a user position tied to a specific LSD Depository.
/// It tracks the total collateral deposits and UXD borrowed.

#[account(zero_copy)]
#[repr(packed)]
pub struct LsdPosition {
    pub bump: u8,
    pub depository: Pubkey,
    pub collateral_amount: u64,
    pub redeemable_amount: u64,
    pub effective_ltv_bps: u8,
    pub liquidation_price: u64,

    pub _reserved: [u8; LSD_POSITION_RESERVED_SPACE],
}
