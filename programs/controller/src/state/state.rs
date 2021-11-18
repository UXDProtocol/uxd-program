use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct State {
    pub bump: u8,
    pub authority: Pubkey,
    pub uxd_mint: Pubkey,
    pub uxd_mint_bump: u8,
}
