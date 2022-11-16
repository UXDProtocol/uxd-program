use crate::error::UxdError;
use crate::events::SetMaplePoolDepositoryMintingDisabledEvent;
use crate::events::SetMaplePoolDepositoryMintingFeeInBpsEvent;
use crate::events::SetMaplePoolDepositoryRedeemableAmountUnderManagementCapEvent;
use crate::events::SetMaplePoolDepositoryRedeemingFeeInBpsEvent;
use crate::state::maple_pool_depository::MaplePoolDepository;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MAPLE_POOL_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditMaplePoolDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_maple_pool_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3
    #[account(
        mut,
        seeds = [
            MAPLE_POOL_DEPOSITORY_NAMESPACE,
            depository.load()?.maple_pool.as_ref(),
            depository.load()?.collateral_mint.as_ref()
        ],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: AccountLoader<'info, MaplePoolDepository>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EditMaplePoolDepositoryFields {
    redeemable_amount_under_management_cap: Option<u128>,
    minting_fee_in_bps: Option<u8>,
    redeeming_fee_in_bps: Option<u8>,
    minting_disabled: Option<bool>,
}

pub(crate) fn handler(
    ctx: Context<EditMaplePoolDepository>,
    fields: &EditMaplePoolDepositoryFields,
) -> Result<()> {
    let depository = &mut ctx.accounts.depository.load_mut()?;

    // optional: redeemable_amount_under_management_cap
    if let Some(redeemable_amount_under_management_cap) =
        fields.redeemable_amount_under_management_cap
    {
        msg!(
            "[edit_maple_pool_depository] redeemable_amount_under_management_cap {}",
            redeemable_amount_under_management_cap
        );
        depository.redeemable_amount_under_management_cap = redeemable_amount_under_management_cap;
        emit!(
            SetMaplePoolDepositoryRedeemableAmountUnderManagementCapEvent {
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
            "[edit_maple_pool_depository] minting_fee_in_bps {}",
            minting_fee_in_bps
        );
        depository.minting_fee_in_bps = minting_fee_in_bps;
        emit!(SetMaplePoolDepositoryMintingFeeInBpsEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            minting_fee_in_bps
        });
    }

    // optional: redeeming_fee_in_bps
    if let Some(redeeming_fee_in_bps) = fields.redeeming_fee_in_bps {
        msg!(
            "[edit_maple_pool_depository] redeeming_fee_in_bps {}",
            redeeming_fee_in_bps
        );
        depository.redeeming_fee_in_bps = redeeming_fee_in_bps;
        emit!(SetMaplePoolDepositoryRedeemingFeeInBpsEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            redeeming_fee_in_bps
        });
    }

    // optional: minting_disabled
    if let Some(minting_disabled) = fields.minting_disabled {
        msg!(
            "[edit_maple_pool_depository] minting_disabled {}",
            minting_disabled
        );
        depository.minting_disabled = minting_disabled;
        emit!(SetMaplePoolDepositoryMintingDisabledEvent {
            version: ctx.accounts.controller.load()?.version,
            controller: ctx.accounts.controller.key(),
            depository: ctx.accounts.depository.key(),
            minting_disabled
        });
    }

    Ok(())
}
