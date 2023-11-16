use crate::state::alloyx_vault_depository::AlloyxVaultDepository;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::utils::DepositoryInfoForTargetRedeemableAmount;
use crate::ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX;
use crate::ROUTER_CREDIX_LP_DEPOSITORY_INDEX;
use crate::ROUTER_DEPOSITORIES_COUNT;
use crate::ROUTER_IDENTITY_DEPOSITORY_INDEX;
use crate::ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX;
use anchor_lang::prelude::*;

use super::calculate_depositories_target_redeemable_amount;
use super::checked_as_u64;

pub struct RouterDepositoriesTargetRedeemableAmount {
    pub identity_depository_target_redeemable_amount: u64,
    pub mercurial_vault_depository_target_redeemable_amount: u64,
    pub credix_lp_depository_target_redeemable_amount: u64,
    pub alloyx_vault_depository_target_redeemable_amount: u64,
}

pub fn calculate_router_depositories_target_redeemable_amount(
    controller: &AccountLoader<Controller>,
    identity_depository: &AccountLoader<IdentityDepository>,
    mercurial_vault_depository: &AccountLoader<MercurialVaultDepository>,
    credix_lp_depository: &AccountLoader<CredixLpDepository>,
    alloyx_vault_depository: &AccountLoader<AlloyxVaultDepository>,
) -> Result<RouterDepositoriesTargetRedeemableAmount> {
    let controller = controller.load()?;

    let mut depositories_info = vec![
        DepositoryInfoForTargetRedeemableAmount {
            weight_bps: 0,
            redeemable_amount_under_management_cap: 0,
        };
        ROUTER_DEPOSITORIES_COUNT
    ];

    depositories_info[ROUTER_IDENTITY_DEPOSITORY_INDEX] = DepositoryInfoForTargetRedeemableAmount {
        weight_bps: controller.identity_depository_weight_bps,
        redeemable_amount_under_management_cap: checked_as_u64(
            identity_depository
                .load()?
                .redeemable_amount_under_management_cap,
        )?,
    };
    depositories_info[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX] =
        DepositoryInfoForTargetRedeemableAmount {
            weight_bps: controller.mercurial_vault_depository_weight_bps,
            redeemable_amount_under_management_cap: checked_as_u64(
                mercurial_vault_depository
                    .load()?
                    .redeemable_amount_under_management_cap,
            )?,
        };
    depositories_info[ROUTER_CREDIX_LP_DEPOSITORY_INDEX] =
        DepositoryInfoForTargetRedeemableAmount {
            weight_bps: controller.credix_lp_depository_weight_bps,
            redeemable_amount_under_management_cap: checked_as_u64(
                credix_lp_depository
                    .load()?
                    .redeemable_amount_under_management_cap,
            )?,
        };
    depositories_info[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX] =
        DepositoryInfoForTargetRedeemableAmount {
            weight_bps: controller.alloyx_vault_depository_weight_bps,
            redeemable_amount_under_management_cap: alloyx_vault_depository
                .load()?
                .redeemable_amount_under_management_cap,
        };

    let depositories_target_redeemable_amount = calculate_depositories_target_redeemable_amount(
        checked_as_u64(controller.redeemable_circulating_supply)?,
        &depositories_info,
    )?;

    drop(controller);

    Ok(RouterDepositoriesTargetRedeemableAmount {
        identity_depository_target_redeemable_amount: depositories_target_redeemable_amount
            [ROUTER_IDENTITY_DEPOSITORY_INDEX],
        mercurial_vault_depository_target_redeemable_amount: depositories_target_redeemable_amount
            [ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
        credix_lp_depository_target_redeemable_amount: depositories_target_redeemable_amount
            [ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
        alloyx_vault_depository_target_redeemable_amount: depositories_target_redeemable_amount
            [ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX],
    })
}
