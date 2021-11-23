use anchor_lang::prelude::*;

use crate::AccountingEvent;

#[account]
#[derive(Default)]
pub struct Controller {
    pub bump: u8,
    pub redeemable_mint_bump: u8,
    // The account that initialize this struct. Only this account can call permissionned instructions.
    pub authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub redeemable_mint_decimals: u8,
    //
    // Accounting -------------------------------
    //
    // The total amount of UXD that can be in circulation, variable, to limit risks, do progressive rollout.
    //  in redeemable Redeemable Native Amount (careful, usually Mint express this in full token, UI amount, u64)
    pub redeemable_global_supply_cap: u128,
    //
    // The actual circulating supply of Redeemable (Also available through TokenProgram info on the mint)
    // This should always be equal to the sum of all Depositories' `redeemable_under_management`
    //  in redeemable Redeemable Native Amount
    pub redeemable_circulating_supply: u128,
}

impl Controller {
    pub fn update_redeemable_circulating_supply(
        &mut self,
        event_type: AccountingEvent,
        amount: u64,
    ) {
        self.redeemable_circulating_supply = match event_type {
            AccountingEvent::Mint => self
                .redeemable_circulating_supply
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Redeem => self
                .redeemable_circulating_supply
                .checked_sub(amount.into())
                .unwrap(),
        }
    }
}
