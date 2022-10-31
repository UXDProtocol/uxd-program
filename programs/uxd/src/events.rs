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
