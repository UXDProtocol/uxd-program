use crate::error::UxdError;
use anchor_lang::prelude::*;

use super::DepositoryConfiguration;

pub trait DepositoryAccounting: DepositoryConfiguration {
    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    fn get_collateral_amount_deposited(&self) -> u128;
    fn set_collateral_amount_deposited(&mut self, value: u128);

    // The amount of minted redeemable using this repository
    // Equals to collateral_amount_deposited, minus precision loss
    fn get_redeemable_amount_under_management(&self) -> u128;
    fn set_redeemable_amount_under_management(&mut self, value: u128);

    // The amount of fees accrued from minting
    // Expressed in redeemable mint decimals (6)
    fn get_total_paid_minting_fee(&self) -> u128;
    fn set_total_paid_minting_fee(&mut self, value: u128);
    // The amount of fees accrued from redeeming
    // Expressed in redeemable mint decimals (6)
    fn get_total_paid_redeeming_fee(&self) -> u128;
    fn set_total_paid_redeeming_fee(&mut self, value: u128);

    // Accounting when depositing/minting
    fn deposited_collateral_and_minted_redeemable(
        &mut self,
        collateral_amount_deposited: u64,
        redeemable_amount_under_management: u64,
    ) -> Result<()> {
        require!(
            collateral_amount_deposited > 0,
            UxdError::InvalidCollateralAmount,
        );
        require!(
            redeemable_amount_under_management > 0,
            UxdError::MinimumMintedRedeemableAmountError
        );
        // Actually add the recent change
        self.set_collateral_amount_deposited(
            self.get_collateral_amount_deposited()
                .checked_add(collateral_amount_deposited.into())
                .ok_or(UxdError::MathError)?,
        );
        self.set_redeemable_amount_under_management(
            self.get_redeemable_amount_under_management()
                .checked_add(redeemable_amount_under_management.into())
                .ok_or(UxdError::MathError)?,
        );
        // Check that we're not minting past the cap
        require!(
            self.get_redeemable_amount_under_management()
                <= self.get_redeemable_amount_under_management_cap(),
            UxdError::DepositoryRedeemableCapOverflow
        );
        Ok(())
    }

    // Accounting when redeeming/burning
    fn withdrawn_collateral_and_burned_redeemable(
        &mut self,
        withdrawn_collateral_amount: u64,
        burned_redeemable_amount: u64,
    ) -> Result<()> {
        require!(
            withdrawn_collateral_amount > 0,
            UxdError::InvalidCollateralAmount,
        );
        require!(
            burned_redeemable_amount > 0,
            UxdError::MinimumMintedRedeemableAmountError
        );
        // Actually add the recent change
        self.set_collateral_amount_deposited(
            self.get_collateral_amount_deposited()
                .checked_sub(withdrawn_collateral_amount.into())
                .ok_or(UxdError::MathError)?,
        );
        self.set_redeemable_amount_under_management(
            self.get_redeemable_amount_under_management()
                .checked_sub(burned_redeemable_amount.into())
                .ok_or(UxdError::MathError)?,
        );
        Ok(())
    }

    // Account for paid fees during minting
    fn increase_total_paid_minting_fee(&mut self, paid_minting_fee: u64) -> Result<()> {
        self.set_total_paid_minting_fee(
            self.get_total_paid_minting_fee()
                .checked_add(paid_minting_fee.into())
                .ok_or(UxdError::MathError)?,
        );
        Ok(())
    }

    // Account for paid fees during redeeming
    fn increase_total_paid_redeeming_fee(&mut self, paid_redeeming_fee: u64) -> Result<()> {
        self.set_total_paid_redeeming_fee(
            self.get_total_paid_redeeming_fee()
                .checked_add(paid_redeeming_fee.into())
                .ok_or(UxdError::MathError)?,
        );
        Ok(())
    }
}
