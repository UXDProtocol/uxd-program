use crate::{ErrorCode, UxdResult};
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct MangoDepository {
    pub bump: u8,
    pub collateral_passthrough_bump: u8,
    pub insurance_passthrough_bump: u8,
    pub mango_account_bump: u8,
    // Version used - for migrations later if needed
    pub version: u8,
    pub collateral_mint: Pubkey,
    pub collateral_passthrough: Pubkey,
    pub insurance_mint: Pubkey,
    pub insurance_passthrough: Pubkey,
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
    // The amount of delta neutral position that is backing circulating redeemables.
    // Updated after each mint/redeem
    // In Redeemable native units
    pub redeemable_amount_under_management: u128,
    //
    // The amount of delta neutral position accounting for taker fees during mint and redeem operations, so with no equivalence circulating as redeemable.
    //
    // This represent the amount of the delta neutral position that is locked, accounting for fees settlements.
    // Fee are paid in USDC, and so we keep a piece of the delta neutral quote position to account for them during each minting/redeeming operations.
    //
    // Updated after each mint/redeem (/rebalance_fees when implemented)
    // In Redeemable native units
    pub delta_neutral_quote_fee_offset: u128,
    //
    // The total amount of Redeemable Tokens this Depository instance hold
    // This should always be equal to `delta_neutral_quote_position` - `delta_neutral_quote_fee_offset`
    // This is equivalent to the circulating supply or Redeemable that this depository is hedging
    // Updated after each mint/redeem (/rebalance_fees/rebalance when implemented)
    // In Redeemable native units
    pub delta_neutral_quote_position: u128,
    //
    // Should add padding?
}

pub enum AccountingEvent {
    Deposit,
    Withdraw,
}

impl MangoDepository {
    pub fn sanity_check(&self) -> UxdResult {
        msg!(
            "redeemable_amount_under_management {}",
            self.redeemable_amount_under_management
        );
        msg!(
            "delta_neutral_quote_fee_offset {}",
            self.delta_neutral_quote_fee_offset
        );
        msg!(
            "delta_neutral_quote_position {}",
            self.delta_neutral_quote_position
        );
        let delta_neutral_quote_position_minus_fees = self
                .delta_neutral_quote_position
                .checked_sub(self.delta_neutral_quote_fee_offset)
                .unwrap();
        msg!(
            "delta_neutral_quote_position_minus_fees {} should equal redeemable_amount_under_management {} (diff {})",
            delta_neutral_quote_position_minus_fees, 
            self.redeemable_amount_under_management,
            self.redeemable_amount_under_management - delta_neutral_quote_position_minus_fees
        );
        if !(self.redeemable_amount_under_management
            == self
                .delta_neutral_quote_position
                .checked_sub(self.delta_neutral_quote_fee_offset)
                .unwrap())
        {
            return Err(ErrorCode::InvalidDepositoryAccounting);
        }
        Ok(())
    }

    pub fn update_insurance_amount_deposited(&mut self, event_type: &AccountingEvent, amount: u64) {
        self.insurance_amount_deposited = match event_type {
            AccountingEvent::Deposit => self
                .insurance_amount_deposited
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Withdraw => self
                .insurance_amount_deposited
                .checked_sub(amount.into())
                .unwrap(),
        }
    }

    pub fn update_collateral_amount_deposited(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) {
        self.collateral_amount_deposited = match event_type {
            AccountingEvent::Deposit => self
                .collateral_amount_deposited
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Withdraw => self
                .collateral_amount_deposited
                .checked_sub(amount.into())
                .unwrap(),
        }
    }

    pub fn update_redeemable_amount_under_management(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) {
        self.redeemable_amount_under_management = match event_type {
            AccountingEvent::Deposit => self
                .redeemable_amount_under_management
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Withdraw => self
                .redeemable_amount_under_management
                .checked_sub(amount.into())
                .unwrap(),
        }
    }

    pub fn update_delta_neutral_quote_fee_offset(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) {
        self.delta_neutral_quote_fee_offset = match event_type {
            AccountingEvent::Deposit => self
                .delta_neutral_quote_fee_offset
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Withdraw => self
                .delta_neutral_quote_fee_offset
                .checked_add(amount.into())
                .unwrap(),
        }
    }

    pub fn update_delta_neutral_quote_position(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) {
        self.delta_neutral_quote_position = match event_type {
            AccountingEvent::Deposit => self
                .delta_neutral_quote_position
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Withdraw => self
                .delta_neutral_quote_position
                .checked_sub(amount.into())
                .unwrap(),
        }
    }
}
