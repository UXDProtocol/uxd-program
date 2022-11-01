use crate::{error::UxdError, state::controller::Controller};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

pub fn validate_collateral_mint_usdc(
    collateral_mint: &Account<Mint>,
    controller: &AccountLoader<Controller>,
) -> Result<()> {
    // Only few stablecoin collateral mint are whitelisted
    // Redeemable and collateral should always be 1:1
    #[cfg(feature = "production")]
    {
        let usdc_mint: Pubkey =
            Pubkey::from_str(b"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        require!(
            self.collateral_mint.key().eq(&usdc_mint),
            UxdError::CollateralMintNotAllowed,
        );
    }
    // In development, we can't check the address directly as there are many devnet USDC
    #[cfg(feature = "development")]
    {
        // Collateral mint and redeemable mint should share the same decimals
        // due to the fact that decimal delta is not handled in the mint/redeem instructions
        require!(
            collateral_mint
                .decimals
                .eq(&controller.load()?.redeemable_mint_decimals),
            UxdError::CollateralMintNotAllowed,
        );
        // Collateral mint should be different than redeemable mint
        require!(
            collateral_mint
                .key()
                .ne(&controller.load()?.redeemable_mint),
            UxdError::CollateralMintEqualToRedeemableMint,
        );
    }
    // Done
    Ok(())
}
