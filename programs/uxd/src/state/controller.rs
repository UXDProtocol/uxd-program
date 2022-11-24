use crate::error::UxdError;
use anchor_lang::prelude::*;
use fixed::types::I80F48;

pub const MAX_REGISTERED_MANGO_DEPOSITORIES: usize = 8;
pub const MAX_REGISTERED_MERCURIAL_VAULT_DEPOSITORIES: usize = 4;
pub const MAX_REGISTERED_MAPLE_POOL_DEPOSITORIES: usize = 4;
pub const MAX_REGISTERED_CREDIX_LP_DEPOSITORIES: usize = 4;

// Total should be 885 bytes
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
    + (32 * MAX_REGISTERED_MERCURIAL_VAULT_DEPOSITORIES)
    + 1
    + (32 * MAX_REGISTERED_MAPLE_POOL_DEPOSITORIES)
    + 1
    + (32 * MAX_REGISTERED_CREDIX_LP_DEPOSITORIES)
    + 1
    + 117;

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
    //
    // The Mercurial Vault Depositories registered with this Controller
    pub registered_mercurial_vault_depositories:
        [Pubkey; MAX_REGISTERED_MERCURIAL_VAULT_DEPOSITORIES],
    pub registered_mercurial_vault_depositories_count: u8,
    //
    // The Maple Pool Depositories registered with this Controller
    pub registered_maple_pool_depositories: [Pubkey; MAX_REGISTERED_MAPLE_POOL_DEPOSITORIES],
    pub registered_maple_pool_depositories_count: u8,
    //
    // The Credix Lp Depositories registered with this Controller
    pub registered_credix_lp_depositories: [Pubkey; MAX_REGISTERED_CREDIX_LP_DEPOSITORIES],
    pub registered_credix_lp_depositories_count: u8,
}

impl Controller {
    pub(crate) fn add_registered_mango_depository_entry(
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

    pub fn add_registered_mercurial_vault_depository_entry(
        &mut self,
        mercurial_vault_depository_id: Pubkey,
    ) -> Result<()> {
        let current_size = usize::from(self.registered_mercurial_vault_depositories_count);
        require!(
            current_size < MAX_REGISTERED_MERCURIAL_VAULT_DEPOSITORIES,
            UxdError::MaxNumberOfMercurialVaultDepositoriesRegisteredReached
        );
        // Increment registered Mercurial Vault Depositories count
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

    pub fn add_registered_maple_pool_depository_entry(
        &mut self,
        maple_pool_depository: Pubkey,
    ) -> Result<()> {
        let current_size = usize::from(self.registered_maple_pool_depositories_count);
        require!(
            current_size < MAX_REGISTERED_MAPLE_POOL_DEPOSITORIES,
            UxdError::MaxNumberOfMaplePoolDepositoriesRegisteredReached
        );
        // Increment registered Maple Pool Depositories count
        self.registered_maple_pool_depositories_count = self
            .registered_maple_pool_depositories_count
            .checked_add(1)
            .ok_or(UxdError::MathError)?;
        // Add the new Maple Pool Depository ID to the array of registered Depositories
        let new_entry_index = current_size;
        self.registered_maple_pool_depositories[new_entry_index] = maple_pool_depository;
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
        self.redeemable_circulating_supply =
            I80F48::checked_from_num(self.redeemable_circulating_supply)
                .ok_or(UxdError::MathError)?
                .checked_add(
                    I80F48::checked_from_num(redeemable_amount_change)
                        .ok_or(UxdError::MathError)?,
                )
                .ok_or(UxdError::MathError)?
                .checked_to_num()
                .ok_or(UxdError::MathError)?;

        if redeemable_amount_change > 0 {
            require!(
                self.redeemable_circulating_supply <= self.redeemable_global_supply_cap,
                UxdError::RedeemableGlobalSupplyCapReached
            );
        }

        Ok(())
    }
}
