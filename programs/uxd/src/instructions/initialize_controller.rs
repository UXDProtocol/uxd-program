use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

use crate::Controller;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SOLANA_MAX_MINT_DECIMALS;

// Here we should set a deployer authority for the first person who init the UXD program stack, like mango IDO?
// Not sure it matter but then we should double check what happen when several version are instantiated with the way seed are defined
// pub const DEPLOYER_AUTHORITY = "";

#[derive(Accounts)]
#[instruction(
    redeemable_mint_decimals: u8,
    controller_bump: u8,
    redeemable_mint_bump: u8
)]
pub struct InitializeController<'info> {
    // This account is important, only this identity will be able to do admin calls in the future. Choose wisely
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [&Controller::discriminator()[..]],
        bump = controller_bump,
        payer = authority,
    )]
    pub controller: Box<Account<'info, Controller>>,
    #[account(
        init,
        seeds = [REDEEMABLE_MINT_NAMESPACE],
        bump = redeemable_mint_bump,
        mint::authority = controller,
        mint::decimals = redeemable_mint_decimals,
        payer = authority,
        constraint = redeemable_mint_decimals <= SOLANA_MAX_MINT_DECIMALS
    )]
    pub redeemable_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializeController>,
    redeemable_mint_decimals: u8,
    controller_bump: u8,
    redeemable_mint_bump: u8,
) -> ProgramResult {
    ctx.accounts.controller.bump = controller_bump;
    ctx.accounts.controller.authority = ctx.accounts.authority.key();
    ctx.accounts.controller.redeemable_mint = ctx.accounts.redeemable_mint.key();
    ctx.accounts.controller.redeemable_mint_bump = redeemable_mint_bump;
    ctx.accounts.controller.redeemable_mint_decimals = redeemable_mint_decimals;

    Ok(())
}
