use crate::error::UxdError;
use crate::events::SetRedeemableGlobalSupplyCapEvent;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use anchor_lang::prelude::*;

/// Takes 2 accounts - 2 used locally - 0 for CPI - 0 Programs - 0 Sysvar
#[derive(Accounts)]
pub struct SetRedeemableGlobalSupplyCap<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,
}

pub fn handler(
    ctx: Context<SetRedeemableGlobalSupplyCap>,
    redeemable_global_supply_cap: u128,
) -> Result<()> {
    ctx.accounts.controller.redeemable_global_supply_cap = redeemable_global_supply_cap;
    emit!(SetRedeemableGlobalSupplyCapEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        redeemable_global_supply_cap
    });
    Ok(())
}

// Validate input arguments
#[allow(clippy::absurd_extreme_comparisons)]
impl<'info> SetRedeemableGlobalSupplyCap<'info> {
    // Asserts that the redeemable global supply cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
    pub fn validate(&self, redeemable_global_supply_cap: u128) -> Result<()> {
        if redeemable_global_supply_cap <= MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP {
            error!(UxdError::InvalidRedeemableGlobalSupplyCap);
        }

        Ok(())
    }
}
