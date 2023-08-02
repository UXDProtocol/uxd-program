use std::mem::size_of;

use crate::error::UxdError;
use crate::utils::checked_add_u128_and_i128;
use anchor_lang::prelude::*;

pub const MAX_REGISTERED_MERCURIAL_VAULT_DEPOSITORIES: usize = 4;
pub const MAX_REGISTERED_CREDIX_LP_DEPOSITORIES: usize = 4;

// Total should be 885 bytes
pub const CONTROLLER_RESERVED_SPACE: usize = 104;
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
    + 8 // _unused4
    + size_of::<Pubkey>() * MAX_REGISTERED_MERCURIAL_VAULT_DEPOSITORIES // registered_mercurial_vault_depositories
    + size_of::<u8>() // registered_mercurial_vault_depositories_count
    + size_of::<Pubkey>() * MAX_REGISTERED_CREDIX_LP_DEPOSITORIES // registered_credix_lp_depositories
    + size_of::<u8>() // registered_credix_lp_depositories_count
    + size_of::<u128>() // profits_total_collected
    + size_of::<u16>() // identity_depository_weight_bps
    + size_of::<u16>() // mercurial_vault_depository_weight_bps
    + size_of::<u16>() // credix_lp_depository_weight_bps
    + size_of::<Pubkey>() // identity_depository
    + size_of::<Pubkey>() // mercurial_vault_depository
    + size_of::<Pubkey>() // credix_lp_depository
    + size_of::<u64>() // limit_redeem_amount_per_day
    + size_of::<u64>() // recently_redeemed_amount
    + size_of::<i64>() // last_redeem_timestamp
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
    pub _unused4: [u8; 8],
    //
    // The Mercurial Vault Depositories registered with this Controller
    pub registered_mercurial_vault_depositories:
        [Pubkey; MAX_REGISTERED_MERCURIAL_VAULT_DEPOSITORIES],
    pub registered_mercurial_vault_depositories_count: u8,
    //
    // The Credix Lp Depositories registered with this Controller
    pub registered_credix_lp_depositories: [Pubkey; MAX_REGISTERED_CREDIX_LP_DEPOSITORIES],
    pub registered_credix_lp_depositories_count: u8,
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

    // Redeem limitation flags
    pub limit_outflow_amount_per_day: u64, // or limit_outflow_bps_per_day
    pub last_redeem_timestamp: i64,
    pub last_day_outflow_amount: u64,

    // For future usage
    pub _reserved: [u8; CONTROLLER_RESERVED_SPACE],
}

impl Controller {
    pub fn add_registered_mercurial_vault_depository_entry(
        &mut self,
        mercurial_vault_depository_id: Pubkey,
    ) -> Result<()> {
        let current_size = usize::from(self.registered_mercurial_vault_depositories_count);
        require!(
            current_size < MAX_REGISTERED_MERCURIAL_VAULT_DEPOSITORIES,
            UxdError::MaxNumberOfMercurialVaultDepositoriesRegisteredReached
        );
        // Increment registered Mercurial Pool Depositories count
        self.registered_mercurial_vault_depositories_count = self
            .registered_mercurial_vault_depositories_count
            .checked_add(1)
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Add the new Mercurial Vault Depository ID to the array of registered Depositories
        let new_entry_index = current_size;
        self.registered_mercurial_vault_depositories[new_entry_index] =
            mercurial_vault_depository_id;
        Ok(())
    }

    pub(crate) fn add_registered_credix_lp_depository_entry(
        &mut self,
        credix_lp_depository_id: Pubkey,
    ) -> Result<()> {
        let current_size = usize::from(self.registered_credix_lp_depositories_count);
        require!(
            current_size < MAX_REGISTERED_CREDIX_LP_DEPOSITORIES,
            UxdError::MaxNumberOfCredixLpDepositoriesRegisteredReached
        );
        // Increment registered Credix Lp Depositories count
        self.registered_credix_lp_depositories_count = self
            .registered_credix_lp_depositories_count
            .checked_add(1)
            .ok_or_else(|| error!(UxdError::MathError))?;
        // Add the new Credix Lp Depository ID to the array of registered Depositories
        let new_entry_index = current_size;
        self.registered_credix_lp_depositories[new_entry_index] = credix_lp_depository_id;
        Ok(())
    }

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
        self.profits_total_collected = self
            .profits_total_collected
            .checked_add(profits_collected.into())
            .ok_or(UxdError::MathError)?;
        Ok(())
    }
}
