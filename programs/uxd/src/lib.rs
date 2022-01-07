use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::error::UxdErrorCode;
use crate::instructions::*;
use crate::state::*;
use anchor_lang::prelude::*;
use error::UxdError;

#[macro_use]
pub mod error;

pub mod instructions;
pub mod mango_program;
pub mod mango_utils;
pub mod state;
pub mod events;

solana_program::declare_id!("UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr");

// Version used for accounts structure and future migrations
pub const PROGRAM_VERSION: u8 = 1;

// These are just "namespaces" seeds for the PDA creations.
pub const REDEEMABLE_MINT_NAMESPACE: &[u8] = b"REDEEMABLE";
pub const COLLATERAL_PASSTHROUGH_NAMESPACE: &[u8] = b"COLLATERALPASSTHROUGH";
pub const INSURANCE_PASSTHROUGH_NAMESPACE: &[u8] = b"INSURANCEPASSTHROUGH";
pub const MANGO_ACCOUNT_NAMESPACE: &[u8] = b"MANGOACCOUNT";
pub const CONTROLLER_NAMESPACE: &[u8] = b"CONTROLLER";
pub const MANGO_DEPOSITORY_NAMESPACE: &[u8] = b"MANGODEPOSITORY";

pub const MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = u128::MAX;
pub const DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = 1_000_000; // 1 Million redeemable UI units

pub const MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP: u64 = u64::MAX;
pub const DEFAULT_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP: u64 = 10_000; // 10 Thousand redeemable UI units

pub type UxdResult<T = ()> = Result<T, UxdError>;

declare_check_assert_macros!(SourceFileId::Lib);

#[program]
#[deny(unused_must_use)]
pub mod uxd {

    use super::*;

    // Initialize a Controller instance.
    #[access_control(valid_redeemable_mint_decimals(redeemable_mint_decimals))]
    pub fn initialize_controller(
        ctx: Context<InitializeController>,
        bump: u8,
        redeemable_mint_bump: u8,
        redeemable_mint_decimals: u8,
    ) -> ProgramResult {
        instructions::initialize_controller::handler(
            ctx,
            bump,
            redeemable_mint_bump,
            redeemable_mint_decimals,
        )
        .map_err(|e| {
            msg!("<*> {}", e); // log the error
            e.into() // convert UxdError to generic ProgramError
        })
    }

    // Set the Redeemable global supply cap.
    //
    // Goal is to roll out progressively, and limit risks.
    // If this is set below the current circulating supply of UXD, it would effectively pause Minting.
    #[access_control(valid_redeemable_global_supply_cap(redeemable_global_supply_cap))]
    pub fn set_redeemable_global_supply_cap(
        ctx: Context<SetRedeemableGlobalSupplyCap>,
        redeemable_global_supply_cap: u128,
    ) -> ProgramResult {
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
    #[access_control(valid_mango_depositories_redeemable_soft_cap(redeemable_soft_cap))]
    pub fn set_mango_depositories_redeemable_soft_cap(
        ctx: Context<SetMangoDepositoriesRedeemableSoftCap>,
        redeemable_soft_cap: u64,
    ) -> ProgramResult {
        instructions::set_mango_depositories_redeemable_soft_cap::handler(ctx, redeemable_soft_cap)
            .map_err(|e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            })
    }

    // Create and Register a new `MangoDepository` to the `Controller`.
    // Each `Depository` account manages a specific collateral mint.
    // A `Depository` account owns a `collateral_passthrough` PDA as the owner of the mango account and
    //   the token account must be the same so we can't move fund directly from the use to Mango.
    // A `Depository` account own a `mango_account` PDA to deposit, withdraw, and open orders on Mango Market.
    pub fn register_mango_depository(
        ctx: Context<RegisterMangoDepository>,
        bump: u8,
        collateral_passthrough_bump: u8,
        insurance_passthrough_bump: u8,
        mango_account_bump: u8,
    ) -> ProgramResult {
        instructions::register_mango_depository::handler(
            ctx,
            bump,
            collateral_passthrough_bump,
            insurance_passthrough_bump,
            mango_account_bump,
        )
        .map_err(|e| {
            msg!("<*> {}", e); // log the error
            e.into() // convert UxdError to generic ProgramError
        })
    }

    #[access_control(
        check_deposit_insurance_amount_constraints(&ctx, insurance_amount)
    )]
    pub fn deposit_insurance_to_mango_depository(
        ctx: Context<DepositInsuranceToMangoDepository>,
        insurance_amount: u64,
    ) -> ProgramResult {
        instructions::deposit_insurance_to_mango_depository::handler(ctx, insurance_amount).map_err(
            |e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            },
        )
    }

    // Withdraw insurance previously deposited, if any available, in the limit of mango health.
    #[access_control(check_withdraw_insurance_amount_constraints(insurance_amount))]
    pub fn withdraw_insurance_from_mango_depository(
        ctx: Context<WithdrawInsuranceFromMangoDepository>,
        insurance_amount: u64,
    ) -> ProgramResult {
        instructions::withdraw_insurance_from_mango_depository::handler(ctx, insurance_amount)
            .map_err(|e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            })
    }

    // WIP on branch : feature/rebalancing
    // This is intended to be in the later version of UXD - At release we will cap the UXD supply and secure with the insurance fund.
    //
    // Currently on hold due to solana account limit per transaction
    // https://docs.solana.com/proposals/transactions-v2#other-proposals
    // Program will remain hard capped in term of redeemable until this is implemented.
    // This will allow to prevent liquidation thanks to the repurposed insurance fund in the meantime.
    //
    // Reduce or increase the delta neutral position size to account for it's current PnL.
    // Update accounting, check accounting.
    // #[access_control(
    //     valid_slippage(slippage)
    //     check_max_rebalancing_amount_constraints(max_rebalancing_amount)
    // )]
    // pub fn rebalance_mango_depository(
    //     ctx: Context<RebalanceMangoDepository>,
    //     max_rebalancing_amount: u64,
    //     slippage: u32,
    // ) -> ProgramResult {
    //     instructions::rebalance_mango_depository::handler(ctx, max_rebalancing_amount, slippage)
    // }

    // Mint Redeemable tokens by depositing Collateral to mango and opening the equivalent short perp position.
    // Callers pays taker_fees, that are deducted from the returned redeemable tokens (and part of the delta neutral position)
    #[access_control(
        valid_slippage(slippage)
        check_collateral_amount_constraints(&ctx, collateral_amount)
    )]
    pub fn mint_with_mango_depository(
        ctx: Context<MintWithMangoDepository>,
        collateral_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
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
        valid_slippage(slippage)
        check_redeemable_amount_constraints(&ctx, redeemable_amount)
    )]
    pub fn redeem_from_mango_depository(
        ctx: Context<RedeemFromMangoDepository>,
        redeemable_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        instructions::redeem_from_mango_depository::handler(ctx, redeemable_amount, slippage)
            .map_err(|e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            })
    }
}

// MARK: - ACCESS CONTROL  ----------------------------------------------------

const SLIPPAGE_BASIS: u32 = 1000;
const SOLANA_MAX_MINT_DECIMALS: u8 = 9;

// Asserts that the redeemable mint decimals is between 0 and 9.
fn valid_redeemable_mint_decimals<'info>(decimals: u8) -> ProgramResult {
    check!(
        decimals <= SOLANA_MAX_MINT_DECIMALS,
        UxdErrorCode::InvalidRedeemableMintDecimals
    )?;
    Ok(())
}

// Asserts that the redeemable global supply cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
fn valid_redeemable_global_supply_cap<'info>(redeemable_global_supply_cap: u128) -> ProgramResult {
    check!(
        redeemable_global_supply_cap <= MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP,
        UxdErrorCode::InvalidRedeemableGlobalSupplyCap
    )?;
    Ok(())
}

// Asserts that the Mango Depositories redeemable soft cap is between 0 and MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.
fn valid_mango_depositories_redeemable_soft_cap<'info>(redeemable_soft_cap: u64) -> ProgramResult {
    check!(
        redeemable_soft_cap <= MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP,
        UxdErrorCode::InvalidMangoDepositoriesRedeemableSoftCap
    )?;
    Ok(())
}

// Asserts that the amount of usdc for the operation is above 0.
// Asserts that the amount of usdc is available in the user account.
fn valid_slippage<'info>(slippage: u32) -> ProgramResult {
    check!(slippage <= SLIPPAGE_BASIS, UxdErrorCode::InvalidSlippage)?;
    Ok(())
}

pub fn check_collateral_amount_constraints<'info>(
    ctx: &Context<MintWithMangoDepository<'info>>,
    collateral_amount: u64,
) -> ProgramResult {
    check!(collateral_amount > 0, UxdErrorCode::InvalidCollateralAmount)?;
    check!(
        ctx.accounts.user_collateral.amount >= collateral_amount,
        UxdErrorCode::InsufficientCollateralAmount
    )?;
    Ok(())
}

pub fn check_redeemable_amount_constraints<'info>(
    ctx: &Context<RedeemFromMangoDepository<'info>>,
    redeemable_amount: u64,
) -> ProgramResult {
    check!(redeemable_amount > 0, UxdErrorCode::InvalidRedeemableAmount)?;
    check!(
        ctx.accounts.user_redeemable.amount >= redeemable_amount,
        UxdErrorCode::InsufficientRedeemableAmount
    )?;
    Ok(())
}

pub fn check_deposit_insurance_amount_constraints<'info>(
    ctx: &Context<DepositInsuranceToMangoDepository<'info>>,
    insurance_amount: u64,
) -> ProgramResult {
    check!(insurance_amount > 0, UxdErrorCode::InvalidInsuranceAmount)?;
    check!(
        ctx.accounts.authority_insurance.amount >= insurance_amount,
        UxdErrorCode::InsufficientAuthorityInsuranceAmount
    )?;
    Ok(())
}

pub fn check_withdraw_insurance_amount_constraints<'info>(insurance_amount: u64) -> ProgramResult {
    check!(insurance_amount > 0, UxdErrorCode::InvalidInsuranceAmount)?;
    // Mango withdraw will fail with proper error thanks to  `disabled borrow` set to true if the balance is not enough.
    Ok(())
}

pub fn check_max_rebalancing_amount_constraints(max_rebalancing_amount: u64) -> ProgramResult {
    check!(
        max_rebalancing_amount > 0,
        UxdErrorCode::InvalidRebalancedAmount
    )?;
    Ok(())
}
