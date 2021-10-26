use anchor_lang::prelude::*;

#[error]
pub enum ControllerError {
    #[msg("Error while getting the UXD value of the deposited coin amount.")]
    PositionAmountCalculation,
    #[msg("The associated mango root bank index cannot be found for the deposited coin..")]
    RootBankIndexNotFound,
    #[msg("The slippage value is invalid. Must be in the [0...1000] range points.")]
    InvalidSlippage,
    #[msg("The perp position could not be fully filled with the provided slippage.")]
    PerpPartiallyFilled,
}
