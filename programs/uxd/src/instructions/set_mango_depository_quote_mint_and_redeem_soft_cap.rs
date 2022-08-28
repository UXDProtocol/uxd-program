use crate::error::UxdError;
use crate::events::SetMangoDepositoryQuoteMintAndRedeemSoftCapEvent;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use anchor_lang::prelude::*;

/// Takes 3 accounts
#[derive(Accounts)]
pub struct SetMangoDepositoryQuoteMintAndRedeemSoftCap<'info> {
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

pub(crate) fn handler(
    ctx: Context<SetMangoDepositoryQuoteMintAndRedeemSoftCap>,
    quote_redeemable_soft_cap: u64, // in redeemable native units
) -> Result<()> {
    ctx.accounts
        .controller
        .load_mut()?
        .mango_depositories_quote_redeemable_soft_cap = quote_redeemable_soft_cap;
    emit!(SetMangoDepositoryQuoteMintAndRedeemSoftCapEvent {
        version: ctx.accounts.controller.load()?.version,
        controller: ctx.accounts.controller.key(),
        quote_mint_and_redeem_soft_cap: quote_redeemable_soft_cap
    });
    Ok(())
}
