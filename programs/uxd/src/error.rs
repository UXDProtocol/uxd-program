use crate::MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP;
use crate::MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP;
use anchor_lang::prelude::error;
use anchor_lang::prelude::ProgramError;
use mango::error::MangoError;
use num_enum::IntoPrimitive;
use thiserror::Error;

#[error_code]
pub enum UxdError {
    /// Program errors
    ///
    #[error("The redeemable mint decimals must be between 0 and 9 (inclusive).")]
    InvalidRedeemableMintDecimals = 0,
    #[error("Redeemable global supply above {}.", MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP)]
    InvalidRedeemableGlobalSupplyCap,
    #[error("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[error("The slippage value is invalid. Must be in the [0...1000] range points.")]
    InvalidSlippage,
    #[error("Could not fill the order given order book state and provided slippage.")]
    EffectiveOrderPriceBeyondLimitPrice,
    #[error("Collateral amount must be > 0 in order to mint.")]
    InvalidCollateralAmount,
    #[error("The balance of the collateral ATA is not enough to fulfill the mint operation.")]
    InsufficientCollateralAmount,
    #[error("The redeemable amount for redeem must be superior to 0.")]
    InvalidRedeemableAmount,
    #[error("The balance of the redeemable ATA is not enough to fulfill the redeem operation.")]
    InsufficientRedeemableAmount,
    #[error("The perp position could not be fully filled with the provided slippage.")]
    PerpOrderPartiallyFilled,
    #[error("Minting amount would go past the Redeemable Global Supply Cap.")]
    RedeemableGlobalSupplyCapReached = 10,
    #[error("Operation not allowed due to being over the Redeemable soft Cap.")]
    MangoDepositoriesSoftCapOverflow,
    #[error("Cannot register more mango depositories, the limit has been reached.")]
    MaxNumberOfMangoDepositoriesRegisteredReached,
    #[error("The amount to withdraw from the Insurance Fund must be superior to zero..")]
    InvalidInsuranceAmount,
    #[error("The Insurance ATA from authority doesn't have enough balance.")]
    InsufficientAuthorityInsuranceAmount,
    #[error("The rebalanced amount must be superior to zero..")]
    InvalidRebalancedAmount,
    #[error("Insufficient order book depth for order.")]
    InsufficientOrderBookDepth,
    #[error("The executed order size does not match the expected one.")]
    InvalidExecutedOrderSize,
    #[error("Could not find the perp market index for the given collateral.")]
    MangoPerpMarketIndexNotFound,
    #[error(
        "Mango depositories redeemable soft cap above {}.",
        MAX_MANGO_DEPOSITORIES_REDEEMABLE_SOFT_CAP
    )]
    InvalidMangoDepositoriesRedeemableSoftCap,
    #[error("Quote_lot_delta can't be 0.")]
    InvalidQuoteDelta = 20,
    #[error("The perp order wasn't executed in the right direction.")]
    InvalidOrderDirection,
    #[error("Math error.")]
    MathError,
    #[error("The order couldn't be executed with the provided slippage.")]
    SlippageReached,
    #[error("The rebalancing amount must be above 0.")]
    InvalidRebalancingAmount,
    #[error("The Quote amount in the provided user_quote ATA must be >= max_amount_rebalancing.")]
    InsufficientQuoteAmount,
    #[error("The PnL polarity provided is not the same as the perp position's one.")]
    InvalidPnlPolarity,
    #[error("The rebalanced amount doesn't match the expected rebalance amount.")]
    RebalancingError,
    #[error("A bump was expected but is missing.")]
    BumpError,
    #[error("The order is below size is below the min lot size.")]
    OrderSizeBelowMinLotSize,
    #[error("The collateral delta post perp order doesn't match the planned one.")]
    InvalidCollateralDelta,
    #[error("MangoErrorCode::Default Check the source code for more info")]
    Default = u32::MAX,

    /// Anchor DSL related errors
    ///
    #[msg("Only the Program initializer authority can access this instructions.")]
    InvalidAuthority = 200,
    #[msg("The Depository's controller doesn't match the provided Controller.")]
    InvalidController,
    #[msg("The Depository provided is not registered with the Controller.")]
    InvalidDepository,
    #[msg("The provided collateral mint does not match the depository's collateral mint.")]
    InvalidCollateralMint,
    #[msg("The provided insurance mint does not match the depository's insurance mint.")]
    InvalidInsuranceMint,
    #[msg("The authority's Insurance ATA's mint does not match the Depository's one.")]
    InvalidAuthorityInsuranceATAMint,
    #[msg("The Collateral Passthrough Account isn't the Depository one.")]
    InvalidCollateralPassthroughAccount,
    #[msg("The Insurance Passthrough Account isn't the Depository one.")]
    InvalidInsurancePassthroughAccount,
    #[msg("The Mango Account isn't the Depository one.")]
    InvalidMangoAccount,
    #[msg("The Insurance Passthrough ATA's mint does not match the Depository's one.")]
    InvalidInsurancePassthroughATAMint,
    #[msg("The Redeemable Mint provided does not match the Controller's one.")]
    InvalidRedeemableMint,
    #[msg("The Collateral Passthrough ATA's mint does not match the Depository's one.")]
    InvalidCollateralPassthroughATAMint,
    #[msg("The Quote Passthrough Account isn't the Depository one.")]
    InvalidQuotePassthroughAccount,
    #[msg("The Quote Passthrough ATA's mint does not match the Depository's one.")]
    InvalidQuotePassthroughATAMint,
    #[msg("The provided quote mint does not match the depository's quote mint.")]
    InvalidQuoteMint,
    #[msg("The instruction doesn't support this version of the Depository. Migrate first.")]
    UnsupportedDepositoryVersion,
}

impl From<MangoError> for UxdError {
    fn from(me: MangoError) -> Self {
        let pe: ProgramError = me.into();
        pe.into()
    }
}
