use anchor_lang::prelude::*;

#[error_code]
pub enum UxdError {
    /// Program errors
    ///
    #[msg("The redeemable mint decimals must be between 0 and 9 (inclusive).")]
    InvalidRedeemableMintDecimals,
    #[msg("Redeemable global supply above.")]
    InvalidRedeemableGlobalSupplyCap,
    #[msg("Collateral amount cannot be 0")]
    InvalidCollateralAmount,
    #[msg("Redeemable amount must be > 0 in order to redeem.")]
    InvalidRedeemableAmount,
    #[msg("The balance of the collateral ATA is not enough to fulfill the mint operation.")]
    InsufficientCollateralAmount,
    #[msg("The balance of the redeemable ATA is not enough to fulfill the redeem operation.")]
    InsufficientRedeemableAmount,
    #[msg("Minting amount would go past the Redeemable Global Supply Cap.")]
    RedeemableGlobalSupplyCapReached,
    #[msg("Minting amount would go past the mercurial vault depository Redeemable Amount Under Management Cap.")]
    RedeemableMercurialVaultAmountUnderManagementCap,
    #[msg("Minting amount would go past the credix lp depository Redeemable Amount Under Management Cap.")]
    RedeemableCredixLpAmountUnderManagementCap,
    #[msg("Math error.")]
    MathError,
    #[msg("The order couldn't be executed with the provided slippage.")]
    SlippageReached,
    #[msg("A bump was expected but is missing.")]
    BumpError,
    #[msg("Minting is disabled for the current depository.")]
    MintingDisabled,
    #[msg("Collateral deposit left some value unaccounted for.")]
    CollateralDepositHasRemainingDust,
    #[msg("Collateral deposit didn't result in the correct amounts being moved.")]
    CollateralDepositAmountsDoesntMatch,
    #[msg("Received token of which the value doesn't match the deposited collateral.")]
    CollateralDepositDoesntMatchTokenValue,
    #[msg("The mercurial vault lp mint does not match the Depository's one.")]
    InvalidMercurialVaultLpMint,
    #[msg("Cannot register more mercurial vault depositories, the limit has been reached.")]
    MaxNumberOfMercurialVaultDepositoriesRegisteredReached,
    #[msg("Cannot register more credix lp depositories, the limit has been reached.")]
    MaxNumberOfCredixLpDepositoriesRegisteredReached,
    #[msg("The provided collateral do not match the provided mercurial vault token.")]
    MercurialVaultDoNotMatchCollateral,
    #[msg("The provided collateral do not match the provided credix lp token.")]
    CredixLpDoNotMatchCollateral,
    #[msg("Collateral mint should be different than redeemable mint.")]
    CollateralMintEqualToRedeemableMint,
    #[msg("Provided collateral mint is not allowed.")]
    CollateralMintNotAllowed,
    #[msg("Mint resulted to 0 redeemable token being minted.")]
    MinimumMintedRedeemableAmountError,
    #[msg("Redeem resulted to 0 collateral token being redeemed.")]
    MinimumRedeemedCollateralAmountError,
    #[msg("The depository lp token vault does not match the Depository's one.")]
    InvalidDepositoryLpTokenVault,
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
    #[msg("The Redeemable Mint provided does not match the Controller's one.")]
    InvalidRedeemableMint,
    #[msg("The provided token account is not owner by the expected party.")]
    InvalidOwner,
    #[msg("The provided depository collateral does not match the depository's one.")]
    InvalidDepositoryCollateral,
    #[msg("The provided depository shares does not match the depository's one.")]
    InvalidDepositoryShares,
    #[msg("The Profits beneficiary collateral provided does not match the depository's one.")]
    InvalidProfitsBeneficiaryCollateral,
<<<<<<< HEAD
    #[msg("The Profits beneficiary collateral provided is set to an invalid value.")]
    UninitializedProfitsBeneficiaryCollateral,
=======
>>>>>>> main
    #[msg("The provided mercurial vault does not match the Depository's one.")]
    InvalidMercurialVault,
    #[msg("The provided mercurial vault collateral token safe does not match the mercurial vault one.")]
    InvalidMercurialVaultCollateralTokenSafe,
    #[msg("Minting amount would go past the identity depository Redeemable Amount Under Management Cap.")]
    RedeemableIdentityDepositoryAmountUnderManagementCap,
    #[msg("Program is already frozen/resumed.")]
    ProgramAlreadyFrozenOrResumed,
    #[msg("The program is currently in Frozen state.")]
    ProgramFrozen,

    #[msg("The Credix ProgramState isn't the Depository one.")]
    InvalidCredixProgramState,
    #[msg("The Credix GlobalMarketState isn't the Depository one.")]
    InvalidCredixGlobalMarketState,
    #[msg("The Credix SigningAuthority isn't the Depository one.")]
    InvalidCredixSigningAuthority,
    #[msg("The Credix LiquidityCollateral isn't the Depository one.")]
    InvalidCredixLiquidityCollateral,
    #[msg("The Credix SharesMint isn't the Depository one.")]
    InvalidCredixSharesMint,
    #[msg("The Credix Pass isn't the one owned by the correct depository.")]
    InvalidCredixPass,
    #[msg("The Credix Pass doesn't have the fees exemption.")]
    InvalidCredixPassNoFees,
    #[msg("The Credix Multisig Key isn't the ProgramState one.")]
    InvalidCredixMultisigKey,
    #[msg("The Credix TreasuryCollateral isn't the GlobalMarketState one.")]
    InvalidCredixTreasuryCollateral,

    #[msg("Default - Check the source code for more info.")]
    Default,
}
