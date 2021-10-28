use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct State {
    pub bump: u8,
    pub authority_key: Pubkey,
    pub uxd_mint_key: Pubkey,
}
