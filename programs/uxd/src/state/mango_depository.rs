use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct MangoDepository {
    pub bump: u8,
    pub collateral_passthrough_bump: u8,
    pub mango_account_bump: u8,
    pub collateral_mint: Pubkey,
    pub collateral_passthrough: Pubkey,
    pub mango_account: Pubkey,
    //
    // Accounting -------------------------------
    // Note : To keep track of the in and out of a depository
    // Note : collateral and base are technically interchangeable as one Depository manage a single collateral
    //
    // The amount of USDC InsuranceFund deposited/withdrawn by Authority on the underlying Mango Account - It doesn't represent the actual amount that varies based on Mango Account
    // Updated after each deposit/withdraw insurance fund
    // In Collateral native units
    pub insurance_amount_deposited: u128,
    // The amount of collateral deposited by users to mint UXD - The optimal size of the basis trade
    // Updated after each mint/redeem
    // In Collateral native units
    pub collateral_amount_deposited: u128,
    // The total amount of Redeemable Tokens this Depository instance is currently Hedging/Managing
    // Updated after each mint/redeem
    // In Redeemable native units
    pub redeemable_amount_under_management: u128,
}

pub enum AccountingEvent {
    Mint,
    Redeem,
}

impl MangoDepository {
    // pub fn update_insurance_deposited(&mut self, event_type: BalanceUpdateType, amount: u64) {
    //     self.insurance_amount_deposited = match event_type {
    //         BalanceUpdateType::Deposit => {
    //             self.insurance_amount_deposited.checked_add(amount.into()).unwrap()
    //         }
    //         BalanceUpdateType::Withdraw => {
    //             self.insurance_amount_deposited.checked_sub(amount.into()).unwrap()
    //         }
    //     }
    // }

    pub fn update_collateral_amount_deposited(&mut self, event_type: AccountingEvent, amount: u64) {
        self.collateral_amount_deposited = match event_type {
            AccountingEvent::Mint => self
                .collateral_amount_deposited
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Redeem => self
                .collateral_amount_deposited
                .checked_sub(amount.into())
                .unwrap(),
        }
    }

    pub fn update_redeemable_amount_under_management(
        &mut self,
        event_type: AccountingEvent,
        amount: u64,
    ) {
        self.redeemable_amount_under_management = match event_type {
            AccountingEvent::Mint => self
                .redeemable_amount_under_management
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Redeem => self
                .redeemable_amount_under_management
                .checked_sub(amount.into())
                .unwrap(),
        }
    }
}
