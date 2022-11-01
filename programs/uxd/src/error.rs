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
    #[msg("Collateral amount cannot be 0")]
    InvalidCollateralAmount,
    #[msg("Quote amount must be > 0 in order to mint.")]
    InvalidQuoteAmount,
    #[msg("Redeemable amount must be > 0 in order to redeem.")]
    InvalidRedeemableAmount,
    #[msg("The balance of the collateral ATA is not enough to fulfill the mint operation.")]
    InsufficientCollateralAmount,
    #[msg("The balance of the quote ATA is not enough to fulfil the mint operation.")]
    InsufficientQuoteAmountMint,
    #[msg("The balance of the redeemable ATA is not enough to fulfil the redeem operation.")]
    InsufficientRedeemableAmountMint,
    #[msg("The balance of the redeemable ATA is not enough to fulfill the redeem operation.")]
    InsufficientRedeemableAmount,
    #[msg("The perp position could not be fully filled with the provided slippage.")]
    PerpOrderPartiallyFilled,
    #[msg("Minting amount would go past the Redeemable Global Supply Cap.")]
    RedeemableGlobalSupplyCapReached,
    #[msg(
        "Minting amount would go past the mango depository Redeemable Amount Under Management Cap."
    )]
    RedeemableMangoAmountUnderManagementCap,
    #[msg("Minting amount would go past the mercurial vault depository Redeemable Amount Under Management Cap.")]
    RedeemableMercurialVaultAmountUnderManagementCap,
    #[msg("Minting amount would go past the maple pool depository Redeemable Amount Under Management Cap.")]
    RedeemableMaplePoolAmountUnderManagementCap,
    #[msg("Operation not allowed due to being over the Mango Redeemable soft Cap.")]
    MangoDepositoriesSoftCapOverflow,
    #[msg("Cannot register more mango depositories, the limit has been reached.")]
    MaxNumberOfMangoDepositoriesRegisteredReached,
    #[msg("Cannot register more maple pool depositories, the limit has been reached.")]
    MaxNumberOfMaplePoolDepositoriesRegisteredReached,
    #[msg("The amount to withdraw from the Insurance Fund must be superior to zero..")]
    InvalidInsuranceAmount,
    #[msg("The Quote ATA from authority doesn't have enough balance.")]
    InsufficientAuthorityQuoteAmount,
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
    #[msg("Could not load the provided MangoGroup account.")]
    CannotLoadMangoGroup,
    #[msg("The order quantity is below contract_size of the perp market.")]
    QuantityBelowContractSize,
    #[msg("The amount trying to be quote minted is larger than quote mintable.")]
    QuoteAmountTooHigh,
    #[msg("The amount trying to be quote redeemed is larger than quote redeemable.")]
    RedeemableAmountTooHigh,
    #[msg("Minting is disabled for the current depository.")]
    MintingDisabled,
    #[msg("Minting is already disabled/enabled.")]
    MintingAlreadyDisabledOrEnabled,
    #[msg("The quote amount requested is beyond the soft cap limitation.")]
    QuoteAmountExceedsSoftCap,
    #[msg("The quote currency is not the expected one.")]
    InvalidQuoteCurrency,
    #[msg("The mercurial vault lp mint does not match the Depository's one.")]
    InvalidMercurialVaultLpMint,
    #[msg("Cannot register more mercurial vault depositories, the limit has been reached.")]
    MaxNumberOfMercurialVaultDepositoriesRegisteredReached,
    #[msg("The provided collateral do not match the provided mercurial vault token.")]
    MercurialVaultDoNotMatchCollateral,
    #[msg("The provided collateral do not match the provided maple pool token.")]
    MaplePoolDoNotMatchCollateral,
    #[msg("Collateral mint should be different than redeemable mint.")]
    CollateralMintEqualToRedeemableMint,
    #[msg("Provided collateral mint is not allowed.")]
    CollateralMintNotAllowed,
    #[msg("Collateral deposit left some value unaccounted for.")]
    CollateralDepositHasRemainingDust,
    #[msg("Collateral deposit result in funds movements that doesn't match expectations.")]
    CollateralDepositUnaccountedFor,
    #[msg("Collateral deposit didn't result in the correct amounts being moved")]
    CollateralDepositAmountsDoesntMatch,
    #[msg("Received token of which the value doesn't match the deposited collateral.")]
    CollateralDepositDoesntMatchTokenValue,
    #[msg("Mint resulted to 0 redeemable token being minted.")]
    MinimumMintedRedeemableAmountError,
    #[msg("Redeem resulted to 0 collateral token being redeemed.")]
    MinimumRedeemedCollateralAmountError,
    #[msg("The depository lp token vault does not match the Depository's one.")]
    InvalidDepositoryLpTokenVault,
    #[msg("The mango group is not accepted.")]
    UnAllowedMangoGroup,

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
    #[msg("The provided collateral locker does not match the depository's collateral locker.")]
    InvalidCollateralLocker,
    #[msg("The provided quote mint does not match the depository's quote mint.")]
    InvalidQuoteMint,
    #[msg("The Mango Account isn't the Depository one.")]
    InvalidMangoAccount,
    #[msg("The Redeemable Mint provided does not match the Controller's one.")]
    InvalidRedeemableMint,
    #[msg("The provided perp_market is not the one tied to this Depository.")]
    InvalidDexMarket,
    #[msg("The provided token account is not owner by the expected party.")]
    InvalidOwner,
    #[msg("The max base quantity must be above 0.")]
    InvalidMaxBaseQuantity,
    #[msg("The max quote quantity must be above 0.")]
    InvalidMaxQuoteQuantity,
    #[msg("The provided mercurial vault does not match the Depository's one.")]
    InvalidMercurialVault,
    #[msg("The provided mercurial vault collateral token safe does not match the mercurial vault one.")]
    InvalidMercurialVaultCollateralTokenSafe,
    #[msg("The provided maple pool does not match the Depository's one.")]
    InvalidMaplePool,
    #[msg("The provided maple pool locker does not match the Depository's one.")]
    InvalidMaplePoolLocker,
    #[msg("The provided maple globals does not match the Depository's one.")]
    InvalidMapleGlobals,
    #[msg("The provided maple lender does not match the Depository's one.")]
    InvalidMapleLender,
    #[msg("The provided maple shares mint does not match the Depository's one.")]
    InvalidMapleSharesMint,
    #[msg("The provided maple locked shares does not match the Depository's one.")]
    InvalidMapleLockedShares,
    #[msg("The provided maple lender shares does not match the Depository's one.")]
    InvalidMapleLenderShares,

    #[msg("Default - Check the source code for more info.")]
    Default,
}
