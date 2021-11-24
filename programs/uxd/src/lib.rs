use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod mango_program;
pub mod state;
pub mod utils;

pub use crate::error::ErrorCode;
pub use crate::instructions::*;
pub use crate::state::*;

// These are just "namespaces" seeds for the PDA creations.
pub const REDEEMABLE_MINT_NAMESPACE: &[u8] = b"REDEEMABLE";
pub const COLLATERAL_PASSTHROUGH_NAMESPACE: &[u8] = b"PASSTHROUGH";
pub const MANGO_ACCOUNT_NAMESPACE: &[u8] = b"MANGOACCOUNT";
pub const CONTROLLER_NAMESPACE: &[u8] = b"CONTROLLER";
pub const MANGO_DEPOSITORY_NAMESPACE: &[u8] = b"MANGODEPOSITORY";

pub const MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = u128::MAX;
pub const DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = 1_000_000; // 1 Million redeemable UI units

pub const MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP: u64 = u64::MAX;
pub const DEFAULT_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP: u64 = 10_000; // 10 Thousand redeemable UI units

pub const MAX_REGISTERED_MANGO_DEPOSITORIES: usize = 8;

solana_program::declare_id!("6xwXPEr7e7Vmc4NzDLaVaVNnDqH6ecZCX8yKmBH9K1hw");

#[program]
#[deny(unused_must_use)]
pub mod uxd {

    use super::*;

    // Initialize a Controller instance.
    // The Controller holds the Redeemable Mint and the authority identity.
    // In the case of UXD, the redeemable_mint is the UXD's mint.
    #[access_control(valid_redeemable_mint_decimals(redeemable_mint_decimals))]
    pub fn initialize_controller(
        ctx: Context<InitializeController>,
        bump: u8,
        redeemable_mint_bump: u8,
        redeemable_mint_decimals: u8,
    ) -> ProgramResult {
        msg!("UXD initialize_controller");
        instructions::initialize_controller::handler(
            ctx,
            bump,
            redeemable_mint_bump,
            redeemable_mint_decimals,
        )
    }

    // Transafer Controller authority to another entity.
    // pub fn transfer_controller_authority() -> ProgramResult {}

    // Transfer the UXD mint authority that is held by the controller to the provided account
    //
    // This might be an important safety, the program will be fully controlled and initialize
    // through the Governance program, seems accepable that way.
    // pub fn transfer_uxd_mint_authority() -> ProgramResult {}

    // Set the Redeemable global supply cap.
    //
    // Goal is to roll out progressively, and limit risks.
    // If this is set below the current circulating supply of UXD, it would effectively pause Minting.
    #[access_control(valid_redeemable_global_supply_cap(redeemable_global_supply_cap))]
    pub fn set_redeemable_global_supply_cap(
        ctx: Context<SetRedeemableGlobalSupplyCap>,
        redeemable_global_supply_cap: u128,
    ) -> ProgramResult {
        msg!(
            "UXD set_redeemable_global_supply_cap to {}",
            redeemable_global_supply_cap
        );
        instructions::set_redeemable_global_supply_cap::handler(ctx, redeemable_global_supply_cap)
    }

    // Set Mango Depositories Redeemable soft cap.
    //
    // Goal is to roll out progressively, and limit risks.
    // If this is set to 0, it would effectively pause Redeeming and Minting. (This seems unnacceptable, but will handled by DAO)
    #[access_control(valid_mango_depositories_redeemable_soft_cap(redeemable_soft_cap))]
    pub fn set_mango_depositories_redeemable_soft_cap(
        ctx: Context<SetMangoDepositoriesRedeemableSoftCap>,
        redeemable_soft_cap: u64,
    ) -> ProgramResult {
        msg!(
            "UXD set_mango_depositories_redeemable_soft_cap to {}",
            redeemable_soft_cap
        );
        instructions::set_mango_depositories_redeemable_soft_cap::handler(ctx, redeemable_soft_cap)
    }

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
    pub fn register_mango_depository(
        ctx: Context<RegisterMangoDepository>,
        bump: u8,
        collateral_passthrough_bump: u8,
        mango_account_bump: u8,
    ) -> ProgramResult {
        msg!("UXD register_mango_depository");
        instructions::register_mango_depository::handler(
            ctx,
            bump,
            collateral_passthrough_bump,
            mango_account_bump,
        )
    }

    /// Mint UXD through a Depository using MangoMarkets.
    ///
    /// Through Depository configured for a specific collateral and using Mango Market v3
    /// Deposits user's collateral to mango
    /// open equivalent short perp (within slippage else fails. FoK behavior)
    /// mints uxd in the amount of the mango position to the user
    #[access_control(
        valid_slippage(slippage)
        check_collateral_amount_constraints(&ctx, collateral_amount)
    )]
    pub fn mint_with_mango_depository(
        ctx: Context<MintWithMangoDepository>,
        collateral_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        msg!(
            "UXD mint_with_mango_depository - collateral_amount : {}",
            collateral_amount
        );
        instructions::mint_with_mango_depository::handler(ctx, collateral_amount, slippage)
    }

    /// Exchange UXD for the underlaying collateral from a depository using MangoMarkets.
    ///
    /// Burn the amount of UXD
    /// close equivalent value of mango perp short position (withing slippage else fails. FoK behavior)
    /// withdraw equivalent value of collateral from mango
    /// return the collateral amount quivalent to the burnt UXD value to the user
    #[access_control(
        valid_slippage(slippage)
        check_redeemable_amount_constraints(&ctx, redeemable_amount)
    )]
    pub fn redeem_from_mango_depository(
        ctx: Context<RedeemFromMangoDepository>,
        redeemable_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        msg!(
            "UXD redeem_from_mango_depository - redeemable_amount : {}",
            redeemable_amount
        );
        instructions::redeem_from_mango_depository::handler(ctx, redeemable_amount, slippage)
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
const SOLANA_MAX_MINT_DECIMALS: u8 = 9;

// Asserts that the redeemable mint decimals is between 0 and 9.
fn valid_redeemable_mint_decimals<'info>(decimals: u8) -> ProgramResult {
    if !(decimals <= SOLANA_MAX_MINT_DECIMALS) {
        return Err(ErrorCode::InvalidRedeemableMintDecimals.into());
    }
    Ok(())
}

// Asserts that the redeemable global supply cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
fn valid_redeemable_global_supply_cap<'info>(redeemable_global_supply_cap: u128) -> ProgramResult {
    if !(redeemable_global_supply_cap <= MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP) {
        return Err(ErrorCode::InvalidRedeemableGlobalSupplyCap.into());
    }
    Ok(())
}

// Asserts that the Mango Depositories redeemable soft cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
fn valid_mango_depositories_redeemable_soft_cap<'info>(redeemable_soft_cap: u64) -> ProgramResult {
    if !(redeemable_soft_cap <= MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP) {
        return Err(ErrorCode::InvalidRedeemableGlobalSupplyCap.into());
    }
    Ok(())
}

// Asserts that the amount of usdc for the operation is above 0.
// Asserts that the amount of usdc is available in the user account.
fn valid_slippage<'info>(slippage: u32) -> ProgramResult {
    if !(slippage <= SLIPPAGE_BASIS) {
        return Err(ErrorCode::InvalidSlippage.into());
    }
    Ok(())
}

pub fn check_collateral_amount_constraints<'info>(
    ctx: &Context<MintWithMangoDepository<'info>>,
    collateral_amount: u64,
) -> ProgramResult {
    if !(collateral_amount > 0) {
        return Err(ErrorCode::InvalidCollateralAmount.into());
    }
    if !(ctx.accounts.user_collateral.amount >= collateral_amount) {
        return Err(ErrorCode::InsuficientCollateralAmount.into());
    }
    Ok(())
}

pub fn check_redeemable_amount_constraints<'info>(
    ctx: &Context<RedeemFromMangoDepository<'info>>,
    redeemable_amount: u64,
) -> ProgramResult {
    if !(redeemable_amount > 0) {
        return Err(ErrorCode::InvalidRedeemableAmount.into());
    }
    if !(ctx.accounts.user_redeemable.amount >= redeemable_amount) {
        return Err(ErrorCode::InsuficientRedeemableAmount.into());
    }
    Ok(())
}
