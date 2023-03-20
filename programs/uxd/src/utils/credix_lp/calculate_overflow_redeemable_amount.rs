use crate::error::UxdError;
use crate::state::Controller;
use crate::state::CredixLpDepository;
use crate::utils::checked_convert_u128_to_u64;
use anchor_lang::prelude::AccountLoader;
use anchor_lang::prelude::Result;

pub fn calculate_overflow_redeemable_amount(
    controller: &AccountLoader<Controller>,
    depository: &AccountLoader<CredixLpDepository>,
) -> Result<u64> {
    // TODO - use weights properly from step 1
    // We fetch the target amount of redeemable that we wish the depository have
    let redeemable_amount_under_management_target_amount =
        checked_convert_u128_to_u64(controller.load()?.redeemable_circulating_supply / 2)?;
    // We fetch the current amounf of redeemable the depository have
    let redeemable_amount_under_management =
        checked_convert_u128_to_u64(depository.load()?.redeemable_amount_under_management)?;
    // We substract the current amount from the target amounnt to find how much needs to be withdrawn
    Ok(redeemable_amount_under_management_target_amount
        .checked_sub(redeemable_amount_under_management)
        .ok_or(UxdError::MathError)?)
}
