use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod mango_program;
pub mod state;

pub use crate::error::ControllerError;
pub use crate::instructions::*;
pub use crate::state::*;

pub const UXD_DECIMAL: u8 = 6;
pub const STATE_SEED: &[u8] = b"STATE";
pub const UXD_SEED: &[u8] = b"STABLECOIN";
pub const DEPOSITORY_SEED: &[u8] = b"DEPOSITORY";
pub const PASSTHROUGH_SEED: &[u8] = b"PASSTHROUGH";
pub const MANGO_SEED: &[u8] = b"MANGO";

solana_program::declare_id!("5BkgzsnpEzcbftbtQZ86zb3qi4S9ZfcYhpwuWKTp9nHB");

#[program]
#[deny(unused_must_use)]
pub mod controller {

    use super::*;

    // INITIALIZE
    // create controller state, create uxd (this could happen elsewhere later)
    // the key we pass in as authority *must* be retained/protected to add depositories
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("initialize starts");
        instructions::initialize::handler(ctx)
    }

    // REGISTER DEPOSITORY
    // authority must sign and match authority in our initial state
    // create a mango account for the coin, create an entry indicating we created and trust this depository
    // create a passthrough account for whatever coin corresponds to this depository
    // we need this because the owner of the mango account and the token account must be the same
    // so we cant move funds directly from the user to mango
    pub fn register_depository(ctx: Context<RegisterDepository>) -> ProgramResult {
        msg!("register_depository starts");
        instructions::register_depository::handler(ctx)
    }

    // MINT UXD
    // transfer user coin to our passthrough. open a mango position with that
    // then mint uxd in the amount of the mango position to the user
    #[access_control(valid_slippage(slippage))]
    pub fn mint_uxd(ctx: Context<MintUxd>, coin_amount: u64, slippage: u32) -> ProgramResult {
        msg!("mint_uxd starts");
        instructions::mint_uxd::handler(ctx, coin_amount, slippage)
    }

    // REDEEM UXD
    // burn uxd that is being redeemed. then close out mango position and return coins to user
    #[access_control(valid_slippage(slippage))]
    pub fn redeem_uxd(ctx: Context<RedeemUxd>, uxd_amount: u64, slippage: u32) -> ProgramResult {
        msg!("redeem_uxd starts");
        instructions::redeem_uxd::handler(ctx, uxd_amount, slippage)
    }

    // pub fn rebalance(ctx: Context<Rebalance>) -> ProgramResult {
    //     // validate caller is in rebalance signer(s)
    //     // WARNING DIFFICULT LOGIC
    //     // rebalance needs borrow/lending rate, outstanding pnl balance in an array across collateral types
    //     // probably better for it to just call by depository/collateral type for now,
    //     // since we're going for the single collateral version first
    //     // estimates rebalance cost eg transaction fees
    //     // uses some settable estimation constant (e?) for what the timescale to consider
    //     // if borrow * e * net pnl > est rebalance cost then rebal should go ahead
    //     // rebal for single collateral just amounts to settling some or all of the pnl and rehedging
    //     // for multi collateral there are two versions,
    //     // 1. that single collat balances in parallel for n depositories
    //         // could be a public function
    //     // 2. that optimizes for market rates across range of collateral types
    //         // will change portfolio balances in order to get the highest return on the basis trade
    //         // weighted array of parameters like liquidity, mkt cap, stability
    //         // Not a priority
    //
    // }
    //
}

// MARK: - ACCESS CONTROL  ----------------------------------------------------

const SLIPPAGE_BASIS: u32 = 1000;

// Asserts that the amount of usdc for the operation is above 0.
// Asserts that the amount of usdc is available in the user account.
fn valid_slippage<'info>(slippage: u32) -> ProgramResult {
    if !(slippage <= SLIPPAGE_BASIS) {
        return Err(ControllerError::InvalidSlippage.into());
    }
    Ok(())
}
