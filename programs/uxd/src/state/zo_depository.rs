use anchor_lang::prelude::*;

pub const ZO_DEPOSITORY_SPACE: usize =
    8 + 1 + 1 + 1 + 1 + 32 + 32 + 1 + 32 + 1 + 32 + 32 + 16 + 16 + 16 + 16 + 512;

#[account(zero_copy)]
pub struct ZoDepository {
    pub bump: u8,
    pub zo_account_bump: u8,
    // Version used - for migrations later if needed
    pub version: u8,
    // Registration handle the UXD side of things, initialization create the necessary ZO PDA
    // (Too heavy to do it all in one IX due to current stack limitations)
    pub is_initialized: bool,

    // The ZO dex market tied to this depository, only this dex market can be interacted with
    pub zo_dex_market: Pubkey,

    // | - Collateral
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,

    // | - Quote
    pub quote_mint: Pubkey,
    pub quote_mint_decimals: u8,

    pub zo_account: Pubkey,
    //
    // The Controller instance for which this Depository works for
    pub controller: Pubkey,
    //
    // Accounting -------------------------------
    // Note : To keep track of the in and out of a depository
    //
    // The amount of USDC InsuranceFund deposited/withdrawn by Authority on the underlying ZO Account - The actual amount might be lower/higher depending of funding rate changes
    // In Collateral native units
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
    pub total_amount_rebalanced: u128,
}
