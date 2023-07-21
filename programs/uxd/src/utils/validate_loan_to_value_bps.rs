use crate::error::UxdError;
use anchor_lang::prelude::*;

pub fn validate_loan_to_value_bps(
    loan_to_value_bps: u8,
    depository_loan_to_value_bps: u8,
) -> Result<()> {
    require!(loan_to_value_bps > 0, UxdError::InvalidLtvBps);
    require!(
        loan_to_value_bps <= depository_loan_to_value_bps,
        UxdError::LtvBpsOverLimit
    );
    Ok(())
}
