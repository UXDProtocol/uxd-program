use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct MangoDepository {
    pub bump: u8,
    pub collateral_passthrough_bump: u8,
    pub insurance_passthrough_bump: u8,
    pub mango_account_bump: u8,
    // Version used
    pub version: u8,
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,
    pub collateral_passthrough: Pubkey,
    pub insurance_mint: Pubkey,
    pub insurance_passthrough: Pubkey,
    pub insurance_mint_decimals: u8,
    pub mango_account: Pubkey,
    //
    // The Controller instance for which this Depository works for
    pub controller: Pubkey,
    //
    // Accounting -------------------------------
    // Note : To keep track of the in and out of a depository
    //
    // The amount of USDC InsuranceFund deposited/withdrawn by Authority on the underlying Mango Account - The actual amount might be lower/higher depending of funding rate changes
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
    pub total_amount_paid_taker_fee: u128,
    //
    pub _reserved: u8,
    //
    // This information is shared by all the Depositories, and as such would have been a good
    // candidate for the Controller, but we will lack space in the controller sooner than here.
    //
    // v2 -83 bytes
    pub quote_mint: Pubkey,
    pub quote_passthrough: Pubkey,
    pub quote_passthrough_bump: u8,
    pub quote_mint_decimals: u8,
    //
    // The amount of DN position that has been rebalanced (in quote native units)
    pub total_amount_rebalanced: u128,
    //
    pub _reserved1: MangoDepositoryPadding,
}

#[derive(Clone)]
pub struct MangoDepositoryPadding([u8; 429]);

impl AnchorSerialize for MangoDepositoryPadding {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl AnchorDeserialize for MangoDepositoryPadding {
    fn deserialize(_: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self([0u8; 429]))
    }
}

impl Default for MangoDepositoryPadding {
    fn default() -> Self {
        MangoDepositoryPadding([0u8; 429])
    }
}
