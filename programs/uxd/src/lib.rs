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

// CI Uses F3UToS4WKQkyAAs5TwM_21ANq2xNfDRB7tGRWx4DxapaR on Devnet
// (it's auto swapped by the script, keypair are held in target/deployment)
#[cfg(feature = "development")]
solana_program::declare_id!("FbArqRiw6sYeRb4C1iLoXtks1iCRUk2s43o7ZT3cMvQV");
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

    /// Initialize a Controller on chain account.
    ///
    /// Parameters:
    ///     - redeemable_mint_decimals: the decimals of the redeemable mint.
    ///
    /// Note:
    ///  Only one Controller on chain account will ever exist due to the
    ///  PDA derivation seed having no variations.
    ///
    /// Note:
    ///  In the case of UXDProtocol this is the one in charge of the UXD mint,
    ///  and it has been locked to a single Controller to ever exist by only
    ///  having one possible derivation. (but it's been made generic, and we
    ///  could have added the authority to the seed generation for instance).
    ///
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

    /// Sets the `redeemable_global_supply_cap` of the provided `Controller`
    /// account.
    ///
    /// Parameters:
    ///     - redeemable_global_supply_cap: the new value.
    ///
    /// Note:
    ///  The redeemable global supply cap determines the max total supply
    ///  for the redeemable token. Program will abort when an instruction
    ///  that mints new redeemable would bring the circulating supply
    ///  beyond this value.
    ///
    /// Note:
    ///  Purpose of this is to roll out progressively for OI, and limit risks.
    ///
    /// Note:
    ///  If this is set below the current circulating supply of UXD, it would effectively pause Minting.
    ///
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

    /// Sets the `mango_depositories_redeemable_soft_cap` of the provided
    /// `Controller` account.
    ///
    /// Parameters:
    ///     - redeemable_soft_cap: the new value.
    ///
    /// Note:
    ///  The `mango_depositories_redeemable_soft_cap` determines the
    ///  max amount of redeemable tokens that can be minted during a
    ///  single operation.
    ///
    /// Note:
    ///  This only apply to Minting. Redeeming is always possible.
    ///
    /// Note:
    ///  Purpose of this is to control the max amount minted at once on
    ///  MangoMarkets Depositories.
    ///
    /// Note:
    ///  If this is set to 0, it would effectively pause minting on
    ///  MangoMarkets Depositories.
    ///
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

    /// Create a new`MangoDepository` and registers it to the provided
    /// `Controller` account.
    ///
    /// Note:
    ///  Each `MangoDepository` account manages a specific collateral mint.
    ///  They will only transact for this specific mint to segregate funding
    ///  rates/deposit yield and risks.
    ///
    /// Note:
    ///  Each `MangoDepository` owns a MangoAccount for trading spot/perp,
    ///  leveraged.
    ///
    /// Note:
    ///  Several passthrough accounts are required in order to transaction
    ///  with the `mango_account` as the withdrawals can only be done toward
    ///  accounts owned by the MangoAccount owner (here the Depository).
    ///
    /// Note:
    ///  To keep a coherent interface, deposits are also done through
    ///  passthrough accounts.
    ///
    /// Update:
    ///  In the new version of the MangoMarket Accounts
    ///  this become mandatory too. (we are still using the old init)
    ///
    pub fn register_mango_depository(ctx: Context<RegisterMangoDepository>) -> ProgramResult {
        msg!("[register_mango_depository]");
        instructions::register_mango_depository::handler(ctx).map_err(|e| {
            msg!("<*> {}", e); // log the error
            e.into() // convert UxdError to generic ProgramError
        })
    }

    /// Migrates a `MangoDepository` to the update memory layout.
    /// Added a new "passthrough" account to handle quote currency,
    /// necessary for rebalancing the perp PnL.
    ///
    pub fn migrate_mango_depository_to_v2(
        ctx: Context<MigrateMangoDepositoryToV2>,
    ) -> ProgramResult {
        msg!("[migrate_mango_depository_to_v2]");
        instructions::migrate_mango_depository_to_v2::handler(ctx).map_err(|e| {
            msg!("<*> {}", e); // log the error
            e.into() // convert UxdError to generic ProgramError
        })
    }

    /// Deposit `MangoDepository.insurance_mint` tokens in the `MangoDepository`
    /// underlying `MangoAccount`
    ///
    /// Parameters:
    ///     - insurance_amount: the amount of token to deposit in native unit.
    ///
    /// Note:
    ///  Each `MangoDepository` underlying `MangoAccount` uses leverage to open
    ///  and maintain short positions.
    ///
    /// Note:
    ///  The LTV (Loan to value) ratio is different depending of the mint of
    ///  the `MangoDepository.collateral_mint`.
    ///
    /// Note:
    ///  LTV for BTC/ETH/SOL is at 0.9:1 (0.9$ lent for 1$ of value deposited).
    ///  MangoMarkets Assets specs : https://docs.mango.markets/mango/token-specs
    ///
    /// Note:
    ///  Beyond 80% the `MangoAccount` cannot borrow further, disabling the
    ///  redemption of redeemable tokens or the withdrawal of deposited insurance.
    ///  (Although the insurance should be gone at that point due to funding,
    ///  except in the case of sharp collateral price increase without rebalancing)
    ///
    /// Note:
    ///  Beyond 90% the `MangoAccount` can be liquidated by other mango accounts.
    ///  (And borrows/withdraws are still disabled)
    ///
    /// Note:
    ///  As the funding rate care be either negative or positive, the insurance
    ///  is there as a buffer to ensure that redeemables can be swapped back
    ///  at all time (by unwinding the backing amount of delta neutral
    ///  position).
    ///
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

    /// Withdraw `MangoDepository.insurance_mint` tokens from the `MangoDepository`
    /// underlying `MangoAccount`, if any available, in the limit of the account
    /// borrow health.
    ///
    /// Parameters:
    ///     - insurance_amount: the amount of token to withdraw in native unit.
    ///
    /// Note:
    ///  Withdrawal cannot borrow, nor bring the health of the account in
    ///  liquidation territory.
    ///
    /// Notes:
    ///  The `MangoDepository.insurance_amount_deposited` tracks the amount of
    ///  `MangoDepository.insurance_mint` tokens deposited, but does not represent
    ///  the available amount as it moves depending of funding rates and
    ///  perp positions PnL settlement (temporarily).
    ///
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

    /// Rebalance the delta neutral position of the underlying `MangoDepository`.
    ///
    /// Parameters:
    ///     - max_rebalancing_amount: the maximum amount of quote this rebalance
    ///        instruction will attempt to rebalance, in native unit.
    ///     - polarity: the direction of the rebalancing. This is known on chain
    ///        but required as an argument for clarity.
    ///     - slippage: the maximum deviation in price the user is ok to take
    ///        compared to market price at execution time.
    ///
    /// Note:
    ///  Acts as a swap, reducing the oustanding PnL (paper profit or losses) on
    ///  the underlying `MangoAccount`.
    ///
    /// Note:
    ///  This is the "lite" version as it force the caller to input some quote or
    ///  collateral. This is done to skip the spot order on mango, saving computing
    ///  and also bypassing the issue with teh 34 accounts limits.
    ///  A new version is designed and waiting for the TransactionV2 proposal to hit
    ///  along with the 1M computing units.
    ///
    /// Note:
    ///  Paper profits are represented in Quote, it's currently USDC on
    ///  MangoMarkets, as of 02/17/2022.
    ///
    /// Note:
    ///  This call should goes with a call to `@uxdprotocol/uxd-client`'s
    ///  `MangoDepository.settleMangoDepositoryMangoAccountPnl()`, which convert paper
    ///  profits or losses into realized gain/losses. Once rebalancing is out,
    ///  since it's permissionless, the PnL settlement should be called once in a while
    ///  to make sure that unsettled Positive PNL accumulates and that the MangoAccount
    ///  has to pay borrow rates for it. Some day when computing is plentiful and input
    ///  accounts are increased through TransactionsV2 proposal, we can
    ///  also call the onchain version.
    #[access_control(ctx.accounts.validate(max_rebalancing_amount, &polarity, slippage))]
    pub fn rebalance_mango_depository_lite(
        ctx: Context<RebalanceMangoDepositoryLite>,
        max_rebalancing_amount: u64,
        polarity: PnlPolarity,
        slippage: u32,
    ) -> ProgramResult {
        msg!(
            "[rebalance_mango_depository_lite] slippage {}, polarity {}",
            slippage,
            polarity
        );
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

    /// Mint redeemable tokens in exchange of `MangoDepository.collateral_mint`
    /// tokens, increasing the size of the delta neutral position.
    ///
    /// Parameters:
    ///     - collateral_amount: the amount of collateral to use, in
    ///        collateral_mint native unit.
    ///     - slippage: the maximum deviation in price the user is ok to take
    ///        compared to market price at execution time.
    ///
    /// Flow:
    ///  - Starts by scanning the order book for the amount that we can fill.
    ///  - First transfer collateral_amount from the user collateral ATA to the
    ///     passthrough account.
    ///  - Second transfer is done from passthrough to the mango account balance.
    ///  - Using the spot collateral deposited, the short perp position of equivalent
    ///     size if opened (FoK emulated by using mango IoC + 100% fill verification).
    ///  - Deducts the taker_fees (ceiled) form the value of the opened short, and
    ///     mints the redeemable, then transfer to the user.
    ///  - Internal accounting update + anchor event emission.
    ///  
    /// Note:
    ///  The caller pays for the incurred slippage and taker_fee (4bps at the time
    ///  of writing). This ensures that the system stay "closed".
    ///
    /// Note:
    ///  The value of the collateral is derived from the COLLATERAL-PERP price,
    ///  expressed in USD value.
    ///
    #[access_control(
        ctx.accounts.validate(collateral_amount, slippage)
    )]
    pub fn mint_with_mango_depository(
        ctx: Context<MintWithMangoDepository>,
        collateral_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        msg!(
            "[mint_with_mango_depository] collateral_amount {}, slippage {}",
            collateral_amount,
            slippage
        );
        instructions::mint_with_mango_depository::handler(ctx, collateral_amount, slippage).map_err(
            |e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            },
        )
    }

    /// Redeem `MangoDepository.collateral_mint` by burning redeemable tokens
    /// tokens, and unwind a part of the delta neutral position.
    ///
    /// Parameters:
    ///     - redeemable_amount: the amount of collateral to use, in
    ///        redeemable_mint native unit.
    ///     - slippage: the maximum deviation in price the user is ok to take
    ///        compared to market price at execution time.
    ///
    /// Flow:
    ///  - Starts by scanning the order book to find the best order for
    ///     the redeemable_amount fillable (the requested amount minus max
    ///     fees, as we repay them by keeping a piece of the DN position).
    ///  - Closes the equivalent part of the delta neutral position (FoK
    ///     emulated by using mango IoC + 100% fill verification).
    ///  - Deducts the taker_fees (ceiled) form the value of the opened short, and
    ///     transfer user redeemable token for that amount.
    ///  - Burns the redeemable equivalent to fees + closed position,
    ///     then transfer resulting equivalent collateral to the user (using
    ///     the passthrough account).
    ///  - Internal accounting update + anchor event emission.
    ///  
    /// Note:
    ///  The caller pays for the incurred slippage and taker_fee (4bps at the time
    ///  of writing). This ensures that the system stay "closed".
    ///
    /// Note:
    ///  The value of the collateral is derived from the COLLATERAL-PERP price,
    ///  expressed in USD value.
    ///
    #[access_control(
        ctx.accounts.validate(redeemable_amount, slippage)
    )]
    pub fn redeem_from_mango_depository(
        ctx: Context<RedeemFromMangoDepository>,
        redeemable_amount: u64,
        slippage: u32,
    ) -> ProgramResult {
        msg!(
            "[redeem_from_mango_depository] redeemable_amount {}, slippage {}",
            redeemable_amount,
            slippage
        );
        instructions::redeem_from_mango_depository::handler(ctx, redeemable_amount, slippage)
            .map_err(|e| {
                msg!("<*> {}", e); // log the error
                e.into() // convert UxdError to generic ProgramError
            })
    }
}
