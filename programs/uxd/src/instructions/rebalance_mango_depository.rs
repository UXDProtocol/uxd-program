use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::TokenAccount;
use fixed::types::I80F48;
use mango::error::MangoResult;
use mango::state::MangoAccount;
use mango::state::MangoCache;
use mango::state::MangoGroup;
use mango::state::PerpAccount;

use crate::mango_program;
use crate::utils::perp_base_position;
use crate::utils::PerpInfo;
use crate::Controller;
use crate::ErrorCode;
use crate::MangoDepository;
use crate::CONTROLLER_NAMESPACE;
use crate::MANGO_ACCOUNT_NAMESPACE;
use crate::MANGO_DEPOSITORY_NAMESPACE;

// validate caller is in rebalance signer(s)
// WARNING DIFFICULT LOGIC
// rebalance needs borrow/lending rate, outstanding pnl balance in an array across collateral types
// probably better for it to just call by depository/collateral type for now,
// since we're going for the single collateral version first
// estimates rebalance cost eg transaction fees
// uses some settable estimation constant (e?) for what the timescale to consider
// if borrow * e * net pnl > est rebalance cost then rebal should go ahead
// rebal for single collateral just amounts to settling some or all of the pnl and rehedging
// for multi collateral there are two versions,
// 1. that single collat balances in parallel for n depositories
    // could be a public function
// 2. that optimizes for market rates across range of collateral types
    // will change portfolio balances in order to get the highest return on the basis trade
    // weighted array of parameters like liquidity, mkt cap, stability
    // Not a priority

#[derive(Accounts)]
pub struct RebalanceMangoDepository<'info> {
    #[account(
        mut, 
        constraint = authority.key() == controller.authority @ErrorCode::InvalidAuthority
    )]
    pub authority: Signer<'info>,
    #[account(
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority,
    )]
    pub controller: Account<'info, Controller>,
    #[account(
        seeds = [MANGO_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.bump
    )]
    pub depository: Account<'info, MangoDepository>,
    #[account(
        constraint = collateral_mint.key() == depository.collateral_mint @ErrorCode::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [MANGO_ACCOUNT_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.mango_account_bump,
    )]
    pub depository_mango_account: AccountInfo<'info>,
    // Mango related accounts -------------------------------------------------
    // XXX All these account should be properly constrained if possible
    pub mango_group: AccountInfo<'info>,
    pub mango_cache: AccountInfo<'info>,
    pub mango_signer: AccountInfo<'info>,
    pub mango_root_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub mango_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mango_perp_market: AccountInfo<'info>,
    #[account(mut)]
    pub mango_bids: AccountInfo<'info>,
    #[account(mut)]
    pub mango_asks: AccountInfo<'info>,
    #[account(mut)]
    pub mango_event_queue: AccountInfo<'info>,
    // ------------------------------------------------------------------------
    // programs
    pub mango_program: Program<'info, mango_program::Mango>,
}

pub fn handler(ctx: Context<RebalanceMangoDepository>) -> ProgramResult {
    


    Ok(())
}

// impl<'info> RebalanceMangoDepository<'info> {
//     pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
//         let cpi_program = self.token_program.to_account_info();
//         let cpi_accounts = Burn {
//             mint: self.redeemable_mint.to_account_info(),
//             to: self.user_redeemable.to_account_info(),
//             authority: self.user.to_account_info(),
//         };
//         CpiContext::new(cpi_program, cpi_accounts)
//     }
// }