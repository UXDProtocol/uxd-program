use crate::error::UxdError;
use crate::events::CreateDepositoryMSolConfigEvent;
use crate::state::msol_config::MSolConfig;
use crate::state::msol_config::TARGET_LIQUIDITY_RATIO_MAX;
use crate::Controller;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;
use crate::MSOL_CONFIG_NAMESPACE;
use crate::MSOL_CONFIG_SPACE;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateDepositoryMSolConfig<'info> {
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
        constraint = depository.load()?.collateral_mint == spl_token::native_mint::id() @UxdError::InvalidNonNativeMintUsed
    )]
    pub depository: AccountLoader<'info, MangoDepository>,

    /// #5 Msol config account for the `depository` instance
    #[account(
        init,
        seeds = [MSOL_CONFIG_NAMESPACE, depository.key().as_ref(), &[2u8]],
        bump,
        payer = payer,
        space = MSOL_CONFIG_SPACE
    )]
    pub msol_config: AccountLoader<'info, MSolConfig>,

    /// #6 System Program
    pub system_program: Program<'info, System>,

    /// #7 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateDepositoryMSolConfig>,
    target_liquidity_ratio: u16,
) -> Result<()> {
    let msol_config = &mut ctx.accounts.msol_config.load_init()?;

    msol_config.bump = *ctx
        .bumps
        .get("msol_config")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    msol_config.depository = ctx.accounts.depository.key();
    msol_config.controller = ctx.accounts.controller.key();
    msol_config.enabled = false;
    msol_config.target_liquidity_ratio = target_liquidity_ratio;

    emit!(CreateDepositoryMSolConfigEvent {
        version: ctx.accounts.controller.load()?.version,
        controller: msol_config.controller,
        depository: msol_config.depository,
        msol_config: ctx.accounts.msol_config.key(),
        enabled: msol_config.enabled,
        target_liquidity_ratio: msol_config.target_liquidity_ratio,
    });

    Ok(())
}

impl<'info> CreateDepositoryMSolConfig<'info> {
    pub fn validate(&mut self, target_liquidity_ratio: u16) -> Result<()> {
        require!(
            target_liquidity_ratio <= TARGET_LIQUIDITY_RATIO_MAX,
            UxdError::TargetLiquidityRatioExceedMax
        );
        Ok(())
    }
}
