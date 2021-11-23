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
pub const DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = 1_000_000; // 1 Million redeemables UI units

solana_program::declare_id!("CPDGtzxfhmbTTM6DXHMPHyz3gHNwhsXPAGaCECvk5dqg");

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

    // Rebalance the health of one depository.
    // Short Perp PNL will change over time. When it does, other users can settle match us (forcing the update of our balance, as this unsettle PnL is virtual, i.e. we don't pay interests on it)
    pub fn rebalance_mango_depository(ctx: Context<RebalanceMangoDepository>) -> ProgramResult {
        msg!("UXD rebalance_mango_depository");
        instructions::rebalance_mango_depository::handler(ctx)
    }

    /// Mint UXD through a Depository using MangoMarkets.
    ///
    /// Through Depository configured for a specific collateral and using Mango Market v3
    /// Deposits user's collateral to mango
    /// open equivalent short perp (within slippage else fails. FoK behavior)
    /// mints uxd in the amount of the mango position to the user
    #[access_control(
        valid_slippage(slippage)
        check_amount_constraints(&ctx, collateral_amount)
    )]
    pub fn mint_with_mango_depository(
        ctx: Context<MintWithMangoDepository>,
        collateral_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        msg!("UXD mint_with_mango_depository");
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
        msg!("UXD redeem_from_mango_depository");
        instructions::redeem_from_mango_depository::handler(ctx, uxd_amount, slippage)
    }
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

// Asserts that the amount of usdc for the operation is above 0.
// Asserts that the amount of usdc is available in the user account.
fn valid_slippage<'info>(slippage: u32) -> ProgramResult {
    if !(slippage <= SLIPPAGE_BASIS) {
        return Err(ErrorCode::InvalidSlippage.into());
    }
    Ok(())
}

pub fn check_amount_constraints<'info>(
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
