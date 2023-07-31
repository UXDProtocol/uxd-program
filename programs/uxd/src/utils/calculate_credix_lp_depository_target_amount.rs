use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::utils::DepositoryInfoForTargetRedeemableAmount;
use anchor_lang::prelude::*;

use super::calculate_depositories_target_redeemable_amount;

pub fn calculate_credix_lp_depository_target_amount(
    controller: &AccountLoader<Controller>,
    identity_depository: &AccountLoader<IdentityDepository>,
    mercurial_vault_depository: &AccountLoader<MercurialVaultDepository>,
    credix_lp_depository_marketplace: &AccountLoader<CredixLpDepository>,
    credix_lp_depository_receivables: &AccountLoader<CredixLpDepository>,
) -> Result<u64> {
    let controller = controller.load()?;
    let depositories_target_redeemable_amount = calculate_depositories_target_redeemable_amount(
        controller.redeemable_circulating_supply,
        &vec![
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
            DepositoryInfoForTargetRedeemableAmount {
                weight_bps: controller.mercurial_vault_depository_weight_bps,
                redeemable_amount_under_management_cap: mercurial_vault_depository
                    .load()?
                    .redeemable_amount_under_management_cap,
            },
            // credix_lp_depository_marketplace is the 4th in the list
            DepositoryInfoForTargetRedeemableAmount {
                weight_bps: controller.credix_lp_depository_marketplace_weight_bps,
                redeemable_amount_under_management_cap: credix_lp_depository_marketplace
                    .load()?
                    .redeemable_amount_under_management_cap,
            },
            DepositoryInfoForTargetRedeemableAmount {
                weight_bps: controller.credix_lp_depository_receivables_weight_bps,
                redeemable_amount_under_management_cap: credix_lp_depository_receivables
                    .load()?
                    .redeemable_amount_under_management_cap,
            },
        ],
    )?;
    drop(controller);
    Ok(depositories_target_redeemable_amount[3]) // credix is the 4th in the list
}
