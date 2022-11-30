use crate::error::UxdError;
use crate::events::FreezeProgramEvent;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct FreezeProgram<'info> {
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

pub(crate) fn handler(ctx: Context<FreezeProgram>, freeze: bool) -> Result<()> {
    let controller = &mut ctx.accounts.controller.load_mut()?;
    controller.is_frozen = freeze;
    emit!(FreezeProgramEvent {
        version: controller.version,
        controller: ctx.accounts.controller.key(),
        is_frozen: freeze
    });
    Ok(())
}

impl<'info> FreezeProgram<'info> {
    pub(crate) fn validate(&self, freeze: bool) -> Result<()> {
        require!(
            self.controller.load()?.is_frozen != freeze,
            UxdError::ProgramAlreadyFrozenOrResumed
        );
        Ok(())
    }
}
