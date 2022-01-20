use anchor_lang::prelude::*;

#[error_code]
pub enum UxdError {
    /// Program errors
    ///
    #[msg("The redeemable mint decimals must be between 0 and 9 (inclusive).")]
    InvalidRedeemableMintDecimals,
    #[msg("Redeemable global supply above.")]
    InvalidRedeemableGlobalSupplyCap,
    #[msg("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[msg("The provided limit_price value is invalid, must be > 0")]
    InvalidLimitPrice,
    #[msg("Could not fill the order given order book state and provided slippage.")]
    EffectiveOrderPriceBeyondLimitPrice,
    #[msg("Collateral amount must be > 0 in order to mint.")]
    InvalidCollateralAmount,
    #[msg("The balance of the collateral ATA is not enough to fulfill the mint operation.")]
    InsufficientCollateralAmount,
    #[msg("The redeemable amount for redeem must be superior to 0.")]
    InvalidRedeemableAmount,
    #[msg("The balance of the redeemable ATA is not enough to fulfill the redeem operation.")]
    InsufficientRedeemableAmount,
    #[msg("The perp position could not be fully filled with the provided slippage.")]
    PerpOrderPartiallyFilled,
    #[msg("Minting amount would go past the Redeemable Global Supply Cap.")]
    RedeemableGlobalSupplyCapReached,
    #[msg("Operation not allowed due to being over the Mango Redeemable soft Cap.")]
    MangoDepositoriesSoftCapOverflow,
    #[msg("Cannot register more mango depositories, the limit has been reached.")]
    MaxNumberOfMangoDepositoriesRegisteredReached,
    #[msg("The amount to withdraw from the Insurance Fund must be superior to zero..")]
    InvalidInsuranceAmount,
    #[msg("The Insurance ATA from authority doesn't have enough balance.")]
    InsufficientAuthorityInsuranceAmount,
    #[msg("The rebalanced amount must be superior to zero..")]
    InvalidRebalancedAmount,
    #[msg("Insufficient order book depth for order.")]
    InsufficientOrderBookDepth,
    #[msg("The executed order size does not match the expected one.")]
    InvalidExecutedOrderSize,
    #[msg("Mango depositories redeemable soft cap above.")]
    InvalidMangoDepositoriesRedeemableSoftCap,
    #[msg("Quote_lot_delta can't be 0.")]
    InvalidQuoteDelta,
    #[msg("The perp order wasn't executed in the right direction.")]
    InvalidOrderDirection,
    #[msg("Math error.")]
    MathError,
    // Deprecated
    #[msg("The order couldn't be executed with the provided slippage.")]
    SlippageReached,
    #[msg("The rebalancing amount must be above 0.")]
    InvalidRebalancingAmount,
    #[msg("The Quote amount in the provided user_quote ATA must be >= max_amount_rebalancing.")]
    InsufficientQuoteAmount,
    #[msg("The PnL polarity provided is not the same as the perp position's one.")]
    InvalidPnlPolarity,
    #[msg("The rebalanced amount doesn't match the expected rebalance amount.")]
    RebalancingError,
    #[msg("A bump was expected but is missing.")]
    BumpError,
    #[msg("The order is below size is below the min lot size.")]
    OrderSizeBelowMinLotSize,
    #[msg("The collateral delta post perp order doesn't match the planned one.")]
    InvalidCollateralDelta,
    #[msg("The perp market index could not be found for this MangoMarkets Pair.")]
    MangoPerpMarketIndexNotFound,
    #[msg("Cannot register more ZO depositories, the limit has been reached.")]
    MaxNumberOfZoDepositoriesRegisteredReached,
    #[msg("Operation not allowed due to being over the ZO Redeemable soft Cap.")]
    ZoDepositoriesSoftCapOverflow,

    /// Anchor DSL related errors
    ///
    #[msg("Only the Program initializer authority can access this instructions.")]
    InvalidAuthority,
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
    #[msg("The Zo Account isn't the Depository's one.")]
    InvalidZoAccount,
    #[msg("The Zo PerpMarket index could not be found.")]
    ZOPerpMarketNotFound,
    #[msg("The Zo PerpMarketInfo could not be found.")]
    ZOPerpMarketInfoNotFound,
    #[msg("The Zo OpenOrdersInfo could not be found.")]
    ZOOpenOrdersInfoNotFound,
    #[msg("The Zo Control is in an invalid state.")]
    ZOInvalidControlState,
    #[msg("The Zo depository PDAs are already initialized.")]
    ZoDepositoryAlreadyInitialized,
    #[msg("The Zo depository PDAs haven't been initialized yet.")]
    ZoDepositoryNotInitialized,

    #[msg("Default - Check the source code for more info")]
    Default,
}
