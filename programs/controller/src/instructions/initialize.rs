use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

use crate::State;
use crate::STATE_SEED;
use crate::UXD_DECIMAL;
use crate::UXD_SEED;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [STATE_SEED],
        bump,
        payer = authority,
    )]
    pub state: Box<Account<'info, State>>,
    #[account(
        init,
        seeds = [UXD_SEED],
        bump,
        mint::authority = state,
        mint::decimals = UXD_DECIMAL,
        payer = authority,
    )]
    pub uxd_mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Initialize>) -> ProgramResult {
    // - Update state
    let state_nonce = Pubkey::find_program_address(&[STATE_SEED], ctx.program_id).1;
    ctx.accounts.state.bump = state_nonce;
    ctx.accounts.state.authority_key = ctx.accounts.authority.key();
    ctx.accounts.state.uxd_mint_key = ctx.accounts.uxd_mint.key();

    Ok(())
}
