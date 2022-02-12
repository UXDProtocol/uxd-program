use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::events::SetRedeemableGlobalSupplyCapEvent;
use crate::Controller;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use anchor_lang::prelude::*;

declare_check_assert_macros!(SourceFileId::InstructionSetRedeemableGlobalSupplyCap);

#[derive(Accounts)]
pub struct SetRedeemableGlobalSupplyCap<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump,
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

// Validate input arguments
#[allow(clippy::absurd_extreme_comparisons)]
impl<'info> SetRedeemableGlobalSupplyCap<'info> {
    // Asserts that the redeemable global supply cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
    pub fn validate(&self, redeemable_global_supply_cap: u128) -> ProgramResult {
        check!(
            redeemable_global_supply_cap <= MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP,
            UxdErrorCode::InvalidRedeemableGlobalSupplyCap
        )?;
        Ok(())
    }
}
