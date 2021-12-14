use anchor_lang::error;

#[error(offset = 200)]
pub enum ErrorCode {
    #[msg("The redeemable mint decimals must be between 0 and 9 (inclusive).")]
    InvalidRedeemableMintDecimals,
    #[msg("The redeemable global supply cap must be below MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.")]
    InvalidRedeemableGlobalSupplyCap,
    #[msg("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[msg("The slippage value is invalid. Must be in the [0...1000] range points.")]
    InvalidSlippage,
    #[msg("The provided collateral mint does not match the depository's collateral mint.")]
    InvalidCollateralMint,
    #[msg("The provided insurance mint does not match the depository's insurance mint.")]
    InvalidInsuranceMint, // - 205
    #[msg("Collateral amount must be > 0 in order to mint.")]
    InvalidCollateralAmount,
    #[msg("The balance of the collateral ATA is not enough to fulfill the mint operation.")]
    InsuficientCollateralAmount,
    #[msg("The redeemable amount for redeem must be superior to 0.")]
    InvalidRedeemableAmount, // 0xd0 - 208
    #[msg("The balance of the redeemable ATA is not enough to fulfill the redeem operation.")]
    InsuficientRedeemableAmount, // 0xd1 - 209
    //
    #[msg("Only the Program initializer authority can access this instructions.")]
    InvalidAuthority = 10, // - 210
    #[msg("The Redeemable Mint provided does not match the Controller's one.")]
    InvalidRedeemableMint,
    #[msg("The user's Redeemable ATA's mint does not match the Controller's one.")]
    InvalidUserRedeemableATAMint,
    #[msg("The user's Collateral ATA's mint does not match the Depository's one.")]
    InvalidUserCollateralATAMint,
    #[msg("The authority's Insurance ATA's mint does not match the Depository's one.")]
    InvalidAuthorityInsuranceATAMint,
    #[msg("The Depository Collateral Passthrough ATA's mint does not match the Depository's one.")]
    InvalidCollateralPassthroughATAMint,
    #[msg("The Depository Insurance Passthrough ATA's mint does not match the Depository's one.")]
    InvalidInsurancePassthroughATAMint, // 0xd8 - 216
    #[msg("The perp position could not be fully filled with the provided slippage.")]
    PerpOrderPartiallyFilled, // 0xd9 - 217
    #[msg("Error while getting the redeemable value of the deposited coin amount.")]
    PositionAmountCalculation,
    #[msg("Minting amount would go past the Redeemable Global Supply Cap.")]
    RedeemableGlobalSupplyCapReached,
    #[msg("Operation not allowed due to being over the Redeemable soft Cap.")]
    MangoDepositoriesSoftCapOverflow, // 0xdc - 220
    //
    #[msg("Cannot register more mango depositories, the limit has been reached.")]
    MaxNumberOfMangoDepositoriesRegisteredReached = 30, // - 230
    #[msg("The Depository's controller doesn't match the provided Controller.")]
    InvalidController,
    #[msg("The Depository provided is not registered with the Controller.")]
    InvalidDepository,
    #[msg("The Collateral Passthrough Account isn't the Deposiroty one.")]
    InvalidCollateralPassthroughAccount,
    #[msg("The Insurance Passthrough Account isn't the Deposiroty one.")]
    InvalidInsurancePassthroughAccount,
    #[msg("The Mango Account isn't the Deposiroty one.")]
    InvalidMangoAccount,
    #[msg("The amount to withdraw from the Insurance Fund must be superior to zero..")]
    InvalidInsuranceAmount,
    #[msg("The Insurance ATA from authority doesn't have enough balance.")]
    InsuficientAuthorityInsuranceAmount,
    #[msg("The max amount to rebalance must be superior to zero..")]
    InvalidRebalancingAmount,
    #[msg("Insuficcent order book depth for order.")]
    InsuficentOrderBookDepth,
    #[msg("The executed order size does not match the expected one.")]
    InvalidExecutedOrderSize,

    // Mango Errors Wrappers
    #[msg("Could not load Mango Order book.")]
    MangoOrderBookLoading = 80,
    #[msg("Could not load Mango Group.")]
    MangoGroupLoading,
    #[msg("Could not load Mango Cache.")]
    MangoCacheLoading,
    #[msg("Could not load Mango PerpMarket.")]
    MangoLoadPerpMarket,
    #[msg("Could not load Mango Account.")]
    MangoAccountLoading,
    #[msg("Could not find the perp market index for the given collateral.")]
    MangoPerpMarketIndexNotFound,
    #[msg("The Mango PerpAccount has uncommitted changes.")]
    InvalidPerpAccountState, // 0x11e - 286
    #[msg("The Depository accounting is in an invalid state.")]
    InvalidDepositoryAccounting, // 0x11f - 287
}
