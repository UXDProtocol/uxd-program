use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

use crate::State;
use crate::UXD_DECIMAL;
use crate::UXD_MINT_NAMESPACE;

// Here we should set a deployer authority for the first person who init the UXD program stack, like mango IDO
// pub const DEPLOYER_AUTHORITY = "";

#[derive(Accounts)]
#[instruction(
    state_bump: u8,
    uxd_mint_bump: u8
)]
pub struct Initialize<'info> {
    // This account is important, only this identity will be able to do admin calls in the future. Choose wisely
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [&State::discriminator()[..]],
        bump = state_bump,
        payer = authority,
    )]
    pub state: Box<Account<'info, State>>,
    #[account(
        init,
        seeds = [UXD_MINT_NAMESPACE],
        bump = uxd_mint_bump,
        mint::authority = state,
        mint::decimals = UXD_DECIMAL,
        payer = authority,
    )]
    pub uxd_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>, state_bump: u8, uxd_mint_bump: u8) -> ProgramResult {
    ctx.accounts.state.bump = state_bump;
    ctx.accounts.state.authority = ctx.accounts.authority.key();
    ctx.accounts.state.uxd_mint = ctx.accounts.uxd_mint.key();
    ctx.accounts.state.uxd_mint_bump = uxd_mint_bump;

    Ok(())
}
