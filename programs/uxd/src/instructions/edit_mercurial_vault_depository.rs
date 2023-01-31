use crate::error::UxdError;
use crate::events::SetDepositoryMintingDisabledEvent;
use crate::events::SetDepositoryMintingFeeInBpsEvent;
use crate::events::SetDepositoryProfitsBeneficiaryCollateralEvent;
use crate::events::SetDepositoryRedeemableAmountUnderManagementCapEvent;
use crate::events::SetDepositoryRedeemingFeeInBpsEvent;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditMercurialVaultDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_mercurial_vault_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3 UXDProgram on chain account bound to a Controller instance.
    /// The `MercurialVaultDepository` manages a MercurialVaultAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MERCURIAL_VAULT_DEPOSITORY_NAMESPACE, depository.load()?.mercurial_vault.as_ref(), depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: AccountLoader<'info, MercurialVaultDepository>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EditMercurialVaultDepositoryFields {
    redeemable_amount_under_management_cap: Option<u128>,
    minting_fee_in_bps: Option<u8>,
    redeeming_fee_in_bps: Option<u8>,
    minting_disabled: Option<bool>,
    profits_beneficiary_collateral: Option<Pubkey>,
}

pub(crate) fn handler(
    ctx: Context<EditMercurialVaultDepository>,
    fields: &EditMercurialVaultDepositoryFields,
) -> Result<()> {
    let depository = &mut ctx.accounts.depository.load_mut()?;

    // optional: redeemable_amount_under_management_cap
    if let Some(redeemable_amount_under_management_cap) =
        fields.redeemable_amount_under_management_cap
    {
        msg!(
            "[edit_mercurial_vault_depository] redeemable_amount_under_management_cap {}",
            redeemable_amount_under_management_cap
        );
        depository.redeemable_amount_under_management_cap = redeemable_amount_under_management_cap;
        emit!(SetDepositoryRedeemableAmountUnderManagementCapEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            redeemable_amount_under_management_cap
        });
    }

    // optional: minting_fee_in_bps
    if let Some(minting_fee_in_bps) = fields.minting_fee_in_bps {
        msg!(
            "[edit_mercurial_vault_depository] minting_fee_in_bps {}",
            minting_fee_in_bps
        );
        depository.minting_fee_in_bps = minting_fee_in_bps;
        emit!(SetDepositoryMintingFeeInBpsEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            minting_fee_in_bps
        });
    }

    // optional: redeeming_fee_in_bps
    if let Some(redeeming_fee_in_bps) = fields.redeeming_fee_in_bps {
        msg!(
            "[edit_mercurial_vault_depository] redeeming_fee_in_bps {}",
            redeeming_fee_in_bps
        );
        depository.redeeming_fee_in_bps = redeeming_fee_in_bps;
        emit!(SetDepositoryRedeemingFeeInBpsEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            redeeming_fee_in_bps
        });
    }

    // optional: minting_disabled
    if let Some(minting_disabled) = fields.minting_disabled {
        msg!(
            "[edit_mercurial_vault_depository] minting_disabled {}",
            minting_disabled
        );
        depository.minting_disabled = minting_disabled;
        emit!(SetDepositoryMintingDisabledEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            minting_disabled
        });
    }

    // optional: profits_beneficiary_collateral
    if let Some(profits_beneficiary_collateral) = fields.profits_beneficiary_collateral {
        msg!(
            "[edit_mercurial_vault_depository] profits_beneficiary_collateral {}",
            profits_beneficiary_collateral
        );
        depository.profits_beneficiary_collateral = profits_beneficiary_collateral;
        emit!(SetDepositoryProfitsBeneficiaryCollateralEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            profits_beneficiary_collateral
        });
    }

    Ok(())
}

impl<'info> EditMercurialVaultDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;

        Ok(())
    }
}
