use anchor_lang::prelude::*;

use crate::Controller;
use crate::ErrorCode;
use crate::CONTROLLER_NAMESPACE;

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
        mut,
        seeds = [CONTROLLER_NAMESPACE], 
        bump = controller.bump,
        has_one = authority,
    )]
    pub controller: Account<'info, Controller>,
}

pub fn handler(ctx: Context<RebalanceMangoDepository>) -> ProgramResult {
    
    Ok(())
}