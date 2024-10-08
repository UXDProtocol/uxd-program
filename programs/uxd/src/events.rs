use anchor_lang::prelude::*;

// - Global Events ------------------------------------------------------------

/// Event called in [instructions::initialize_controller::handler].
#[event]
pub struct InitializeControllerEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller being created.
    #[index]
    pub controller: Pubkey,
    /// The authority.
    pub authority: Pubkey,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetRedeemableGlobalSupplyCapEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The new cap.
    pub redeemable_global_supply_cap: u128,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetOutflowLimitPerEpochAmountEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The flat amount redeemable per epoch
    pub outflow_limit_per_epoch_amount: u64,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetOutflowLimitPerEpochBpsEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The portion of supply redeemable per epoch
    pub outflow_limit_per_epoch_bps: u16,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetSlotsPerEpochEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// How many slot for an epoch
    pub slots_per_epoch: u64,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetRouterDepositoriesWeightBps {
    #[index]
    pub controller_version: u8,
    #[index]
    pub controller: Pubkey,
    /// The new weights
    pub identity_depository_weight_bps: u16,
    pub mercurial_vault_depository_weight_bps: u16,
    pub credix_lp_depository_weight_bps: u16,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetRouterDepositories {
    #[index]
    pub controller_version: u8,
    #[index]
    pub controller: Pubkey,
    /// The new addresses
    pub identity_depository: Pubkey,
    pub mercurial_vault_depository: Pubkey,
    pub credix_lp_depository: Pubkey,
}

/// Event called in [instructions::register_mercurial_vault_depository::handler].
#[event]
pub struct RegisterMercurialVaultDepositoryEvent {
    #[index]
    pub version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    pub mercurial_vault: Pubkey,
    pub depository_lp_token_vault: Pubkey,
    pub collateral_mint: Pubkey,
}

/// Event called in [instructions::edit_*_depository::handler].
#[event]
pub struct SetDepositoryRedeemableAmountUnderManagementCapEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub redeemable_amount_under_management_cap: u128,
}

/// Event called in [instructions::edit_*_depository::handler].
#[event]
pub struct SetDepositoryMintingFeeInBpsEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub minting_fee_in_bps: u8,
}

/// Event called in [instructions::edit_*_depository::handler].
#[event]
pub struct SetDepositoryRedeemingFeeInBpsEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub redeeming_fee_in_bps: u8,
}

/// Event called in [instructions::edit_*_depository::handler].
#[event]
pub struct SetDepositoryMintingDisabledEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub minting_disabled: bool,
}

/// Event called in [instructions::edit_*_depository::handler].
#[event]
pub struct SetDepositoryProfitsBeneficiaryCollateralEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub profits_beneficiary_collateral: Pubkey,
}

/// Event called in [instructions::initialize_identity_depository::handler].
#[event]
pub struct InitializeIdentityDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    pub collateral_mint: Pubkey,
}

/// Event called in [instructions::mint_with_identity_depository::handler].
#[event]
pub struct MintWithIdentityDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The collateral amount in native units. (input)
    pub collateral_amount: u64,
}

/// Event called in [instructions::redeem_from_identity_depository::handler].
#[event]
pub struct RedeemFromIdentityDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The redeemable amount in native units. (input)
    pub redeemable_amount: u64,
}

/// Event called in [instructions::register_credix_lp_depository::handler].
#[event]
pub struct RegisterCredixLpDepositoryEvent {
    #[index]
    pub controller_version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    pub collateral_mint: Pubkey,
    pub credix_global_market_state: Pubkey,
}

/// Event called in [instructions::mint_with_credix_lp_depository::handler].
#[event]
pub struct MintWithCredixLpDepositoryEvent {
    #[index]
    pub controller_version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub user: Pubkey,
    /// The collateral amount in native units. (input)
    pub collateral_amount: u64,
    /// The redeemable issued in native units. (output)
    pub redeemable_amount: u64,
    /// The fees paid in native units.
    pub minting_fee_paid: u64,
}

/// Event called in [instructions::redeem_from_credix_lp_depository::handler].
#[event]
pub struct RedeemFromCredixLpDepositoryEvent {
    #[index]
    pub controller_version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub user: Pubkey,
    /// The collateral amount in native units. (output)
    pub collateral_amount: u64,
    /// The redeemable issued in native units. (input)
    pub redeemable_amount: u64,
    /// The fees paid in native units.
    pub redeeming_fee_paid: u64,
}

/// Event called in [instructions::collect_profits_of_credix_lp_depository::handler].
#[event]
pub struct CollectProfitsOfCredixLpDepositoryEvent {
    #[index]
    pub controller_version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    /// The collateral amount in native units. (output)
    pub collateral_amount: u64,
}

/// Event called in [instructions::exchange_liquidity_with_credix_lp_depository::handler].
#[event]
pub struct ExchangeLiquidityWithCredixLpDepositoryEvent {
    #[index]
    pub controller_version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    /// The collateral amount in native units. (input)
    pub collateral_amount: u64,
    /// The shares amount in native units. (output)
    pub shares_amount: u64,
}

/// Event called in [instructions::rebalance_create_withdraw_request_from_credix_lp_depository::handler].
#[event]
pub struct RebalanceCreateWithdrawRequestFromCredixLpDepositoryEvent {
    #[index]
    pub controller_version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    /// The redeemable amount rebalanced in native units. (output)
    pub overflow_value: u64,
    /// The collateral amount of profits collected in native units. (output)
    pub profits_collateral_amount: u64,
    /// The total amount requested in the credix withdrawal
    pub requested_collateral_amount: u64,
}

/// Event called in [instructions::rebalance_redeem_withdraw_request_from_credix_lp_depository::handler].
#[event]
pub struct RebalanceRedeemWithdrawRequestFromCredixLpDepositoryEvent {
    #[index]
    pub controller_version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    /// The redeemable amount rebalanced in native units. (output)
    pub overflow_value: u64,
    /// The collateral amount of profits collected in native units. (output)
    pub profits_collateral_amount: u64,
    /// The total amount requested in the credix withdrawal
    pub requested_collateral_amount: u64,
}

/// Event called in [instructions::collect_profit_of_mercurial_vault_depository::handler].
#[event]
pub struct CollectProfitsOfMercurialVaultDepositoryEvent {
    #[index]
    pub controller_version: u8,
    #[index]
    pub depository_version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    /// The collateral amount in native units. (output)
    pub collateral_amount: u64,
}

/// Event called in [instructions::freeze_program::handler].
#[event]
pub struct FreezeProgramEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// is program frozen
    pub is_frozen: bool,
}
