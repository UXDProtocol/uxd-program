use anchor_lang::prelude::*;
use crate::Controller;
use crate::UxdResult;
use crate::MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use crate::MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP;
use crate::error::check_assert;
use crate::error::UxdErrorCode;
use crate::error::SourceFileId;
use crate::error::UxdIdlErrorCode;
use crate::CONTROLLER_NAMESPACE;
use crate::events::UpdateProgramSettingsEvent;
use crate::borsh::BorshSerialize;
use crate::borsh::BorshDeserialize;

declare_check_assert_macros!(SourceFileId::InstructionUpdateProgramSettings);

#[derive(Accounts)]
pub struct UpdateProgramSettings<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub struct UpdateProgramSettingsArgs {
    pub redeemable_soft_cap: Option<u64>,
    pub redeemable_global_supply_cap: Option<u128>,
}

pub fn handler(
    ctx: Context<UpdateProgramSettings>,
    args: UpdateProgramSettingsArgs,
) -> UxdResult {
    // Check to see if a setting was inputted to be updataed. Else, keep last value.
    ctx.accounts.controller.mango_depositories_redeemable_soft_cap = args.redeemable_soft_cap.unwrap_or(
        ctx.accounts.controller.mango_depositories_redeemable_soft_cap
    );
    ctx.accounts.controller.redeemable_global_supply_cap = args.redeemable_global_supply_cap.unwrap_or(
        ctx.accounts.controller.redeemable_global_supply_cap
    );

    emit!(UpdateProgramSettingsEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        redeemable_mint: ctx.accounts.controller.redeemable_mint,
        redeemable_mint_decimals: ctx.accounts.controller.redeemable_mint_decimals,
        redeemable_soft_cap: args.redeemable_soft_cap.unwrap_or(
            ctx.accounts.controller.mango_depositories_redeemable_soft_cap
        ),
        redeemable_global_supply_cap: args.redeemable_global_supply_cap.unwrap_or(
            ctx.accounts.controller.redeemable_global_supply_cap
        ),
    });
    Ok(())
}

// Validate : There is one validate function per program setting.
impl<'info> UpdateProgramSettings<'info> {
    // Asserts that the Mango Depositories redeemable soft cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
    pub fn validate_set_mango_depositories_redeemable_soft_cap(
        &self,
        redeemable_soft_cap: u64,
    ) -> ProgramResult {
        check!(
            redeemable_soft_cap <= MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP,
            UxdErrorCode::InvalidMangoDepositoriesRedeemableSoftCap
        )?;
        Ok(())
    }

    // Asserts that the redeemable global supply cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
    pub fn validate_set_redeemable_global_supply_cap(
        &self,
        redeemable_global_supply_cap: u128,
    ) -> ProgramResult {
        check!(
            redeemable_global_supply_cap <= MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP,
            UxdErrorCode::InvalidRedeemableGlobalSupplyCap
        )?;
        Ok(())
    }
}
