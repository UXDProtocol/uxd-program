use crate::error::UxdError;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditControllerAuthority<'info> {
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

pub(crate) fn handler(ctx: Context<EditControllerAuthority>, authority: &Pubkey) -> Result<()> {
    let controller = &mut ctx.accounts.controller.load_mut()?;
    controller.authority = *authority;
    Ok(())
}

impl<'info> EditControllerAuthority<'info> {
    pub(crate) fn validate(&self, authority: &Pubkey) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
