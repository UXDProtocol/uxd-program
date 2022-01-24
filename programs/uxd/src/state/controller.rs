use crate::declare_check_assert_macros;
use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::AccountingEvent;
use crate::UxdResult;
use anchor_lang::prelude::*;

declare_check_assert_macros!(SourceFileId::StateController);

pub const MAX_REGISTERED_MANGO_DEPOSITORIES: usize = 8;
pub const MAX_REGISTERED_ZO_DEPOSITORIES: usize = 8;

#[account]
#[derive(Default)]
pub struct Controller {
    pub bump: u8,
    pub redeemable_mint_bump: u8,
    // Version used - for migrations later if needed
    pub version: u8,
    // The account with authority over the UXD stack
    pub authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub redeemable_mint_decimals: u8,
    //
    // The Mango Depositories registered with this Controller
    pub registered_mango_depositories: [Pubkey; 8], //  - IDL bug with constant, so hard 8 literal. -- Still not working in 0.20.0 although it should
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
    //
    // WARNING TODO Should add padding over having to migrate
    // Note : This is the last thing I'm working on and I would love some guidance from the audit. Anchor doesn't seems to play nice with padding
    pub _reserved: ControllerPadding,
    // The ZO Depositories registered with this Controller
    pub registered_zo_depositories: [Pubkey; 8], //  - IDL bug with constant, so hard 8 literal. -- Still not working in 0.20.0 although it should
    pub registered_zo_depositories_count: u8,
}

#[derive(Clone)]
pub struct ControllerPadding([u8; 255]);

impl AnchorSerialize for ControllerPadding {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl AnchorDeserialize for ControllerPadding {
    fn deserialize(_: &mut &[u8]) -> Result<Self, std::io::Error> {
        Ok(Self([0u8; 255]))
    }
}

impl Default for ControllerPadding {
    fn default() -> Self {
        ControllerPadding { 0: [0u8; 255] }
    }
}

impl Controller {
    pub fn update_redeemable_circulating_supply(
        &mut self,
        event_type: &AccountingEvent,
        amount: u64,
    ) -> UxdResult {
        self.redeemable_circulating_supply = match event_type {
            AccountingEvent::Deposit => self
                .redeemable_circulating_supply
                .checked_add(amount.into())
                .ok_or(math_err!())?,
            AccountingEvent::Withdraw => self
                .redeemable_circulating_supply
                .checked_sub(amount.into())
                .ok_or(math_err!())?,
        };
        Ok(())
    }

    pub fn add_registered_mango_depository_entry(
        &mut self,
        mango_depository_id: Pubkey,
    ) -> ProgramResult {
        let current_size = usize::from(self.registered_mango_depositories_count);
        check!(
            current_size < MAX_REGISTERED_MANGO_DEPOSITORIES,
            UxdErrorCode::MaxNumberOfMangoDepositoriesRegisteredReached
        )?;
        // Increment registered Mango Depositories count
        self.registered_mango_depositories_count = self
            .registered_mango_depositories_count
            .checked_add(1)
            .ok_or(math_err!())?;
        // Add the new Mango Depository ID to the array of registered Depositories
        let new_entry_index = current_size;
        self.registered_mango_depositories[new_entry_index] = mango_depository_id;
        Ok(())
    }

    pub fn add_registered_zo_depository_entry(
        &mut self,
        zo_depository_id: Pubkey,
    ) -> ProgramResult {
        let current_size = usize::from(self.registered_zo_depositories_count);
        check!(
            current_size < MAX_REGISTERED_ZO_DEPOSITORIES,
            UxdErrorCode::MaxNumberOfZoDepositoriesRegisteredReached
        )?;
        // Increment registered ZO Depositories count
        self.registered_zo_depositories_count = self
            .registered_zo_depositories_count
            .checked_add(1)
            .ok_or(math_err!())?;
        // Add the new ZO Depository ID to the array of registered Depositories
        let new_entry_index = current_size;
        self.registered_zo_depositories[new_entry_index] = zo_depository_id;
        Ok(())
    }
}
