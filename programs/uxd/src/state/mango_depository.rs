use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

const MANGO_DEPOSITORY_PADDING: usize = 480;

#[account]
#[derive(Default)]
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
    //
    // The amount of redeemable that has been minted with stables
    pub total_stable_minted: u64,
    //
    pub _reserved: MangoDepositoryPadding,
}

#[derive(Clone)]
pub struct MangoDepositoryPadding([u8; MANGO_DEPOSITORY_PADDING]);

impl AnchorSerialize for MangoDepositoryPadding {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl AnchorDeserialize for MangoDepositoryPadding {
    fn deserialize(_: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self([0u8; MANGO_DEPOSITORY_PADDING]))
    }
}

impl Default for MangoDepositoryPadding {
    fn default() -> Self {
        MangoDepositoryPadding([0u8; MANGO_DEPOSITORY_PADDING])
    }
}
