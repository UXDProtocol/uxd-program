use crate::error::UxdError;
use anchor_lang::prelude::*;

pub const MAX_REGISTERED_MANGO_DEPOSITORIES: usize = 8;

pub const CONTROLLER_SPACE: usize = 8
    + 1
    + 1
    + 1
    + 32
    + 32
    + 1
    + (32 * MAX_REGISTERED_MANGO_DEPOSITORIES)
    + 1
    + 16
    + 8
    + 16
    + 8
    + 504;

#[account(zero_copy)]
#[repr(packed)]
pub struct Controller {
    pub bump: u8,
    pub redeemable_mint_bump: u8,
    // Version used
    pub version: u8,
    // The account with authority over the UXD stack
    pub authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub redeemable_mint_decimals: u8,
    //
    // The Mango Depositories registered with this Controller
    pub registered_mango_depositories: [Pubkey; MAX_REGISTERED_MANGO_DEPOSITORIES],
    pub registered_mango_depositories_count: u8,
    //
    // Progressive roll out and safety ----------
    //
    // The total amount of UXD that can be in circulation, variable
    //  in redeemable Redeemable Native Amount (careful, usually Mint express this in full token, UI amount, u64)
    pub redeemable_global_supply_cap: u128,
    //
    // The max amount of Redeemable affected by Mint and Redeem operations on `MangoDepository` instances, variable
    //  in redeemable Redeemable Native Amount
    pub mango_depositories_redeemable_soft_cap: u64,
    //
    // Accounting -------------------------------
    //
    // The actual circulating supply of Redeemable
    // This should always be equal to the sum of all Depositories' `redeemable_amount_under_management`
    //  in redeemable Redeemable Native Amount
    pub redeemable_circulating_supply: u128,
    // The max amount of Redeemable affected by quote Mint and Redeem operations on `MangoDepository` instances
    pub mango_depositories_quote_redeemable_soft_cap: u64,
}

impl Controller {
    pub fn add_registered_mango_depository_entry(
        &mut self,
        mango_depository_id: Pubkey,
    ) -> Result<()> {
        let current_size = usize::from(self.registered_mango_depositories_count);
        require!(
            current_size < MAX_REGISTERED_MANGO_DEPOSITORIES,
            UxdError::MaxNumberOfMangoDepositoriesRegisteredReached
        );
        // Increment registered Mango Depositories count
        self.registered_mango_depositories_count = self
            .registered_mango_depositories_count
            .checked_add(1)
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Add the new Mango Depository ID to the array of registered Depositories
        let new_entry_index = current_size;
        self.registered_mango_depositories[new_entry_index] = mango_depository_id;
        Ok(())
    }
}
