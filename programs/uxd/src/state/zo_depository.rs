use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ZoDepository {
    pub bump: u8,
    pub zo_account_bump: u8,
    pub collateral_passthrough_bump: u8,
    pub insurance_passthrough_bump: u8,
    pub quote_passthrough_bump: u8,
    // Version used - for migrations later if needed
    pub version: u8,
    // Registration handle the UXD side of things, initialization create the necessary ZO PDA
    // (Too heavy to do it all in one IX due to current stack limitations)
    pub is_initialized: bool,

    // | Mint/Passthrough accounts information
    // | - Collateral
    pub collateral_mint: Pubkey,
    pub collateral_passthrough: Pubkey,
    pub collateral_mint_decimals: u8,
    // | - Insurance
    pub insurance_mint: Pubkey,
    pub insurance_passthrough: Pubkey,
    pub insurance_mint_decimals: u8,
    // | - Quote
    pub quote_mint: Pubkey,
    pub quote_passthrough: Pubkey,
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
    //
    pub _reserved: ZoDepositoryPadding,
}

#[derive(Clone)]
pub struct ZoDepositoryPadding([u8; 512]);

impl AnchorSerialize for ZoDepositoryPadding {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl AnchorDeserialize for ZoDepositoryPadding {
    fn deserialize(_: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self([0u8; 512]))
    }
}

impl Default for ZoDepositoryPadding {
    fn default() -> Self {
        ZoDepositoryPadding { 0: [0u8; 512] }
    }
}
