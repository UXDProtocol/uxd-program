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

// Since the leverage is not 1:1 (depend of collateral iirc), price change in the underlying
// asset will create imbalance between the long collateral and the short perp
// - First, short perp Base Pos should be equal to Spot net balance (we only care about one collateral per depository)
//   any excess is the result of fees, odd lot size etc (Should there be any remains also, or is that an issue from the other logic?)
// - then, when the price of the base asset go higher, the leverage will increase as the perp 
//
// From MangoMarket.Daffy :
//
// Is there any liquidation risk for buying spot and shorting an equal amount of perpetuals?
//
// Yes there is still liquidation risk with the basis trade.
// The reason is because as the oracle price goes up, you accrue USDC losses on the perp which have a weight of 1,
// and the value of your token increases the same amount, but that has a weight less than 1.
//
// So the liquidation price on the basis trade is this: 
//
// liq_price = position_price / (maint_liab_weight_perp - maint_asset_weight_token)
//
// For example, for BTC-PERP basis trade of +1 BTC token and -1 BTC-PERP at position price of 60k, 
// your liquidation price would be 60,000 / (1.025 - 0.9)  = 480,000
//
pub fn handler(ctx: Context<RebalanceMangoDepository>) -> ProgramResult {

    // getConfirmedAdressTo - give you all the transactions that touch a certain accoun
    //      check the event to know what actually happened and do the accounting

    // NOTE : what about when we are settle by other parties as the Mango engine works this way, how to keep track of this?
    //          Does that invalidate the pseudo code below?

    // Check unsettled funding for the collateral short perp
    //
    //  if positive
    //      - Settle positive funding ? (maybe another IX, also might not be fully needed here)
    //      - END (We earn interests on it, let it in the balance)
    //  NOTE : seems hard to do that, cause the pending positive funding will be mixed up with the insurance fund in the 
    //          account's USDC balance. Might just also rebalance
    //
    //  if negative
    //      - convert   value of unsettled funding in QUOTE to BASE     `unsettled_funding_in_base_amount` (using SPOT price)
    //      - close     an equivalent amount of                         `unsettled_funding_in_base_amount` of the short perp
    //      - sell spot an equivalent amount of                         `unsettled_funding_in_base_amount` of BASE pos
    //      - settle    
    //  NOTE: the overall position size should stay the same.
    //  NOTE: wanted to wait to settle later, but that would make things very hard to track between the
    //         unsettled funding waiting to be settled and the Insurance fund that will be laying in the USDC balance
    //
    //  NOTE: how to get the "Unrealized PnL" displayed on the platform?
    //         that's what I assumed as unsettled_funding in the above text, probably wrongly

    // - 1 [CHECK UNSETTLED FUNDING] ------------------------------------------

    // - [Get perp informations]
    let perp_info = ctx.accounts.perpetual_info();

    Ok(())
}

// DO DOUBLE ACCOUNTING with the values in the depository state

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

// Additional convenience methods related to the inputed accounts
impl<'info> RebalanceMangoDepository<'info> {
    // Return general information about the perpetual related to the collateral in use
    fn perpetual_info(&self) -> PerpInfo {
        let mango_group =
            MangoGroup::load_checked(&self.mango_group, self.mango_program.key).unwrap();
        let mango_cache =
            MangoCache::load_checked(&self.mango_cache, self.mango_program.key, &mango_group)
                .unwrap();
        let perp_market_index = mango_group
            .find_perp_market_index(self.mango_perp_market.key)
            .unwrap();
        let perp_info = PerpInfo::init(&mango_group, &mango_cache, perp_market_index);
        msg!("Perpetual informations: {:?}", perp_info);
        return perp_info;
    }
}