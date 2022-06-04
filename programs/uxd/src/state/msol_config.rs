use fixed::types::I80F48;

use crate::*;

// 10000 equiv. to 100%
pub const LIQUIDITY_RATIO_BASIS: u16 = 10000;

pub const TARGET_LIQUIDITY_RATIO_MAX: u16 = 10000;

const MSOL_CONFIG_PADDING: usize = 64;

pub const MSOL_CONFIG_SPACE: usize = 8 + 1 + 32 + 32 + 1 + 2 + MSOL_CONFIG_PADDING;

#[account(zero_copy)]
pub struct MSolConfig {
    pub bump: u8,
    // The Depository that this config account dedicated to
    pub depository: Pubkey,
    // The Controller that own this config account
    pub controller: Pubkey,
    // Whether allowing the SOL/mSOL swap
    pub enabled: bool,
    // A constant value of the proportion of SOL we wanna keep from the total amount of SOL + mSOL in terms of their value.
    // In LIQUIDITY_RATIO_BASIS
    pub target_liquidity_ratio: u16,
}

impl MSolConfig {
    pub fn target_liquidity_ratio(&self) -> Result<I80F48> {
        I80F48::checked_from_num(self.target_liquidity_ratio)
            .ok_or_else(|| error!(UxdError::MathError))?
            .checked_div(
                I80F48::checked_from_num(LIQUIDITY_RATIO_BASIS)
                    .ok_or_else(|| error!(UxdError::MathError))?,
            )
            .ok_or_else(|| error!(UxdError::MathError))
    }

    pub fn diff_to_target_liquidity(&self, liquidity_ratio: I80F48) -> Result<I80F48> {
        let target_liquidity_ratio = self.target_liquidity_ratio()?;
        msg!("target_liquidity_ratio {:?}", target_liquidity_ratio);
        liquidity_ratio
            .checked_sub(target_liquidity_ratio)
            .ok_or_else(|| error!(UxdError::MathError))
    }
}
