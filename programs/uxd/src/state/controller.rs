use anchor_lang::prelude::*;

use crate::{AccountingEvent, ErrorCode, MAX_REGISTERED_MANGO_DEPOSITORIES};

#[account]
#[derive(Default)]
pub struct Controller {
    pub bump: u8,
    pub redeemable_mint_bump: u8,
    // Version used - for migrations later if needed
    pub version: u8,
    // The account that initialize this struct. Only this account can call permissionned instructions.
    pub authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub redeemable_mint_decimals: u8,
    //
    // The Mango Depositories registered with this Controller
    pub registered_mango_depositories: [Pubkey; 8], // MAX_REGISTERED_MANGO_DEPOSITORIES - IDL bug with constant...
    pub registered_mango_depositories_count: u8,
    //
    // Progressive roll out and safety ----------
    //
    // The total amount of UXD that can be in circulation, variable
    //  in redeemable Redeemable Native Amount (careful, usually Mint express this in full token, UI amount, u64)
    pub redeemable_global_supply_cap: u128,
    //
    // The max ammount of Redeemable affected by Mint and Redeem operations on `MangoDepository` instances, variable
    //  in redeemable Redeemable Native Amount
    pub mango_depositories_redeemable_soft_cap: u64,
    //
    // Accounting -------------------------------
    //
    // The actual circulating supply of Redeemable
    // This should always be equal to the sum of all Depositories' `redeemable_amount_under_management`
    //  in redeemable Redeemable Native Amount
    pub redeemable_circulating_supply: u128,
    //
    _padding: ControllerPadding,
}

impl Controller {
    pub fn update_redeemable_circulating_supply(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) {
        self.redeemable_circulating_supply = match event_type {
            AccountingEvent::Deposit => self
                .redeemable_circulating_supply
                .checked_add(amount.into())
                .unwrap(),
            AccountingEvent::Withdraw => self
                .redeemable_circulating_supply
                .checked_sub(amount.into())
                .unwrap(),
        }
    }

    pub fn add_registered_mango_depository_entry(
        &mut self,
        mango_depository_id: Pubkey,
    ) -> ProgramResult {
        let current_size = usize::from(self.registered_mango_depositories_count);
        if !(current_size < MAX_REGISTERED_MANGO_DEPOSITORIES) {
            return Err(ErrorCode::MaxNumberOfMangoDepositoriesRegisteredReached.into());
        }
        // Increment registered Mango Depositories count
        self.registered_mango_depositories_count = self
            .registered_mango_depositories_count
            .checked_add(1)
            .unwrap();
        // Add the new Mango Depository ID to the array of registered Depositories
        let new_entry_index = current_size;
        self.registered_mango_depositories[new_entry_index] = mango_depository_id;
        Ok(())
    }
}

#[derive(Clone)]
pub struct ControllerPadding([u8; 1024]);

impl AnchorSerialize for ControllerPadding {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl AnchorDeserialize for ControllerPadding {
    fn deserialize(_: &mut &[u8]) -> Result<Self, std::io::Error> {
        Ok(Self([0u8; 1024]))
    }
}

impl Default for ControllerPadding {
    fn default() -> Self {
        ControllerPadding { 0: [0u8; 1024] }
    }
}
