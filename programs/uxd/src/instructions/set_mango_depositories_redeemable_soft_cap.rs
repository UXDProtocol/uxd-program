use anchor_lang::prelude::*;
use crate::{Controller, UxdResult, MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP};
use crate::error::{check_assert, UxdErrorCode};
use crate::error::SourceFileId;
use crate::error::UxdIdlErrorCode;
use crate::CONTROLLER_NAMESPACE;
use crate::events::SetMangoDepositoryRedeemableSoftCapEvent;

declare_check_assert_macros!(SourceFileId::InstructionSetMangoDepositoriesRedeemableSoftCap);

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
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        redeemable_mint_decimals: ctx.accounts.controller.redeemable_mint_decimals,
        redeemable_mint: ctx.accounts.controller.redeemable_mint,
        redeemable_soft_cap
    });
    Ok(())
}

impl<'info> SetMangoDepositoriesRedeemableSoftCap<'info> {
    // Asserts that the Mango Depositories redeemable soft cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
    pub fn validate(
        &self,
        redeemable_soft_cap: u64,
    ) -> ProgramResult {
        check!(
            redeemable_soft_cap <= MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP,
            UxdErrorCode::InvalidMangoDepositoriesRedeemableSoftCap
        )?;
        Ok(())
    }
}
