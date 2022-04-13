use crate::error::UxdError;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::state::MangoDepository;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetMangoDepositoryQuoteMintAndRedeemFee<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #3 UXDProgram on chain account bound to a Controller instance.
    /// The `MangoDepository` manages a MangoAccount for a single Collateral.
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: Box<Account<'info, MangoDepository>>,
}

pub fn handler(
    ctx: Context<SetMangoDepositoryQuoteMintAndRedeemFee>,
    quote_fee: u8, // in bps
) -> Result<()> {
    ctx.accounts
        .depository
        .quote_mint_and_redeem_fees = quote_fee;

    Ok(())
}