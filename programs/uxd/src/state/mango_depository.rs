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
    // The amount of taker fee paid in quote while placing perp orders
    pub total_amount_paid_taker_fee: u128,
    //
    // Should add padding?
}

pub enum AccountingEvent {
    Deposit,
    Withdraw,
}

impl MangoDepository {
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

    pub fn update_total_amount_paid_taker_fee(&mut self, amount: u64) {
        self.total_amount_paid_taker_fee = self
            .total_amount_paid_taker_fee
            .checked_add(amount.into())
            .unwrap();
    }
}
