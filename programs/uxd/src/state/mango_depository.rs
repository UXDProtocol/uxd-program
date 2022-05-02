use anchor_lang::prelude::*;

pub const MANGO_DEPOSITORY_RESERVED_SPACE: usize = 496;
pub const MANGO_DEPOSITORY_SPACE: usize = 8
    + 1
    + 2
    + 1
    + 1
    + 32
    + 1
    + 32
    + 32
    + 32
    + 1
    + 32
    + 32
    + 16
    + 16
    + 16
    + 16
    + 16
    + MANGO_DEPOSITORY_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct MangoDepository {
    pub bump: u8,
    pub _unused: [u8; 2],
    pub mango_account_bump: u8,
    // Version used
    pub version: u8,
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,
    pub _unused2: [u8; 32],
    pub quote_mint: Pubkey,
    pub _unused3: [u8; 32],
    pub quote_mint_decimals: u8,
    pub mango_account: Pubkey,
    //
    // The Controller instance for which this Depository works for
    pub controller: Pubkey,
    //
    // Accounting -------------------------------
    // Note : To keep track of the in and out of a depository
    //
    // The amount of USDC InsuranceFund deposited/withdrawn by Authority on the underlying Mango Account - The actual amount might be lower/higher depending of funding rate changes
    // In Quote native units
    pub insurance_amount_deposited: u128,
    //
    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    pub collateral_amount_deposited: u128,
    //
    // The amount of delta neutral position that is backing circulating redeemable.
    // Updated after each mint/redeem
    // In Redeemable native units
    pub redeemable_amount_under_management: u128,
    //
    // The amount of taker fee paid in quote while placing perp orders
    pub total_amount_paid_taker_fee: u128,
    //
    // The amount of DN position that has been rebalanced (in quote native units)
    pub total_amount_rebalanced: u128,
}
