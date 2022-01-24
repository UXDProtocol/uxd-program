use crate::declare_check_assert_macros;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::UxdResult;
use anchor_lang::prelude::*;

declare_check_assert_macros!(SourceFileId::StateZoDepository);

#[account]
#[derive(Default)]
pub struct ZoDepository {
    pub bump: u8,
// pub collateral_passthrough_bump: u8,
// pub insurance_passthrough_bump: u8,
    pub zo_account_bump: u8,
    // Version used - for migrations later if needed
    pub version: u8,
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,
// pub collateral_passthrough: Pubkey,
    pub insurance_mint: Pubkey,
// pub insurance_passthrough: Pubkey,
    pub insurance_mint_decimals: u8,
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
    pub total_amount_paid_taker_fee: u128,
    //
    // Note : This is the last thing I'm working on and I would love some guidance from the audit. Anchor doesn't seems to play nice with padding
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
    fn deserialize(_: &mut &[u8]) -> Result<Self, std::io::Error> {
        Ok(Self([0u8; 512]))
    }
}

impl Default for ZoDepositoryPadding {
    fn default() -> Self {
        ZoDepositoryPadding { 0: [0u8; 512] }
    }
}

pub enum AccountingEvent {
    Deposit,
    Withdraw,
}

impl ZoDepository {
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
}
