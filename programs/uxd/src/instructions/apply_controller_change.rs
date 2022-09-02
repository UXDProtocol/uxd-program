use crate::error::UxdError;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use anchor_lang::prelude::*;
use std::cell::RefMut;

#[derive(Accounts)]
pub struct ApplyControllerChange<'info> {
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

pub fn handler<F>(ctx: Context<ApplyControllerChange>, modifier: F) -> Result<()>
where
    F: FnOnce(&Context<ApplyControllerChange>, &mut RefMut<Controller>) -> (),
{
    let controller = &mut ctx.accounts.controller.load_mut()?;
    modifier(&ctx, controller);
    Ok(())
}
