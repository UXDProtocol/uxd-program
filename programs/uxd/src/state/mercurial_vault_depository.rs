use anchor_lang::prelude::*;
use fixed::types::I80F48;

use crate::error::UxdError;

// Total should be 900 bytes
pub const MERCURIAL_VAULT_RESERVED_SPACE: usize = 588;
pub const MERCURIAL_VAULT_DEPOSITORY_SPACE: usize = 8
    + 1     // bump
    + 1     // version
    + 32    // collateral mint
    + 1     // collateral mint decimals
    + 32    // controller
    + 16    // collateral amount deposited
    + 16    // redeemable amount under management
    + 32    // mercurial vault
    + 32    // mercurial vault lp mint
    + 1     // mercurial vault lp mint decimals
    + 32    // lp token vault
    + 1     // lp token vault bump
    + 1     // minting fee in bps
    + 1     // redeeming fee in bps
    + 16    // minting fee total accrued
    + 16    // redeeming fee total accrued
    + 16    // redeemable amount under management cap
    + 1     // minting disabled
    + 16    // profits total collected
    + 8     // last time profits got collected (unix timestamp)
    + 32     // profits beneficiary
    + MERCURIAL_VAULT_RESERVED_SPACE;

#[account(zero_copy)]
#[repr(packed)]
pub struct MercurialVaultDepository {
    pub bump: u8,
    pub version: u8,

    // Token deposited in the vault
    pub collateral_mint: Pubkey,
    pub collateral_mint_decimals: u8,

    // The Controller instance for which this Depository works for
    pub controller: Pubkey,

    // The amount of collateral deposited by users to mint UXD
    // Updated after each mint/redeem
    // In Collateral native units
    pub collateral_amount_deposited: u128,

    // The amount of minted redeemable using this repository
    // Equals to collateral_amount_deposited, minus precision loss
    pub redeemable_amount_under_management: u128,

    // mercurial_vault linked to the depository
    pub mercurial_vault: Pubkey,

    // LP tokens received in exchange for depositing collateral
    pub mercurial_vault_lp_mint: Pubkey,
    pub mercurial_vault_lp_mint_decimals: u8,

    // Token account holding the LP tokens minted by depositing collateral on mercurial vault
    pub lp_token_vault: Pubkey,
    pub lp_token_vault_bump: u8,

    // Fee applied at minting, expressed in basis point (bps) and taken by minting less redeemable for the user.
    // E.g, with a minting fee of 5 bps, if the user mint for 1_000_000 USDC (6 decimals), it should receive 999_500 UXD (6 decimals)
    // Calculation: (10_000 - 5) * 1_000_000 / 10_000
    pub minting_fee_in_bps: u8,

    // Fee applied at redeeming, expressed in basis point (bps) and taken by redeeming less lp token from the mercurial vault
    // thus sending less collateral to the user.
    // E.g, with a redeeming fee of 5 bps, if the user redeem for 1_000_000 UXD (6 decimals), it should receive 999_500 USDC (6 decimals)
    // Calculation: (10_000 - 5) * 1_000_000 / 10_000
    pub redeeming_fee_in_bps: u8,

    // The amount of fees accrued from minting
    // Expressed in redeemable mint decimals (6)
    pub minting_fee_total_accrued: u128,

    // The amount of fees accrued from redeeming
    // Expressed in redeemable mint decimals (6)
    pub redeeming_fee_total_accrued: u128,

    // The total amount of circulating UXD originating from that depository
    pub redeemable_amount_under_management_cap: u128,

    pub minting_disabled: bool,

    // Total amount of interests collected by interests_and_fees_redeem_authority
    pub profits_total_collected: u128,

    // Worth 0 if interests and fees never got collected
    pub last_profits_collection_unix_timestamp: u64,

    // Receiver of the depository's profits
    pub profits_beneficiary_key: Pubkey,
}

impl MercurialVaultDepository {
    pub fn update_onchain_accounting_following_profits_collection(
        &mut self,
        collected_profits: u64,
        current_time_as_unix_timestamp: u64,
    ) -> std::result::Result<(), UxdError> {
        self.profits_total_collected = self
            .profits_total_collected
            .checked_add(collected_profits.into())
            .ok_or(UxdError::MathError)?;

        self.last_profits_collection_unix_timestamp = current_time_as_unix_timestamp;

        Ok(())
    }

    // provides numbers + or - depending on the change
    pub fn update_onchain_accounting_following_mint_or_redeem(
        &mut self,
        collateral_amount_deposited_change: i128,
        redeemable_amount_change: i128,
        paid_minting_fees_change: i128,
        paid_redeeming_fees_change: i128,
    ) -> std::result::Result<(), UxdError> {
        self.collateral_amount_deposited =
            I80F48::checked_from_num(self.collateral_amount_deposited)
                .ok_or(UxdError::MathError)?
                .checked_add(
                    I80F48::checked_from_num(collateral_amount_deposited_change)
                        .ok_or(UxdError::MathError)?,
                )
                .ok_or(UxdError::MathError)?
                .checked_to_num()
                .ok_or(UxdError::MathError)?;

        self.redeemable_amount_under_management =
            I80F48::checked_from_num(self.redeemable_amount_under_management)
                .ok_or(UxdError::MathError)?
                .checked_add(
                    I80F48::checked_from_num(redeemable_amount_change)
                        .ok_or(UxdError::MathError)?,
                )
                .ok_or(UxdError::MathError)?
                .checked_to_num()
                .ok_or(UxdError::MathError)?;

        self.minting_fee_total_accrued = I80F48::checked_from_num(self.minting_fee_total_accrued)
            .ok_or(UxdError::MathError)?
            .checked_add(
                I80F48::checked_from_num(paid_minting_fees_change).ok_or(UxdError::MathError)?,
            )
            .ok_or(UxdError::MathError)?
            .checked_to_num()
            .ok_or(UxdError::MathError)?;

        self.redeeming_fee_total_accrued =
            I80F48::checked_from_num(self.redeeming_fee_total_accrued)
                .ok_or(UxdError::MathError)?
                .checked_add(
                    I80F48::checked_from_num(paid_redeeming_fees_change)
                        .ok_or(UxdError::MathError)?,
                )
                .ok_or(UxdError::MathError)?
                .checked_to_num()
                .ok_or(UxdError::MathError)?;

        Ok(())
    }
}
