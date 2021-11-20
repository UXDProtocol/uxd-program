use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod mango_program;
pub mod state;
pub mod utils;

pub use crate::error::UXDError;
pub use crate::instructions::*;
pub use crate::state::*;

pub const SOLANA_MAX_MINT_DECIMALS: u8 = 9;

// These are just namespaces for the PDA creations. When possible we use
// anchor discriminator of the underlaying account, else these.
pub const REDEEMABLE_MINT_NAMESPACE: &[u8] = b"RedeemableMint";
pub const COLLATERAL_PASSTHROUGH_NAMESPACE: &[u8] = b"CollateralPassthrough";
pub const MANGO_ACCOUNT_NAMESPACE: &[u8] = b"MangoAccount";

solana_program::declare_id!("HTP8ZHRKEANYbJ9nZqoGNDihPdrAdyEiKg8yyzvPae7j");

#[program]
#[deny(unused_must_use)]
pub mod uxd {

    use super::*;

    // Initialize a Controller instance.
    // The Controller holds the Redeemable Mint and the authority identity.
    // In the case of UXD, the redeemable_mint is the UXD's mint.
    pub fn initialize_controller(
        ctx: Context<InitializeController>,
        controller_bump: u8,
        redeemable_mint_bump: u8,
        redeemable_mint_decimals: u8,
    ) -> ProgramResult {
        instructions::initialize_controller::handler(
            ctx,
            redeemable_mint_decimals,
            controller_bump,
            redeemable_mint_bump,
        )
    }

    // Transafer Controller authority to another entity.
    // pub fn transfer_controller_authority() -> ProgramResult {}

    // Transfer the UXD mint authority that is held by the controller to the provided account
    //
    // This might be an important safety, the program will be fully controlled and initialize
    // through the Governance program, seems accepable that way.
    // pub fn transfer_uxd_mint_authority() -> ProgramResult {}

    // Set the UXD supply Hard cap.
    //
    // As a conservative measure, UXD will have a virtual limit to it's supply, enforced through the program.
    // The goal is to roll out UXD progressively for safer and smoother begginings.
    // This would be lifted fully at some point.
    // pub fn set_uxd_hard_cap() -> ProgramResult {}

    // Set the UXD mint/redeem Soft cap.
    //
    // As a conservative measure, UXD will limit the amount of UXD mintable/redeemable to make sure things go smooth
    // or in case of issues we would cause on Mango.
    // This would be lifted fully at some point.
    // pub fn set_uxd_soft_cap() -> ProgramResult {}

    // Create and Register a new `Depository` to a `Controller`.
    // Each `Depository` account manages a specific collateral mint.
    // A `Depository` account owns a `collateral_passthrough` PDA as the owner of the mango account and
    //   the token account must be the same so we can't move fund directly from the use to Mango.
    // A `Depository` account own a `mango_account` PDA to deposit, withdraw, and open orders on Mango Market.
    pub fn register_depository_mango(
        ctx: Context<RegisterMangoDepository>,
        depository_bump: u8,
        collateral_passthrough_bump: u8,
    ) -> ProgramResult {
        instructions::register_mango_depository::handler(
            ctx,
            depository_bump,
            collateral_passthrough_bump,
        )
    }

    /// Mint UXD through a Depository using MangoMarkets.
    ///
    /// Deposits collateral to mango.
    /// open equivalent short perp (within slippage else fails. FoK behavior)
    /// mints uxd in the amount of the mango position to the user
    #[access_control(
        valid_slippage(slippage)
        check_amount_constraints(&ctx, collateral_amount)
    )]
    pub fn mint_on_mango_depository(
        ctx: Context<MintWithMangoDepository>,
        collateral_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        instructions::mint_with_mango_depository::handler(ctx, collateral_amount, slippage)
    }

    /// Exchange UXD for the underlaying collateral from a depository using MangoMarkets.
    ///
    /// Burn the amount of UXD
    /// close equivalent value of mango perp short position (withing slippage else fails. FoK behavior)
    /// withdraw equivalent value of collateral from mango
    /// return the collateral amount quivalent to the burnt UXD value to the user
    #[access_control(valid_slippage(slippage))]
    pub fn redeem_from_mango_depository(
        ctx: Context<RedeemFromMangoDepository>,
        uxd_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        instructions::redeem_from_mango_depository::handler(ctx, uxd_amount, slippage)
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
    ctx: &Context<MintWithMangoDepository<'info>>,
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
