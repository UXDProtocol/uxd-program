use std::mem::size_of;

use crate::utils::checked_add;
use crate::utils::checked_add_u128_and_i128;
use anchor_lang::prelude::*;

// Total should be 885 bytes
pub const CONTROLLER_RESERVED_SPACE: usize = 60;
pub const CONTROLLER_SPACE: usize = 8
    + size_of::<u8>() // bump
    + size_of::<u8>() // redeemable_mint_bump
    + size_of::<u8>() // version
    + size_of::<Pubkey>() // authority
    + size_of::<Pubkey>() // redeemable_mint
    + size_of::<u8>() // redeemable_mint_decimals
    + 255 // _unused, Shh. Free real estate
    + size_of::<bool>() // is_frozen
    + 1 // _unused2
    + size_of::<u128>() // redeemable_global_supply_cap
    + 8 // _unused3
    + size_of::<u128>() // redeemable_circulating_supply
    + 266 // _unused4
    + size_of::<u128>() // profits_total_collected
    + size_of::<u16>() // identity_depository_weight_bps
    + size_of::<u16>() // mercurial_vault_depository_weight_bps
    + size_of::<u16>() // credix_lp_depository_weight_bps
    + size_of::<Pubkey>() // identity_depository
    + size_of::<Pubkey>() // mercurial_vault_depository
    + size_of::<Pubkey>() // credix_lp_depository
    + size_of::<u64>() // outflow_limit_per_epoch_amount
    + size_of::<u16>() // outflow_limit_per_epoch_bps
    + size_of::<u64>() // slots_per_epoch
    + size_of::<u64>() // epoch_outflow_amount
    + size_of::<u64>() // last_outflow_slot
    + size_of::<Pubkey>() // alloyx_vault_depository
    + size_of::<u16>() // alloyx_vault_depository_weight_bps
    + CONTROLLER_RESERVED_SPACE;

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
    pub _unused: [u8; 255],
    // operational status for all ixs associated with this controller instance
    pub is_frozen: bool,
    //
    pub _unused2: u8,
    //
    // Progressive roll out and safety ----------
    //
    // The total amount of UXD that can be in circulation, variable
    //  in redeemable Redeemable Native Amount (careful, usually Mint express this in full token, UI amount, u64)
    pub redeemable_global_supply_cap: u128,
    //
    // The max amount of Redeemable affected by Mint and Redeem operations on `Depository` instances, variable
    //  in redeemable Redeemable Native Amount
    pub _unused3: [u8; 8],
    //
    // Accounting -------------------------------
    //
    // The actual circulating supply of Redeemable
    // This should always be equal to the sum of all Depositories' `redeemable_amount_under_management`
    //  in redeemable Redeemable Native Amount
    pub redeemable_circulating_supply: u128,
    // Padding for data that is no longer needed
    pub _unused4: [u8; 266],
    //
    // Total amount of profits collected into the treasury by any depository
    pub profits_total_collected: u128,

    // The configured router depositories balancing weights
    pub identity_depository_weight_bps: u16,
    pub mercurial_vault_depository_weight_bps: u16,
    pub credix_lp_depository_weight_bps: u16,

    // The configured router depositories addresses
    pub identity_depository: Pubkey,
    pub mercurial_vault_depository: Pubkey,
    pub credix_lp_depository: Pubkey,

    // Flags needed for outflow limitation
    pub outflow_limit_per_epoch_amount: u64,
    pub outflow_limit_per_epoch_bps: u16,
    pub slots_per_epoch: u64,
    pub epoch_outflow_amount: u64,
    pub last_outflow_slot: u64,

    // The configured router depository for alloyx's vault
    pub alloyx_vault_depository: Pubkey,
    pub alloyx_vault_depository_weight_bps: u16,

    // For future usage
    pub _reserved: [u8; CONTROLLER_RESERVED_SPACE],
}

impl Controller {
    // provides numbers + or - depending on the change
    pub fn update_onchain_accounting_following_mint_or_redeem(
        &mut self,
        redeemable_amount_change: i128,
    ) -> Result<()> {
        self.redeemable_circulating_supply = checked_add_u128_and_i128(
            self.redeemable_circulating_supply,
            redeemable_amount_change,
        )?;
        Ok(())
    }

    // When collecting profits, we need to add it to the total
    pub fn update_onchain_accounting_following_profits_collection(
        &mut self,
        profits_collected: u64,
    ) -> Result<()> {
        self.profits_total_collected =
            checked_add(self.profits_total_collected, u128::from(profits_collected))?;
        Ok(())
    }
}
