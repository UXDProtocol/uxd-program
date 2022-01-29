use crate::declare_check_assert_macros;
use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::Controller;
use crate::UxdError;
use crate::UxdErrorCode;
use crate::UxdResult;
use crate::CONTROLLER_NAMESPACE;
use crate::DEFAULT_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP;
use crate::DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use crate::PROGRAM_VERSION;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SOLANA_MAX_MINT_DECIMALS;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use crate::events::InitializeControllerEvent;

declare_check_assert_macros!(SourceFileId::InstructionInitializeController);

#[derive(Accounts)]
#[instruction(
    bump: u8,
    redeemable_mint_bump: u8,
    redeemable_mint_decimals: u8,
)]
pub struct InitializeController<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds = [CONTROLLER_NAMESPACE],
        bump = bump,
        payer = payer,
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        init,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = redeemable_mint_bump,
        mint::authority = controller,
        mint::decimals = redeemable_mint_decimals,
        payer = payer,
        constraint = redeemable_mint_decimals <= SOLANA_MAX_MINT_DECIMALS
    )]
    pub redeemable_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializeController>,
    bump: u8,
    redeemable_mint_bump: u8,
    redeemable_mint_decimals: u8,
) -> UxdResult {
    let redeemable_mint_unit = 10_u64
        .checked_pow(redeemable_mint_decimals.into())
        .ok_or(math_err!())?;

    ctx.accounts.controller.bump = bump;
    ctx.accounts.controller.redeemable_mint_bump = redeemable_mint_bump;
    ctx.accounts.controller.version = PROGRAM_VERSION;
    ctx.accounts.controller.authority = ctx.accounts.authority.key();
    ctx.accounts.controller.redeemable_mint = ctx.accounts.redeemable_mint.key();
    ctx.accounts.controller.redeemable_mint_decimals = redeemable_mint_decimals;
    // Default to 1 Million redeemable total cap
    ctx.accounts.controller.redeemable_global_supply_cap = DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP
        .checked_mul(redeemable_mint_unit.into())
        .ok_or(math_err!())?;
    // Default to 10 Thousand redeemable per mint/redeem
    ctx.accounts
        .controller
        .mango_depositories_redeemable_soft_cap = DEFAULT_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP
        .checked_mul(redeemable_mint_unit)
        .ok_or(math_err!())?;
    ctx.accounts.controller.redeemable_circulating_supply = u128::MIN;

    emit!(InitializeControllerEvent {
        version: ctx.accounts.controller.version,
        controller: ctx.accounts.controller.key(),
        authority: ctx.accounts.authority.key(),
    });
    Ok(())
}

// Validate
impl<'info> InitializeController<'info> {
    // Asserts that the redeemable mint decimals is between 0 and 9.
    pub fn validate(
        &self,
        decimals: u8,
    ) -> ProgramResult {
        check!(
            decimals <= SOLANA_MAX_MINT_DECIMALS,
            UxdErrorCode::InvalidRedeemableMintDecimals
        )?;
        Ok(())
    }
}
