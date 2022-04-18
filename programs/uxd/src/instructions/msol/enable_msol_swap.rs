use crate::declare_check_assert_macros;
use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::state::msol_config::MSolConfig;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MSOL_CONFIG_NAMESPACE;
use anchor_lang::prelude::*;

declare_check_assert_macros!(SourceFileId::InstructionEnableMsolSwap);

#[derive(Accounts)]
pub struct EnableMsolSwap<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
        has_one = authority @UxdIdlErrorCode::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 Msol config account for the `depository` instance
    #[account(
        mut,
        seeds = [MSOL_CONFIG_NAMESPACE, depository.key().as_ref()],
        bump = msol_config.bump,
        has_one = controller @UxdIdlErrorCode::InvalidController,
        has_one = depository @UxdIdlErrorCode::InvalidDepository,
    )]
    pub msol_config: Box<Account<'info, MSolConfig>>,
}

pub fn handler(ctx: Context<EnableMsolSwap>, enable: bool) -> UxdResult {
    ctx.accounts.msol_config.enabled = enable;
    Ok(())
}

impl<'info> EnableMsolSwap<'info> {
    pub fn validate(&mut self, enable: bool) -> ProgramResult {
        if (self.msol_config.enabled != enable){
            return Err(error!(UxdError::InvalidEnablingMsolSwap));
        }
        Ok(())
    }
}
