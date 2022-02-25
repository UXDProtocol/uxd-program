use crate::error::UxdError;
use crate::events::InitializeControllerEvent;
use crate::Controller;
use crate::CONTROLLER_ACCOUNT_VERSION;
use crate::CONTROLLER_NAMESPACE;
use crate::DEFAULT_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP;
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

    /// #3 The top level UXDProgram on chain account managing the redeemable mint
    #[account(
        init,
        seeds = [CONTROLLER_NAMESPACE],
        bump,
        payer = payer,
    )]
    pub controller: Box<Account<'info, Controller>>,

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

    /// #5 System Program
    pub system_program: Program<'info, System>,

    /// #6 Token Program
    pub token_program: Program<'info, Token>,

    /// #7 Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeController>, redeemable_mint_decimals: u8) -> Result<()> {
    let redeemable_mint_unit = 10_u64
        .checked_pow(redeemable_mint_decimals.into())
        .ok_or(error!(UxdError::MathError))?;

    ctx.accounts.controller.bump = *ctx
        .bumps
        .get("controller")
        .ok_or(error!(UxdError::BumpError))?;
    ctx.accounts.controller.redeemable_mint_bump = *ctx
        .bumps
        .get("redeemable_mint")
        .ok_or(error!(UxdError::BumpError))?;
    ctx.accounts.controller.version = CONTROLLER_ACCOUNT_VERSION;
    ctx.accounts.controller.authority = ctx.accounts.authority.key();
    ctx.accounts.controller.redeemable_mint = ctx.accounts.redeemable_mint.key();
    ctx.accounts.controller.redeemable_mint_decimals = redeemable_mint_decimals;
    // Default to 1 Million redeemable total cap
    ctx.accounts.controller.redeemable_global_supply_cap = DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP
        .checked_mul(redeemable_mint_unit.into())
        .ok_or(error!(UxdError::MathError))?;
    // Default to 10 Thousand redeemable per mint/redeem
    ctx.accounts
        .controller
        .mango_depositories_redeemable_soft_cap = DEFAULT_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP
        .checked_mul(redeemable_mint_unit)
        .ok_or(error!(UxdError::MathError))?;
    ctx.accounts.controller.redeemable_circulating_supply = u128::MIN;

    emit!(InitializeControllerEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        authority: ctx.accounts.authority.key(),
    });
    Ok(())
}

// Validate input arguments
impl<'info> InitializeController<'info> {
    // Asserts that the redeemable mint decimals is between 0 and 9.
    pub fn validate(&self, decimals: u8) -> Result<()> {
        if decimals <= SOLANA_MAX_MINT_DECIMALS {
            return Err(error!(UxdError::InvalidRedeemableMintDecimals));
        }
        Ok(())
    }
}
