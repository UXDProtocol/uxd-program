use super::calculate_depositories_sum_value;
use super::calculate_depositories_target_redeemable_amount;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::utils::checked_convert_u128_to_u64;
use crate::utils::DepositoryInfoForTargetRedeemableAmount;
use anchor_lang::prelude::*;

pub fn calculate_credix_lp_depository_target_amount(
    controller: &AccountLoader<Controller>,
    identity_depository: &AccountLoader<IdentityDepository>,
    mercurial_vault_depository: &AccountLoader<MercurialVaultDepository>,
    credix_lp_depository: &AccountLoader<CredixLpDepository>,
) -> Result<u64> {
    let controller = controller.load()?;
    let total_redeemable_amount_under_management = calculate_depositories_sum_value(&vec![
        checked_convert_u128_to_u64(
            identity_depository
                .load()?
                .redeemable_amount_under_management,
        )?,
        checked_convert_u128_to_u64(
            mercurial_vault_depository
                .load()?
                .redeemable_amount_under_management,
        )?,
        checked_convert_u128_to_u64(
            credix_lp_depository
                .load()?
                .redeemable_amount_under_management,
        )?,
    ])?;
    let depositories_target_redeemable_amount = calculate_depositories_target_redeemable_amount(
        total_redeemable_amount_under_management,
        &vec![
            // credix is the first in the list
            DepositoryInfoForTargetRedeemableAmount {
                weight_bps: controller.credix_lp_depository_weight_bps,
                redeemable_amount_under_management_cap: credix_lp_depository
                    .load()?
                    .redeemable_amount_under_management_cap,
            },
            DepositoryInfoForTargetRedeemableAmount {
                weight_bps: controller.identity_depository_weight_bps,
                redeemable_amount_under_management_cap: identity_depository
                    .load()?
                    .redeemable_amount_under_management_cap,
            },
            DepositoryInfoForTargetRedeemableAmount {
                weight_bps: controller.mercurial_vault_depository_weight_bps,
                redeemable_amount_under_management_cap: mercurial_vault_depository
                    .load()?
                    .redeemable_amount_under_management_cap,
            },
        ],
    )?;
    drop(controller);
    Ok(depositories_target_redeemable_amount[0]) // credix is the first in the list
}
