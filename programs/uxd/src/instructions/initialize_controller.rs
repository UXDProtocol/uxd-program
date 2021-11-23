use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use crate::REDEEMABLE_MINT_NAMESPACE;
use crate::SOLANA_MAX_MINT_DECIMALS;

// Here we should set a deployer authority for the first person who init the UXD program stack, like mango IDO?
// Not sure it matter but then we should double check what happen when several version are instantiated with the way seed are defined
// pub const DEPLOYER_AUTHORITY = "";

#[derive(Accounts)]
#[instruction(
    bump: u8,
    redeemable_mint_bump: u8,
    redeemable_mint_decimals: u8,
)]
pub struct InitializeController<'info> {
    // This account is important, only this identity will be able to do admin calls in the future. Choose wisely
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [CONTROLLER_NAMESPACE],
        bump = bump,
        payer = authority,
    )]
    pub controller: Account<'info, Controller>,
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
    bump: u8,
    redeemable_mint_bump: u8,
    redeemable_mint_decimals: u8,
) -> ProgramResult {
    ctx.accounts.controller.bump = bump;
    ctx.accounts.controller.redeemable_mint_bump = redeemable_mint_bump;
    ctx.accounts.controller.authority = ctx.accounts.authority.key();
    ctx.accounts.controller.redeemable_mint = ctx.accounts.redeemable_mint.key();
    ctx.accounts.controller.redeemable_mint_decimals = redeemable_mint_decimals;
    // Default to 100T - converted to redeemable native unit
    ctx.accounts.controller.redeemable_global_supply_cap = DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP;
    ctx.accounts.controller.redeemable_circulating_supply = u128::MIN;

    Ok(())
}

// Keep here to remmeber how to test later, useless now here
// fn to_native_amount(ui_amount: u64, mint_decimals: u8) -> u64 {
//     let redeemable_mint_unit = 10_u64.pow(u32::from(mint_decimals));
//     ui_amount
//         .checked_mul(redeemable_mint_unit)
//         .unwrap()
//         .checked_to_num()
//         .unwrap()
// }

// #[cfg(test)]
// mod test {
//     use crate::MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP;

//     use super::*;
//     use proptest::prelude::*;

//     proptest! {

//         #[test]
//         fn proptest_to_native_amount(ui_amount in u64::MIN..MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP, decimals in 0u8..SOLANA_MAX_MINT_DECIMALS) {
//             // GIVEN

//             // WHEN
//             let expected_native_amount: u64 = ui_amount.checked_mul(10u64.pow(u32::from(decimals))).unwrap();
//             let actual_native_amount = to_native_amount(ui_amount, decimals);

//             // THEN
//             assert_eq!(
//                 expected_native_amount, actual_native_amount,
//                 "mismatch for ui_amount {} and decimals {}", ui_amount, decimals
//             );
//         }
//     }
// }
