use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::UxdResult;
use anchor_lang::prelude::*;

declare_check_assert_macros!(SourceFileId::StateMangoDepository);

#[account]
#[derive(Default)]
pub struct MangoDepository {
    pub bump: u8,                        // Unused
    pub collateral_passthrough_bump: u8, // Unused
    pub insurance_passthrough_bump: u8,  // Unused
    pub mango_account_bump: u8,          // Unused
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
    // v2 -82 bytes
    pub quote_mint: Pubkey,
    pub quote_passthrough: Pubkey,
    pub quote_mint_decimals: u8,
    //
    // The amount of DN position that has been rebalanced (in quote native units)
    pub total_amount_rebalanced: u128,
    //
    pub _reserved1: MangoDepositoryPadding,
}

#[derive(Clone)]
pub struct MangoDepositoryPadding([u8; 430]);

impl AnchorSerialize for MangoDepositoryPadding {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl AnchorDeserialize for MangoDepositoryPadding {
    fn deserialize(_: &mut &[u8]) -> Result<Self, std::io::Error> {
        Ok(Self([0u8; 430]))
    }
}

impl Default for MangoDepositoryPadding {
    fn default() -> Self {
        MangoDepositoryPadding([0u8; 430])
    }
}

pub enum AccountingEvent {
    Deposit,
    Withdraw,
}

impl MangoDepository {
    pub fn update_insurance_amount_deposited(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) -> UxdResult {
        self.insurance_amount_deposited = match event_type {
            AccountingEvent::Deposit => self
                .insurance_amount_deposited
                .checked_add(amount.into())
                .ok_or(math_err!())?,
            AccountingEvent::Withdraw => self
                .insurance_amount_deposited
                .checked_sub(amount.into())
                .ok_or(math_err!())?,
        };
        Ok(())
    }

    pub fn update_collateral_amount_deposited(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) -> UxdResult {
        self.collateral_amount_deposited = match event_type {
            AccountingEvent::Deposit => self
                .collateral_amount_deposited
                .checked_add(amount.into())
                .ok_or(math_err!())?,
            AccountingEvent::Withdraw => self
                .collateral_amount_deposited
                .checked_sub(amount.into())
                .ok_or(math_err!())?,
        };
        Ok(())
    }

    pub fn update_redeemable_amount_under_management(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) -> UxdResult {
        self.redeemable_amount_under_management = match event_type {
            AccountingEvent::Deposit => self
                .redeemable_amount_under_management
                .checked_add(amount.into())
                .ok_or(math_err!())?,
            AccountingEvent::Withdraw => self
                .redeemable_amount_under_management
                .checked_sub(amount.into())
                .ok_or(math_err!())?,
        };
        Ok(())
    }

    pub fn update_total_amount_paid_taker_fee(&mut self, amount: u64) -> UxdResult {
        self.total_amount_paid_taker_fee = self
            .total_amount_paid_taker_fee
            .checked_add(amount.into())
            .ok_or(math_err!())?;
        Ok(())
    }

    pub fn update_rebalanced_amount(&mut self, amount: u64) -> UxdResult {
        self.total_amount_rebalanced = self
            .total_amount_rebalanced
            .checked_add(amount.into())
            .ok_or(math_err!())?;
        Ok(())
    }
}
