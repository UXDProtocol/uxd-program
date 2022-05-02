use crate::*;

// 10000 equiv. to 100%
pub const LIQUIDITY_RATIO_BASIS: u16 = 10000;

pub const TARGET_LIQUIDITY_RATIO_MAX: u16 = 10000;

const MSOL_CONFIG_PADDING: usize = 64;

pub const MSOL_CONFIG_SPACE: usize =
    8 + 1 + 32 + 32 + 1 + 2 + MSOL_CONFIG_PADDING;

#[account(zero_copy)]
pub struct MSolConfig {
    pub bump: u8,
    // The Depository that this config account dedicated to
    pub depository: Pubkey,
    // The Controller that own this config account
    pub controller: Pubkey,
    // Whether allowing the SOL/mSOL swap
    pub enabled: bool,
    // A constant value of the proportion of SOL we wanna keep from the total amount of SOL + mSOL in terms of their value.
    // In LIQUIDITY_RATIO_BASIS
    pub target_liquidity_ratio: u16,
}