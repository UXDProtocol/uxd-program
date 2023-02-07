use crate::error::UxdError;
use crate::events::InitializeControllerEvent;
use crate::Controller;
use crate::CONTROLLER_ACCOUNT_VERSION;
use crate::CONTROLLER_NAMESPACE;
use crate::CONTROLLER_SPACE;
use crate::DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SOLANA_MAX_MINT_DECIMALS;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

/// Takes 7 accounts - 4 used locally - 0 for CPI - 2 Programs - 1 Sysvar
#[derive(Accounts)]
#[instruction(
    redeemable_mint_decimals: u8,
)]
pub struct InitializeController<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,
    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,
    /*
    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        init,
        seeds = [CONTROLLER_NAMESPACE],
        bump,
        payer = payer,
        space = CONTROLLER_SPACE
    )]
    pub controller: AccountLoader<'info, Controller>,
    /// #4 The redeemable mint managed by the `controller` instance
    #[account(
        init,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump,
        mint::authority = controller,
        mint::decimals = redeemable_mint_decimals,
        payer = payer,
        constraint = redeemable_mint_decimals <= SOLANA_MAX_MINT_DECIMALS
    )]
    pub redeemable_mint: Account<'info, Mint>,
    */
    /// #5 System Program
    pub system_program: Program<'info, System>,
    /// #6 Token Program
    pub token_program: Program<'info, Token>,
    /// #7 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(
    ctx: Context<InitializeController>,
    redeemable_mint_decimals: u8,
) -> Result<()> {
    /*
    let controller = &mut ctx.accounts.controller.load_init()?;
    let redeemable_mint_unit = 10_u64
        .checked_pow(redeemable_mint_decimals.into())
        .ok_or_else(|| error!(UxdError::MathError))?;

    controller.bump = *ctx
        .bumps
        .get("controller")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    controller.redeemable_mint_bump = *ctx
        .bumps
        .get("redeemable_mint")
        .ok_or_else(|| error!(UxdError::BumpError))?;
    controller.version = CONTROLLER_ACCOUNT_VERSION;
    controller.authority = ctx.accounts.authority.key();
    controller.redeemable_mint = ctx.accounts.redeemable_mint.key();
    controller.redeemable_mint_decimals = redeemable_mint_decimals;
    // Default to 1 Million redeemable total cap
    controller.redeemable_global_supply_cap = DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP
        .checked_mul(redeemable_mint_unit.into())
        .ok_or_else(|| error!(UxdError::MathError))?;
    controller.redeemable_circulating_supply = u128::MIN;
    controller.is_frozen = false;
    controller.profits_total_collected = u128::MIN;

    emit!(InitializeControllerEvent {
        version: controller.version,
        controller: ctx.accounts.controller.key(),
        authority: ctx.accounts.authority.key(),
    });
     */
    Ok(())
}

// Validate input arguments
impl<'info> InitializeController<'info> {
    // Asserts that the redeemable mint decimals is between 0 and 9.
    pub(crate) fn validate(&self, decimals: u8) -> Result<()> {
        require!(
            decimals <= SOLANA_MAX_MINT_DECIMALS,
            UxdError::InvalidRedeemableMintDecimals
        );
        Ok(())
    }
}
