use crate::error::UxdError;
use crate::events::SetDepositories;
use crate::events::SetDepositoriesWeightBps;
use crate::events::SetRedeemableGlobalSupplyCapEvent;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::BPS_UNIT_CONVERSION;
use crate::CONTROLLER_NAMESPACE;
use crate::MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditController<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,
    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct EditControllerDepositoriesWeightBps {
    pub identity_depository_weight_bps: u16,
    pub mercurial_vault_depository_weight_bps: u16,
    pub credix_lp_depository_weight_bps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct EditControllerDepositories {
    pub identity_depository: Pubkey,
    pub mercurial_vault_depository: Pubkey,
    pub credix_lp_depository: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct EditControllerFields {
    pub redeemable_global_supply_cap: Option<u128>,
    pub depositories_weight_bps: Option<EditControllerDepositoriesWeightBps>,
    pub depositories: Option<EditControllerDepositories>,
}

pub(crate) fn handler(ctx: Context<EditController>, fields: &EditControllerFields) -> Result<()> {
    let controller = &mut ctx.accounts.controller.load_mut()?;

    // Optionally edit all depositories weights
    if let Some(depositories_weight_bps) = fields.depositories_weight_bps {
        let identity_depository_weight_bps = depositories_weight_bps.identity_depository_weight_bps;
        let mercurial_vault_depository_weight_bps =
            depositories_weight_bps.mercurial_vault_depository_weight_bps;
        let credix_lp_depository_weight_bps =
            depositories_weight_bps.credix_lp_depository_weight_bps;
        msg!(
            "[edit_controller] identity_depository_weight_bps {}",
            identity_depository_weight_bps
        );
        msg!(
            "[edit_controller] mercurial_vault_depository_weight_bps {}",
            mercurial_vault_depository_weight_bps
        );
        msg!(
            "[edit_controller] credix_lp_depository_weight_bps {}",
            credix_lp_depository_weight_bps
        );
        controller.identity_depository_weight_bps = identity_depository_weight_bps;
        controller.mercurial_vault_depository_weight_bps = mercurial_vault_depository_weight_bps;
        controller.credix_lp_depository_weight_bps = credix_lp_depository_weight_bps;
        emit!(SetDepositoriesWeightBps {
            version: controller.version,
            controller: ctx.accounts.controller.key(),
            identity_depository_weight_bps,
            mercurial_vault_depository_weight_bps,
            credix_lp_depository_weight_bps,
        });
    }

    // Optionally edit all depositories addresses
    if let Some(depositories) = fields.depositories {
        let identity_depository = depositories.identity_depository;
        let mercurial_vault_depository = depositories.mercurial_vault_depository;
        let credix_lp_depository = depositories.credix_lp_depository;
        msg!(
            "[edit_controller] identity_depository {}",
            identity_depository
        );
        msg!(
            "[edit_controller] mercurial_vault_depository {}",
            mercurial_vault_depository
        );
        msg!(
            "[edit_controller] credix_lp_depository {}",
            credix_lp_depository
        );
        controller.identity_depository = identity_depository;
        controller.mercurial_vault_depository = mercurial_vault_depository;
        controller.credix_lp_depository = credix_lp_depository;
        emit!(SetDepositories {
            version: controller.version,
            controller: ctx.accounts.controller.key(),
            identity_depository,
            mercurial_vault_depository,
            credix_lp_depository,
        });
    }

    // Optionally edit "redeemable_global_supply_cap"
    if let Some(redeemable_global_supply_cap) = fields.redeemable_global_supply_cap {
        msg!(
            "[edit_controller] redeemable_global_supply_cap {}",
            redeemable_global_supply_cap
        );
        controller.redeemable_global_supply_cap = redeemable_global_supply_cap;
        emit!(SetRedeemableGlobalSupplyCapEvent {
            version: controller.version,
            controller: ctx.accounts.controller.key(),
            redeemable_global_supply_cap
        });
    }
    Ok(())
}

#[allow(clippy::absurd_extreme_comparisons)]
impl<'info> EditController<'info> {
    pub(crate) fn validate(&self, fields: &EditControllerFields) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;

        // Validate the redeemable_global_supply_cap if specified
        if let Some(redeemable_global_supply_cap) = fields.redeemable_global_supply_cap {
            require!(
                redeemable_global_supply_cap <= MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP,
                UxdError::InvalidRedeemableGlobalSupplyCap
            );
        }

        // Validate the depositories_weight_bps if specified
        if let Some(depositories_weight_bps) = fields.depositories_weight_bps {
            let total_weight_bps = depositories_weight_bps
                .identity_depository_weight_bps
                .checked_add(depositories_weight_bps.mercurial_vault_depository_weight_bps)
                .ok_or(UxdError::MathError)?
                .checked_add(depositories_weight_bps.credix_lp_depository_weight_bps)
                .ok_or(UxdError::MathError)?;
            require!(
                u64::from(total_weight_bps) == BPS_UNIT_CONVERSION,
                UxdError::InvalidDepositoriesWeightBps
            );
        }

        Ok(())
    }
}
