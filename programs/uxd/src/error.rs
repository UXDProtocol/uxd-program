use anchor_lang::prelude::*;

#[error]
pub enum UXDError {
    #[msg("Only the Program initializer authority can access this instructions.")]
    InvalidAuthority,
    #[msg("Error while getting the redeemable value of the deposited coin amount.")]
    // WAT? improve
    PositionAmountCalculation,
    #[msg("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[msg("The slippage value is invalid. Must be in the [0...1000] range points.")]
    InvalidSlippage,
    #[msg("The perp position could not be fully filled with the provided slippage.")]
    PerpOrderPartiallyFilled,
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
    #[msg("The Redeemable Mint provided does not match the Controller's one.")]
    InvalidRedeemableMint,
    #[msg("The Collateral Mint provided does not match the one from the depository.")]
    MintMismatchCollateral,
    #[msg("The user's Redeemable ATA's mint does not match the Controller's one.")]
    InvalidUserRedeemableATAMint,
}
