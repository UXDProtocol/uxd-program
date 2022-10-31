use crate::error::UxdError;
use crate::events::SetRedeemableGlobalSupplyCapEvent;
use crate::Controller;
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

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct EditControllerFields {
    redeemable_global_supply_cap: Option<u128>,
}

pub(crate) fn handler(ctx: Context<EditController>, fields: &EditControllerFields) -> Result<()> {
    let controller = &mut ctx.accounts.controller.load_mut()?;
    // Optionally edit "redeemable_global_supply_cap"
    if let Some(redeemable_global_supply_cap) = fields.redeemable_global_supply_cap {
        msg!("[set_redeemable_global_supply_cap]");
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
        if let Some(redeemable_global_supply_cap) = fields.redeemable_global_supply_cap {
            require!(
                redeemable_global_supply_cap <= MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP,
                UxdError::InvalidRedeemableGlobalSupplyCap
            );
        }
        Ok(())
    }
}
