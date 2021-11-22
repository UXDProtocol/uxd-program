use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Controller {
    pub bump: u8,
    pub redeemable_mint_bump: u8,
    // The account that initialize this struct. Only this account can call permissionned instructions.
    pub authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub redeemable_mint_decimals: u8,
    // The total amount of UXD that can be in circulation, variable, to limit risks, do progressive rollout.
    //  in redeemable UI amount without decimals
    pub redeemable_global_supply_cap: u64,
}
