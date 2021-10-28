use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Depository {
    pub bump: u8,
    pub collateral_mint_key: Pubkey,
    pub collateral_passthrough_key: Pubkey,
    pub mango_account_key: Pubkey,
}
