use anchor_lang::prelude::*;

#[error]
pub enum UXDError {
    #[msg("Only the Program initializer authority can access this instructions.")]
    InvalidAuthority,
    #[msg("Error while getting the UXD value of the deposited coin amount.")]
    PositionAmountCalculation,
    #[msg("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[msg("The slippage value is invalid. Must be in the [0...1000] range points.")]
    InvalidSlippage,
    #[msg("The perp position could not be fully filled with the provided slippage.")]
    PerpOrderPartiallyFilled,
    #[msg("The provided collateral mint does not match the depository's collateral mint.")]
    UnexpectedCollateralMint,
    #[msg("Collateral amount must be > 0 in order to mint.")]
    InvalidCollateralAmount,
    #[msg("The balance of the collateral ATA is not enough to fulfill the desired mint quantity.")]
    InsuficientCollateralAmount,
    #[msg("The redeem amount must be superior to 0.")]
    InvalidUxdRedeemAmount,
    #[msg("The balance of the uxd ATA is not enough to fulfill the desired mint quantity.")]
    InsuficientUxdAmount,
    #[msg("The provided UXD Mint provided is not valid.")]
    InvalidUxdMint,
    #[msg("The Collateral Mint provided does not match the one from the depository.")]
    MintMismatchCollateral,
    #[msg("The UXD ATA does not have the right mint.")]
    InvalidUserUXDAssocTokenAccount,
}
