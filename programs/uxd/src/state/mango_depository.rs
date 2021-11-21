use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct MangoDepository {
    pub bump: u8,
    pub collateral_passthrough_bump: u8,
    pub mango_account_bump: u8,
    pub collateral_mint: Pubkey,
    pub collateral_passthrough: Pubkey,
    pub mango_account: Pubkey,
}
