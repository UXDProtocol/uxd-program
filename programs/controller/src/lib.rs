use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod mango_program;
pub mod state;

pub use crate::error::UXDError;
pub use crate::instructions::*;
pub use crate::state::*;

pub const UXD_DECIMAL: u8 = 6;

// These are just namespaces for the PDA creations. When possible we use
// anchor discriminator of the underlaying account, else these.
pub const UXD_MINT_NAMESPACE: &[u8] = b"Stablecoin";
pub const COLLATERAL_PASSTHROUGH_NAMESPACE: &[u8] = b"Passthrough";
pub const MANGO_ACCOUNT_NAMESPACE: &[u8] = b"MangoAccount";

solana_program::declare_id!("8LwByH5dsDPpW2TdqKWAbZ9X4bQQifnq3UT9wJjmcpWv");

#[program]
#[deny(unused_must_use)]
pub mod controller {

    use super::*;

    // INITIALIZE
    // configure the main state account of the program, and the UXD mint under it's authority
    // the authority is set (the signer of this call), any other admin operation will be
    // only accessible through the same signer later on. (Use a multisig/mango DAO for it)
    pub fn initialize(
        ctx: Context<Initialize>,
        state_bump: u8,
        uxd_mint_bump: u8,
    ) -> ProgramResult {
        msg!("initialize starts");
        instructions::initialize::handler(ctx, state_bump, uxd_mint_bump)
    }

    // REGISTER DEPOSITORY
    // authority must sign and match authority in our initial state
    // create a mango account for the coin, create an entry indicating we created and trust this depository
    // create a passthrough account for whatever coin corresponds to this depository
    // we need this because the owner of the mango account and the token account must be the same
    // so we cant move funds directly from the user to mango
    pub fn register_depository(
        ctx: Context<RegisterDepository>,
        depository_bump: u8,
        collateral_passthrough_bump: u8,
    ) -> ProgramResult {
        msg!("register_depository starts");
        instructions::register_depository::handler(
            ctx,
            depository_bump,
            collateral_passthrough_bump,
        )
    }

    /// MINT UXD
    /// deposit collateral to mango
    /// open equivalent short perp (within slippage else fails. FoK behavior)
    /// mints uxd in the amount of the mango position to the user
    #[access_control(
        valid_slippage(slippage)
        check_amount_constraints(&ctx, collateral_amount)
    )]
    pub fn mint_uxd(ctx: Context<MintUxd>, collateral_amount: u64, slippage: u32) -> ProgramResult {
        msg!("mint_uxd starts");
        instructions::mint_uxd::handler(ctx, collateral_amount, slippage)
    }

    /// REDEEM UXD
    /// burn the amount of UXD
    /// close equivalent value of mango perp short position (withing slippage else fails. FoK behavior)
    /// withdraw equivalent value of collateral from mango
    /// return the collateral amount quivalent to the burnt UXD value to the user
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
        return Err(UXDError::InvalidSlippage.into());
    }
    Ok(())
}

pub fn check_amount_constraints<'info>(
    ctx: &Context<MintUxd<'info>>,
    collateral_amount: u64,
) -> ProgramResult {
    if !(collateral_amount > 0) {
        return Err(UXDError::InvalidCollateralAmount.into());
    }
    if !(ctx.accounts.user_collateral.amount >= collateral_amount) {
        return Err(UXDError::InsuficientCollateralAmount.into());
    }
    Ok(())
}
