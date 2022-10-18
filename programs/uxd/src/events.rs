use crate::instructions::rebalance_mango_depository_lite::PnlPolarity;
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

/// Event called in [instructions::set_redeemable_global_supply_cap::handler].
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

/// Deprecated, use `RegisterMangoDepositoryEventV2` - Keep for decoding on chain history
/// Event called in [instructions::register_mango_depository::handler].
#[event]
pub struct RegisterMangoDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The collateral mint.
    pub collateral_mint: Pubkey,
    /// The insurance mint.
    pub insurance_mint: Pubkey,
    /// The MangoAccount PDA.
    pub mango_account: Pubkey,
}

/// Event called in [instructions::register_mango_depository::handler].
#[event]
pub struct RegisterMangoDepositoryEventV2 {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The depository version.
    #[index]
    pub depository_version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The collateral mint.
    pub collateral_mint: Pubkey,
    /// The quote mint.
    pub quote_mint: Pubkey,
    /// The MangoAccount PDA.
    pub mango_account: Pubkey,
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

/// Event called in [instructions::set_mango_depository_redeemable_soft_cap::handler].
#[event]
pub struct SetMangoDepositoryRedeemableSoftCapEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The redeemable mint.
    pub redeemable_mint: Pubkey,
    /// The redeemable mint decimals.
    pub redeemable_mint_decimals: u8,
    /// The new cap.
    pub redeemable_soft_cap: u64,
}

/// Event called in [instructions::*::deposit_insurance_to_*_depository::handler].
#[event]
pub struct DepositInsuranceToDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The insurance mint.
    pub quote_mint: Pubkey,
    /// The insurance mint decimals.
    pub quote_mint_decimals: u8,
    /// The deposited amount in native units.
    pub deposited_amount: u64,
}

/// Event called in [instructions::mango_dex::withdraw_insurance_from_mango_depository::handler].
#[event]
pub struct WithdrawInsuranceFromMangoDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The insurance mint.
    pub insurance_mint: Pubkey,
    /// The insurance mint decimals.
    pub insurance_mint_decimals: u8,
    /// The withdrawn amount in native units.
    pub withdrawn_amount: u64,
}

/// Event called in [instructions::*_dex::withdraw_insurance_from_*_depository::handler].
#[event]
pub struct WithdrawInsuranceFromDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The insurance mint.
    pub quote_mint: Pubkey,
    /// The insurance mint decimals.
    pub quote_mint_decimals: u8,
    /// The withdrawn amount in native units.
    pub withdrawn_amount: u64,
}

/// Event called in [instructions::mango_dex::mint_with_mango_depository::handler].
#[event]
pub struct MintWithMangoDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The collateral amount in native units. (input)
    pub collateral_amount: u64,
    /// The user provided limit_price. (input)
    pub limit_price: f32,
    /// The different deltas after successful minting operation.
    pub base_delta: i64,
    pub quote_delta: i64,
    pub fee_delta: i64,
}

/// Event called in [instructions::mango_dex::redeem_from_mango_depository::handler].
#[event]
pub struct RedeemFromMangoDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The redeemable amount in native units. (input)
    pub redeemable_amount: u64,
    /// The user provided limit_price. (input)
    pub limit_price: f32,
    /// The different deltas after successful minting operation.
    pub base_delta: i64,
    pub quote_delta: i64,
    pub fee_delta: i64,
}

/// Event called in [instructions::rebalance_mango_depository_lite::handler].
#[event]
pub struct RebalanceMangoDepositoryLiteEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The depository version.
    #[index]
    pub depository_version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The polarity of the rebalancing operation. (input)
    pub polarity: PnlPolarity,
    /// The desired rebalancing amount in Quote native units. (input)
    pub rebalancing_amount: u64,
    /// The actual rebalancing amount in Quote native units.
    pub rebalanced_amount: u64,
    /// The user provided limit_price. (input)
    pub limit_price: f32,
    /// The different deltas after successful rebalancing operation.
    pub base_delta: i64,
    pub quote_delta: i64,
    pub fee_delta: i64,
}

/// Event called in [instructions::edit_mango_depository::handler].
#[event]
pub struct SetMangoDepositoryQuoteMintAndRedeemSoftCapEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    pub quote_mint_and_redeem_soft_cap: u64,
}

/// Event called in [instructions::edit_mango_depository::handler].
#[event]
pub struct SetMangoDepositoryQuoteMintAndRedeemFeeEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub quote_mint_and_redeem_fee: u8,
}

/// Event called in [instructions::edit_mango_depository::handler].
#[event]
pub struct SetMangoDepositoryRedeemableAmountUnderManagementCapEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub redeemable_amount_under_management_cap: u128,
}

/// Event called in [instructions::edit_mercurial_vault_depository::handler].
#[event]
pub struct SetMercurialVaultDepositoryRedeemableAmountUnderManagementCapEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub redeemable_amount_under_management_cap: u128,
}

/// Event called in [instructions::edit_mercurial_vault_depository::handler].
#[event]
pub struct SetMercurialVaultDepositoryMintingFeeInBpsEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub minting_fee_in_bps: u8,
}

/// Event called in [instructions::edit_mercurial_vault_depository::handler].
#[event]
pub struct SetMercurialVaultDepositoryRedeemingFeeInBpsEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub redeeming_fee_in_bps: u8,
}

/// Event called in [instructions::edit_mercurial_vault_depository::handler].
#[event]
pub struct SetMercurialVaultDepositoryMintingDisabledEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub minting_disabled: bool,
}

/// Event called in [instructions::mango_dex::quote_redeem_from_mango_depository::handler].
#[event]
pub struct QuoteRedeemFromMangoDepositoryEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub user: Pubkey,
    pub quote_redeemable_amount: u64,
    pub quote_redeem_fee: u64,
}

/// Event called in [instructions::mango_dex::quote_mint_with_mango_depository::handler].
#[event]
pub struct QuoteMintWithMangoDepositoryEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    #[index]
    pub user: Pubkey,
    pub quote_mint_amount: u64,
    pub quote_mint_fee: u64,
}

/// Event called in [instructions::*_dex::disable_depository_regular_minting::handler].
#[event]
pub struct DisableDepositoryRegularMintingEvent {
    #[index]
    pub version: u8,
    #[index]
    pub controller: Pubkey,
    #[index]
    pub depository: Pubkey,
    pub regular_minting_disabled: bool,
}

/// Event called in [instructions::initialize_identity_depository::handler].
#[event]
pub struct InitializeIdentityDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The depository version.
    #[index]
    pub depository_version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The collateral mint.
    pub collateral_mint: Pubkey,
}

/// Event called in [instructions::mint_with_identity_depository::handler].
#[event]
pub struct MintWithIdentityDepositoryEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
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
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The redeemable amount in native units. (input)
    pub redeemable_amount: u64,
}
