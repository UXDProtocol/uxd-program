use anchor_lang::prelude::*;

use crate::Controller;
use crate::ErrorCode;
use crate::CONTROLLER_NAMESPACE;

#[derive(Accounts)]
pub struct SetMangoDepositoriesRedeemableSoftCap<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority @ErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,
}

pub fn handler(
    ctx: Context<SetMangoDepositoriesRedeemableSoftCap>,
    redeemable_soft_cap: u64, // In Redeemable Native amount
) -> ProgramResult {
    ctx.accounts.controller.mango_depositories_redeemable_soft_cap = redeemable_soft_cap;
    Ok(())
}