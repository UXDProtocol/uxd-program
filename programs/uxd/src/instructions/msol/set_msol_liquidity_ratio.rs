use crate::error::UxdError;
use crate::state::msol_config::MSolConfig;
use crate::state::msol_config::TARGET_LIQUIDITY_RATIO_MAX;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MSOL_CONFIG_NAMESPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetMsolLiquidityRatio<'info> {
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
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    /// #4 UXDProgram on chain account bound to a Controller instance
    /// The `MangoDepository` manages a MangoAccount for a single Collateral
    #[account(
        mut,
        seeds = [MANGO_DEPOSITORY_NAMESPACE, depository.collateral_mint.as_ref()],
        bump = depository.bump,
        has_one = controller @UxdError::InvalidController,
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdError::InvalidDepository
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 Msol config account for the `depository` instance
    #[account(
        mut,
        seeds = [MSOL_CONFIG_NAMESPACE, depository.key().as_ref()],
        bump = msol_config.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = depository @UxdError::InvalidDepository,
    )]
    pub msol_config: Box<Account<'info, MSolConfig>>,
}

pub fn handler(ctx: Context<SetMsolLiquidityRatio>, target_liquidity_ratio: u16) -> Result<()> {
    ctx.accounts.msol_config.target_liquidity_ratio = target_liquidity_ratio;
    Ok(())
}

impl<'info> SetMsolLiquidityRatio<'info> {
    pub fn validate(&mut self, target_liquidity_ratio: u16) -> Result<()> {
        if target_liquidity_ratio > TARGET_LIQUIDITY_RATIO_MAX {
            return Err(error!(UxdError::TargetLiquidityRatioExceedMax));
        }
        Ok(())
    }
}
