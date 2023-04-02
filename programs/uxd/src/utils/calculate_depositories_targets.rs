use anchor_lang::prelude::Result;

pub struct DepositoriesTargets {
    pub identity_depository_target_amount: u64,
    pub mercurial_vault_depository_0_target_amount: u64,
    pub credix_lp_depository_0_target_amount: u64,
}

pub fn calculate_depositories_targets(
    identity_depository_weight: u16,
    identity_depository_redeemable_amount_under_management: u128,
    identity_depository_redeemable_amount_under_management_cap: u128,
    mercurial_vault_depository_0_weight: u16,
    mercurial_vault_depository_0_redeemable_amount_under_management: u128,
    mercurial_vault_depository_0_redeemable_amount_under_management_cap: u128,
    credix_lp_depository_0_weight: u16,
    credix_lp_depository_0_redeemable_amount_under_management: u128,
    credix_lp_depository_0_redeemable_amount_under_management_cap: u128,
) -> Result<DepositoriesTargets> {
}
