use anchor_lang::prelude::*;

use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;

use super::calculate_depositories_target_redeemable_amount;
use super::DepositoryInfoForTargetRedeemableAmount;

pub struct RouterTargetRedeemableAmount {
    pub identity_depository_target_redeemable_amount: u64,
    pub mercurial_vault_depository_target_redeemable_amount: u64,
    pub credix_lp_depository_target_redeemable_amount: u64,
}

pub fn calculate_router_target_redeemable_amount(
    redeemable_circulating_supply: u128,
    controller: &AccountLoader<Controller>,
    identity_depository: &AccountLoader<IdentityDepository>,
    mercurial_vault_depository: &AccountLoader<MercurialVaultDepository>,
    credix_lp_depository: &AccountLoader<CredixLpDepository>,
) -> Result<RouterTargetRedeemableAmount> {
    let controller = controller.load()?;
    let depositories_target_redeemable_amount = calculate_depositories_target_redeemable_amount(
        redeemable_circulating_supply,
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
                weight_bps: controller.credix_lp_depository_weight_bps,
                redeemable_amount_under_management_cap: credix_lp_depository
                    .load()?
                    .redeemable_amount_under_management_cap,
            },
        ],
    )?;
    drop(controller);
    Ok(RouterTargetRedeemableAmount {
        // Identity depository is the first one of the input vector
        identity_depository_target_redeemable_amount: depositories_target_redeemable_amount[0],
        // Mercurial vault depository is the second one of the input vector
        mercurial_vault_depository_target_redeemable_amount: depositories_target_redeemable_amount
            [1],
        // Credix lp depository is the third one of the input vector
        credix_lp_depository_target_redeemable_amount: depositories_target_redeemable_amount[2],
    })
}
