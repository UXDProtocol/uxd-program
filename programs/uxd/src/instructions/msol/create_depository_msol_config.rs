use crate::declare_check_assert_macros;
use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdError;
use crate::error::UxdErrorCode;
use crate::error::UxdIdlErrorCode;
use crate::events::CreateDepositoryMSolConfigEvent;
use crate::state::msol_config::MSolConfig;
use crate::state::msol_config::TARGET_LIQUIDITY_RATIO_MAX;
use crate::Controller;
use crate::MangoDepository;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MSOL_CONFIG_NAMESPACE;
use anchor_lang::prelude::*;

declare_check_assert_macros!(SourceFileId::InstructionCreateDepositoryMSolConfig);

#[derive(Accounts)]
pub struct CreateDepositoryMSolConfig<'info> {
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
        constraint = controller.registered_mango_depositories.contains(&depository.key()) @UxdIdlErrorCode::InvalidDepository,
        constraint = depository.collateral_mint == spl_token::native_mint::id() @UxdIdlErrorCode::InvalidNonNativeMintUsed
    )]
    pub depository: Box<Account<'info, MangoDepository>>,

    /// #5 Msol config account for the `depository` instance
    #[account(
        init,
        seeds = [MSOL_CONFIG_NAMESPACE, depository.key().as_ref()],
        bump,
        payer = authority,
    )]
    pub msol_config: Box<Account<'info, MSolConfig>>,

    /// #6 System Program
    pub system_program: Program<'info, System>,

    /// #7 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateDepositoryMSolConfig>, target_liquidity_ratio: u16) -> UxdResult {
    ctx.accounts.msol_config.bump = *ctx.bumps.get("msol_config").ok_or(bump_err!())?;
    ctx.accounts.msol_config.depository = ctx.accounts.depository.key();
    ctx.accounts.msol_config.controller = ctx.accounts.controller.key();
    ctx.accounts.msol_config.enabled = false;
    ctx.accounts.msol_config.target_liquidity_ratio = target_liquidity_ratio;

    emit!(CreateDepositoryMSolConfigEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.msol_config.controller,
        depository: ctx.accounts.msol_config.depository,
        msol_config: ctx.accounts.msol_config.key(),
        enabled: ctx.accounts.msol_config.enabled,
        target_liquidity_ratio: ctx.accounts.msol_config.target_liquidity_ratio,
    });

    Ok(())
}

impl<'info> CreateDepositoryMSolConfig<'info> {
    pub fn validate(&mut self, target_liquidity_ratio: u16) -> ProgramResult {
        if (target_liquidity_ratio > TARGET_LIQUIDITY_RATIO_MAX) {
            return Err(error!(UxdError::TargetLiquidityRatioExceedMax));
        }
        Ok(())
    }
}
