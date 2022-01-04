use anchor_lang::prelude::*;
use crate::{Controller, UxdResult};
use crate::error::UxdIdlErrorCode;
use crate::CONTROLLER_NAMESPACE;
use crate::events::SetMangoDepositoryRedeemableSoftCapEvent;

#[derive(Accounts)]
pub struct SetMangoDepositoriesRedeemableSoftCap<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,
}

pub fn handler(
    ctx: Context<SetMangoDepositoriesRedeemableSoftCap>,
    redeemable_soft_cap: u64, // native amount
) -> UxdResult {
    ctx.accounts.controller.mango_depositories_redeemable_soft_cap = redeemable_soft_cap;
    emit!(SetMangoDepositoryRedeemableSoftCapEvent {
        controller: ctx.accounts.controller.key(),
        redeemable_soft_cap
    });
    Ok(())
}