use anchor_lang::prelude::*;

pub const MAX_STRATEGY: usize = 30;
pub const MAX_BUMPS: usize = 10;
pub const LOCKED_PROFIT_DEGRADATION_DENOMINATOR: u128 = 1_000_000_000_000;

const WRAPPED_LOCKED_PROFIT_TRACKER_SPACE: usize = 8 + 8 + 8;

pub const MERCURIAL_VAULT_DEPOSITORY_SPACE: usize = 8
    + 1
    + 1
    + 1
    + 8
    + 32
    + 32
    + 32
    + 32
    + 32 * MAX_STRATEGY
    + 32
    + 32
    + 32
    + WRAPPED_LOCKED_PROFIT_TRACKER_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct WrappedMercurialVault {
    pub enabled: u8,
    pub bumps: WrappedVaultBumps,

    pub total_amount: u64,

    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub token_mint: Pubkey,

    pub lp_mint: Pubkey,
    pub strategies: [Pubkey; MAX_STRATEGY],

    pub base: Pubkey,
    pub admin: Pubkey,
    pub operator: Pubkey,
    pub locked_profit_tracker: WrappedLockedProfitTracker,
}

#[account(zero_copy)]
#[repr(packed)]
pub struct WrappedLockedProfitTracker {
    pub last_updated_locked_profit: u64,
    pub last_report: u64,
    pub locked_profit_degradation: u64,
}

#[account(zero_copy)]
#[repr(packed)]
pub struct WrappedVaultBumps {
    pub vault_bump: u8,
    pub token_vault_bump: u8,
}
