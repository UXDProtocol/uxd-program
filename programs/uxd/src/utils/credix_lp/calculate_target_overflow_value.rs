use crate::error::UxdError;
use crate::state::Controller;
use crate::state::CredixLpDepository;
use crate::utils::checked_convert_u128_to_u64;
use anchor_lang::prelude::AccountLoader;
use anchor_lang::prelude::Result;

pub fn calculate_target_overflow_value(
    controller: &AccountLoader<Controller>,
    depository: &AccountLoader<CredixLpDepository>,
    profits_value: u64,
) -> Result<u64> {
    // TODO - use weights properly from step 1
    // We fetch the target amount of redeemable that we wish the depository have
    let redeemable_amount_under_management_target_amount =
        checked_convert_u128_to_u64(controller.load()?.redeemable_circulating_supply / 2)?;

    let redeemable_amount_under_management =
        checked_convert_u128_to_u64(depository.load()?.redeemable_amount_under_management)?;

    let dudu = redeemable_amount_under_management;

    // If the depository is currently underweight, there is no overflow
    if redeemable_amount_under_management < redeemable_amount_under_management_target_amount {
        return Ok(0);
    }
    // We substract the current redeemable amount from the target redeemable amount
    // to find how much needs to be withdrawn from the depository
    Ok(redeemable_amount_under_management_target_amount
        .checked_sub(redeemable_amount_under_management)
        .ok_or(UxdError::MathError)?)
}
