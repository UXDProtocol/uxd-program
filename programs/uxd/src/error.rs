use anchor_lang::error;

#[error(offset = 200)]
pub enum ErrorCode {
    #[msg("The redeemable mint decimals must be between 0 and 9 (inclusive).")]
    InvalidRedeemableMintDecimals,
    #[msg("The redeemable global supply cap must be below MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP.")]
    InvalidRedeemableGlobalSupplyCap,
    #[msg("Only the Program initializer authority can access this instructions.")]
    InvalidAuthority,
    #[msg("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[msg("The slippage value is invalid. Must be in the [0...1000] range points.")]
    InvalidSlippage,
    #[msg("The provided collateral mint does not match the depository's collateral mint.")]
    InvalidCollateralMint,
    #[msg("Collateral amount must be > 0 in order to mint.")]
    InvalidCollateralAmount,
    #[msg("The balance of the collateral ATA is not enough to fulfill the mint operation.")]
    InsuficientCollateralAmount,
    #[msg("The redeem amount must be superior to 0.")]
    InvalidRedeemAmount,
    #[msg("The balance of the redeemable ATA is not enough to fulfill the redeem operation.")]
    InsuficientRedeemableAmount,
    //
    #[msg("The Redeemable Mint provided does not match the Controller's one.")]
    InvalidRedeemableMint = 220,
    #[msg("The user's Redeemable ATA's mint does not match the Controller's one.")]
    InvalidUserRedeemableATAMint,
    #[msg("The user's Collateral ATA's mint does not match the Depository's one.")]
    InvalidUserCollateralATAMint,
    #[msg("The perp position could not be fully filled with the provided slippage.")]
    PerpOrderPartiallyFilled,
    #[msg("Error while getting the redeemable value of the deposited coin amount.")]
    PositionAmountCalculation,
    #[msg("Minting amount would go past the Redeemable Global Supply Cap.")]
    RedeemableGlobalSupplyCapReached,
}
