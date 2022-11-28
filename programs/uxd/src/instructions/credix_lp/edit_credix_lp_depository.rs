use crate::error::UxdError;
use crate::events::SetCredixLpDepositoryMintingDisabledEvent;
use crate::events::SetCredixLpDepositoryMintingFeeInBpsEvent;
use crate::events::SetCredixLpDepositoryProfitTreasuryCollateralEvent;
use crate::events::SetCredixLpDepositoryRedeemableAmountUnderManagementCapEvent;
use crate::events::SetCredixLpDepositoryRedeemingFeeInBpsEvent;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditCredixLpDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_credix_lp_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3
    #[account(
        mut,
        seeds = [
            CREDIX_LP_DEPOSITORY_NAMESPACE,
            depository.load()?.credix_global_market_state.as_ref(),
            depository.load()?.collateral_mint.as_ref()
        ],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: AccountLoader<'info, CredixLpDepository>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EditCredixLpDepositoryFields {
    redeemable_amount_under_management_cap: Option<u128>,
    minting_fee_in_bps: Option<u8>,
    redeeming_fee_in_bps: Option<u8>,
    minting_disabled: Option<bool>,
    profit_treasury_collateral: Option<Pubkey>,
}

pub(crate) fn handler(
    ctx: Context<EditCredixLpDepository>,
    fields: &EditCredixLpDepositoryFields,
) -> Result<()> {
    let depository = &mut ctx.accounts.depository.load_mut()?;

    // optional: redeemable_amount_under_management_cap
    if let Some(redeemable_amount_under_management_cap) =
        fields.redeemable_amount_under_management_cap
    {
        msg!(
            "[edit_credix_lp_depository] redeemable_amount_under_management_cap {}",
            redeemable_amount_under_management_cap
        );
        depository.redeemable_amount_under_management_cap = redeemable_amount_under_management_cap;
        emit!(
            SetCredixLpDepositoryRedeemableAmountUnderManagementCapEvent {
                version: ctx.accounts.controller.load()?.version,
                controller: ctx.accounts.controller.key(),
                depository: ctx.accounts.depository.key(),
                redeemable_amount_under_management_cap
            }
        );
    }

    // optional: minting_fee_in_bps
    if let Some(minting_fee_in_bps) = fields.minting_fee_in_bps {
        msg!(
            "[edit_credix_lp_depository] minting_fee_in_bps {}",
            minting_fee_in_bps
        );
        depository.minting_fee_in_bps = minting_fee_in_bps;
        emit!(SetCredixLpDepositoryMintingFeeInBpsEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            minting_fee_in_bps
        });
    }

    // optional: redeeming_fee_in_bps
    if let Some(redeeming_fee_in_bps) = fields.redeeming_fee_in_bps {
        msg!(
            "[edit_credix_lp_depository] redeeming_fee_in_bps {}",
            redeeming_fee_in_bps
        );
        depository.redeeming_fee_in_bps = redeeming_fee_in_bps;
        emit!(SetCredixLpDepositoryRedeemingFeeInBpsEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            redeeming_fee_in_bps
        });
    }

    // optional: minting_disabled
    if let Some(minting_disabled) = fields.minting_disabled {
        msg!(
            "[edit_credix_lp_depository] minting_disabled {}",
            minting_disabled
        );
        depository.minting_disabled = minting_disabled;
        emit!(SetCredixLpDepositoryMintingDisabledEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            minting_disabled
        });
    }

    // optional: profit_treasury_collateral
    if let Some(profit_treasury_collateral) = fields.profit_treasury_collateral {
        msg!(
            "[edit_credix_lp_depository] profit_treasury_collateral {}",
            profit_treasury_collateral
        );
        depository.profit_treasury_collateral = profit_treasury_collateral;
        emit!(SetCredixLpDepositoryProfitTreasuryCollateralEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            profit_treasury_collateral
        });
    }

    Ok(())
}