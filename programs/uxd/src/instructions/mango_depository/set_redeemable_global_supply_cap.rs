use anchor_lang::prelude::*;

use crate::Controller;
use crate::ErrorCode;
use crate::CONTROLLER_NAMESPACE;

#[derive(Accounts)]
pub struct SetRedeemableGlobalSupplyCap<'info> {
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
    ctx: Context<SetRedeemableGlobalSupplyCap>,
    redeemable_global_supply_cap: u128, // native amount
) -> ProgramResult {
    ctx.accounts.controller.redeemable_global_supply_cap = redeemable_global_supply_cap;
    Ok(())
}