use crate::utils::calculate_amount_less_fees;
use anchor_lang::prelude::*;

pub trait DepositoryConfiguration {
    // Maximum outstanding supply that can be minted using this depository
    fn get_minted_redeemable_soft_cap(&self) -> u128;

    // Fee applied at minting, expressed in basis point (bps) and taken by minting less redeemable for the user.
    // E.g, with a minting fee of 5 bps, if the user mint for 1_000_000 USDC (6 decimals), it should receive 999_500 UXD (6 decimals)
    // Calculation: (10_000 - 5) * 1_000_000 / 10_000
    fn get_minting_fees_bps(&self) -> u8;
    // Fee applied at redeeming, expressed in basis point (bps) and taken by redeeming less lp token from the mercurial vault
    // thus sending less collateral to the user.
    // E.g, with a redeeming fee of 5 bps, if the user redeem for 1_000_000 UXD (6 decimals), it should receive 999_500 USDC (6 decimals)
    // Calculation: (10_000 - 5) * 1_000_000 / 10_000
    fn get_redeeming_fees_bps(&self) -> u8;

    fn substract_minting_fees_amount(&self, amount: u64) -> Result<u64> {
        return calculate_amount_less_fees(amount, self.get_minting_fees_bps());
    }
    fn substract_redeeming_fees_amount(&self, amount: u64) -> Result<u64> {
        return calculate_amount_less_fees(amount, self.get_redeeming_fees_bps());
    }
}
