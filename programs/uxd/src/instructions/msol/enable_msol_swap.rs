use crate::error::UxdError;
use crate::state::msol_config::MSolConfig;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MSOL_CONFIG_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EnableMsolSwap<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        has_one = authority @UxdError::InvalidAuthority,
        constraint = controller.load()?.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 Msol config account for the `depository` instance
    #[account(
        mut,
        seeds = [MSOL_CONFIG_NAMESPACE, depository.key().as_ref()],
        bump = msol_config.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = depository @UxdError::InvalidDepository,
    )]
    pub msol_config: AccountLoader<'info, MSolConfig>,
}

pub fn handler(ctx: Context<EnableMsolSwap>, enable: bool) -> Result<()> {
    let msol_config = &mut ctx.accounts.msol_config.load_mut()?;
    msol_config.enabled = enable;
    Ok(())
}

impl<'info> EnableMsolSwap<'info> {
    pub fn validate(&mut self, enable: bool) -> Result<()> {
        require!(
            self.msol_config.load()?.enabled != enable,
            UxdError::InvalidEnablingMsolSwap
        );
        Ok(())
    }
}
