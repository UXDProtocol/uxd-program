use crate::error::UxdError;
use anchor_lang::prelude::*;

use super::DepositoryConfiguration;

pub trait DepositoryAccounting: DepositoryConfiguration {
    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    fn get_deposited_collateral_amount(&self) -> u128;
    fn set_deposited_collateral_amount(&mut self, value: u128);

    // The amount of minted redeemable using this repository
    // Equals to deposited_collateral_amount, minus precision loss
    fn get_minted_redeemable_amount(&self) -> u128;
    fn set_minted_redeemable_amount(&mut self, value: u128);

    // The amount of fees accrued from minting
    // Expressed in redeemable mint decimals (6)
    fn get_minting_fees_total_paid(&self) -> u128;
    fn set_minting_fees_total_paid(&mut self, value: u128);
    // The amount of fees accrued from redeeming
    // Expressed in redeemable mint decimals (6)
    fn get_redeeming_fees_total_paid(&self) -> u128;
    fn set_redeeming_fees_total_paid(&mut self, value: u128);

    // Accounting when depositing/minting
    fn deposited_collateral_and_minted_redeemable(
        &mut self,
        deposited_collateral_amount: u64,
        minted_redeemable_amount: u64,
    ) -> Result<()> {
        require!(
            deposited_collateral_amount > 0,
            UxdError::InvalidCollateralAmount,
        );
        require!(
            minted_redeemable_amount > 0,
            UxdError::MinimumMintedRedeemableAmountError
        );
        // Actually add the recent change
        self.set_deposited_collateral_amount(
            self.get_deposited_collateral_amount()
                .checked_add(deposited_collateral_amount.into())
                .ok_or(UxdError::MathError)?,
        );
        self.set_minted_redeemable_amount(
            self.get_minted_redeemable_amount()
                .checked_add(minted_redeemable_amount.into())
                .ok_or(UxdError::MathError)?,
        );
        // Check that we're not minting past the soft cap
        require!(
            self.get_minted_redeemable_amount() <= self.get_minted_redeemable_soft_cap(),
            UxdError::DepositoryRedeemableSoftCapOverflow
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
        self.set_deposited_collateral_amount(
            self.get_deposited_collateral_amount()
                .checked_sub(withdrawn_collateral_amount.into())
                .ok_or(UxdError::MathError)?,
        );
        self.set_minted_redeemable_amount(
            self.get_minted_redeemable_amount()
                .checked_sub(burned_redeemable_amount.into())
                .ok_or(UxdError::MathError)?,
        );
        Ok(())
    }

    // Account for paid fees during minting
    fn increase_minting_fees_total_paid(&mut self, minting_fees_paid: u64) -> Result<()> {
        self.set_minting_fees_total_paid(
            self.get_minting_fees_total_paid()
                .checked_add(minting_fees_paid.into())
                .ok_or(UxdError::MathError)?,
        );
        Ok(())
    }

    // Account for paid fees during redeeming
    fn increase_redeeming_fees_total_paid(&mut self, redeeming_fees_paid: u64) -> Result<()> {
        self.set_redeeming_fees_total_paid(
            self.get_redeeming_fees_total_paid()
                .checked_add(redeeming_fees_paid.into())
                .ok_or(UxdError::MathError)?,
        );
        Ok(())
    }
}
