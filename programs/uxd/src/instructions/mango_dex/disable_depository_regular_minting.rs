use crate::error::UxdError;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;

/// Takes 3 accounts
#[derive(Accounts)]
pub struct DisableDepositoryRegularMinting<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #3 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,
}

pub(crate) fn handler(ctx: Context<DisableDepositoryRegularMinting>, disable: bool) -> Result<()> {
    let depository = &mut ctx.accounts.depository.load_mut()?;
    depository.regular_minting_disabled = disable;
    Ok(())
}

// Validate input arguments
impl<'info> DisableDepositoryRegularMinting<'info> {
    pub(crate) fn validate(&self, disable: bool) -> Result<()> {
        require!(
            self.depository.load()?.regular_minting_disabled != disable,
            UxdError::MintingAlreadyDisabledOrEnabled
        );
        Ok(())
    }
}
