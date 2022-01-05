use anchor_lang::prelude::*;
use crate::{Controller, UxdResult};
use crate::error::UxdIdlErrorCode;
use crate::CONTROLLER_NAMESPACE;
use crate::events::SetRedeemableGlobalSupplyCapEvent;

#[derive(Accounts)]
pub struct SetRedeemableGlobalSupplyCap<'info> {
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
    ctx: Context<SetRedeemableGlobalSupplyCap>,
    redeemable_global_supply_cap: u128, // native amount
) -> UxdResult {
    ctx.accounts.controller.redeemable_global_supply_cap = redeemable_global_supply_cap;
    emit!(SetRedeemableGlobalSupplyCapEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        redeemable_global_supply_cap
    });
    Ok(())
}