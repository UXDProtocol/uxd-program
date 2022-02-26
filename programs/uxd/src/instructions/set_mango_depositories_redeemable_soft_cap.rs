use crate::error::UxdError;
use crate::events::SetMangoDepositoryRedeemableSoftCapEvent;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP;
use anchor_lang::prelude::*;

/// Takes 2 accounts - 2 used locally - 0 for CPI - 0 Programs - 0 Sysvar
#[derive(Accounts)]
pub struct SetMangoDepositoriesRedeemableSoftCap<'info> {
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
    ctx: Context<SetMangoDepositoriesRedeemableSoftCap>,
    redeemable_soft_cap: u64,
) -> Result<()> {
    ctx.accounts
        .controller
        .mango_depositories_redeemable_soft_cap = redeemable_soft_cap;
    emit!(SetMangoDepositoryRedeemableSoftCapEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        redeemable_mint_decimals: ctx.accounts.controller.redeemable_mint_decimals,
        redeemable_mint: ctx.accounts.controller.redeemable_mint,
        redeemable_soft_cap
    });
    Ok(())
}

// Validate input arguments
#[allow(clippy::absurd_extreme_comparisons)]
impl<'info> SetMangoDepositoriesRedeemableSoftCap<'info> {
    // Asserts that the Mango Depositories redeemable soft cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
    pub fn validate(&self, redeemable_soft_cap: u64) -> Result<()> {
        if redeemable_soft_cap > MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP {
            return Err(error!(UxdError::InvalidMangoDepositoriesRedeemableSoftCap));
        }
        Ok(())
    }
}
