use crate::error::check_assert;
use crate::error::UxdErrorCode;
use crate::instructions::*;
use crate::state::*;
use anchor_lang::prelude::*;
use error::UxdError;

#[macro_use]
pub mod error;

pub mod events;
pub mod instructions;
pub mod mango_program;
pub mod mango_utils;
pub mod state;
pub mod test;

// CI Uses AyEU8xdZGokmgRbeahLBhg4L_1LbyRXUFQ2qcNkSRyAeH on Devnet
// (it's auto swapped by the script, keypair are held in target/deployment)
#[cfg(feature = "development")]
solana_program::declare_id!("FZu6LF1bzwCTrLEGaNKVrbkhzsJwEe5jYBG4nY7JdoQS");
#[cfg(feature = "production")]
solana_program::declare_id!("UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr");

// Version used for accounts structure and future migrations
pub const MANGO_DEPOSITORY_ACCOUNT_VERSION: u8 = 2;
pub const CONTROLLER_ACCOUNT_VERSION: u8 = 1;

// These are just "namespaces" seeds for the PDA creations.
pub const REDEEMABLE_MINT_NAMESPACE: &[u8] = b"REDEEMABLE";
pub const COLLATERAL_PASSTHROUGH_NAMESPACE: &[u8] = b"COLLATERALPASSTHROUGH";
pub const INSURANCE_PASSTHROUGH_NAMESPACE: &[u8] = b"INSURANCEPASSTHROUGH";
pub const QUOTE_PASSTHROUGH_NAMESPACE: &[u8] = b"QUOTEPASSTHROUGH";
pub const MANGO_ACCOUNT_NAMESPACE: &[u8] = b"MANGOACCOUNT";
pub const CONTROLLER_NAMESPACE: &[u8] = b"CONTROLLER";
pub const MANGO_DEPOSITORY_NAMESPACE: &[u8] = b"MANGODEPOSITORY";

pub const MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = u128::MAX;
pub const DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = 1_000_000; // 1 Million redeemable UI units

pub const MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP: u64 = u64::MAX;
pub const DEFAULT_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP: u64 = 10_000; // 10 Thousand redeemable UI units

const SLIPPAGE_BASIS: u32 = 1000;
const SOLANA_MAX_MINT_DECIMALS: u8 = 9;

pub type UxdResult<T = ()> = Result<T, UxdError>;

declare_check_assert_macros!(SourceFileId::Lib);

#[program]
#[deny(unused_must_use)]
pub mod uxd {

    use super::*;

    // Initialize a Controller instance.
    #[access_control(ctx.accounts.validate(redeemable_mint_decimals))]
    pub fn initialize_controller(
        ctx: Context<InitializeController>,
        redeemable_mint_decimals: u8,
    ) -> ProgramResult {
        msg!("[initialize_controller]");
        instructions::initialize_controller::handler(ctx, redeemable_mint_decimals).map_err(|e| {
            msg!("<*> {}", e); // log the error
            e.into() // convert UxdError to generic ProgramError
        })
    }

    // Set the Redeemable global supply cap.
    //
    // Goal is to roll out progressively, and limit risks.
    // If this is set below the current circulating supply of UXD, it would effectively pause Minting.
    #[access_control(ctx.accounts.validate(redeemable_global_supply_cap))]
    pub fn set_redeemable_global_supply_cap(
        ctx: Context<SetRedeemableGlobalSupplyCap>,
        redeemable_global_supply_cap: u128,
    ) -> ProgramResult {
        msg!("[set_redeemable_global_supply_cap]");
        instructions::set_redeemable_global_supply_cap::handler(ctx, redeemable_global_supply_cap)
            .map_err(|e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            })
    }

    // Set Mango Depositories Redeemable soft cap (for Minting operation).
    //
    // Goal is to roll out progressively, and limit risks.
    // If this is set to 0, it would effectively pause Minting.
    // Note : This would effectively pause minting.
    #[access_control(ctx.accounts.validate(redeemable_soft_cap))]
    pub fn set_mango_depositories_redeemable_soft_cap(
        ctx: Context<SetMangoDepositoriesRedeemableSoftCap>,
        redeemable_soft_cap: u64,
    ) -> ProgramResult {
        msg!("[set_mango_depositories_redeemable_soft_cap]");
        instructions::set_mango_depositories_redeemable_soft_cap::handler(ctx, redeemable_soft_cap)
            .map_err(|e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            })
    }

    // Create and Register a new `MangoDepository` to the `Controller`.
    // Each `MangoDepository` account manages a specific collateral mint.
    // A `MangoDepository` account owns a `mango_account` PDA to deposit, withdraw, and open orders on Mango Market.
    // Several passthrough accounts are required in order to transaction with the `mango_account`.
    pub fn register_mango_depository(ctx: Context<RegisterMangoDepository>) -> ProgramResult {
        msg!("[register_mango_depository]");
        instructions::register_mango_depository::handler(ctx).map_err(|e| {
            msg!("<*> {}", e); // log the error
            e.into() // convert UxdError to generic ProgramError
        })
    }

    pub fn migrate_mango_depository_to_v2(
        ctx: Context<MigrateMangoDepositoryToV2>,
    ) -> ProgramResult {
        msg!("[migrate_mango_depository_to_v2]");
        instructions::migrate_mango_depository_to_v2::handler(ctx).map_err(|e| {
            msg!("<*> {}", e); // log the error
            e.into() // convert UxdError to generic ProgramError
        })
    }

    #[access_control(ctx.accounts.validate(insurance_amount))]
    pub fn deposit_insurance_to_mango_depository(
        ctx: Context<DepositInsuranceToMangoDepository>,
        insurance_amount: u64,
    ) -> ProgramResult {
        msg!("[deposit_insurance_to_mango_depository]");
        instructions::deposit_insurance_to_mango_depository::handler(ctx, insurance_amount).map_err(
            |e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            },
        )
    }

    // Withdraw insurance previously deposited, if any available, in the limit of mango health.
    #[access_control(ctx.accounts.validate(insurance_amount))]
    pub fn withdraw_insurance_from_mango_depository(
        ctx: Context<WithdrawInsuranceFromMangoDepository>,
        insurance_amount: u64,
    ) -> ProgramResult {
        msg!("[withdraw_insurance_from_mango_depository]");
        instructions::withdraw_insurance_from_mango_depository::handler(ctx, insurance_amount)
            .map_err(|e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            })
    }

    #[access_control(ctx.accounts.validate(max_rebalancing_amount, &polarity, slippage))]
    pub fn rebalance_mango_depository_lite(
        ctx: Context<RebalanceMangoDepositoryLite>,
        max_rebalancing_amount: u64,
        polarity: PnlPolarity,
        slippage: u32,
    ) -> ProgramResult {
        msg!("[rebalance_mango_depository_lite] slippage {}, polarity {}", slippage, polarity);
        instructions::rebalance_mango_depository_lite::handler(
            ctx,
            max_rebalancing_amount,
            &polarity,
            slippage,
        )
        .map_err(|e| {
            msg!("<*> {}", e); // log the error
            e.into() // convert UxdError to generic ProgramError
        })
    }

    // Mint Redeemable tokens by depositing Collateral to mango and opening the equivalent short perp position.
    // Callers pays taker_fees, that are deducted from the returned redeemable tokens (and part of the delta neutral position)
    #[access_control(
        ctx.accounts.validate(collateral_amount, slippage)
    )]
    pub fn mint_with_mango_depository(
        ctx: Context<MintWithMangoDepository>,
        collateral_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        msg!("[mint_with_mango_depository] slippage {}", slippage);
        instructions::mint_with_mango_depository::handler(ctx, collateral_amount, slippage).map_err(
            |e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            },
        )
    }

    // Burn Redeemable tokens and return the equivalent quote value of Collateral by unwinding a part of the delta neutral position.
    // Callers pays taker_fees.
    #[access_control(
        ctx.accounts.validate(redeemable_amount, slippage)
    )]
    pub fn redeem_from_mango_depository(
        ctx: Context<RedeemFromMangoDepository>,
        redeemable_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        msg!("[redeem_from_mango_depository] slippage {}", slippage);
        instructions::redeem_from_mango_depository::handler(ctx, redeemable_amount, slippage)
            .map_err(|e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            })
    }
}
