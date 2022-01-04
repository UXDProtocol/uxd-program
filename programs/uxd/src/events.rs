use crate::mango_utils::OrderDelta;
use anchor_lang::prelude::*;

// - Global Events ------------------------------------------------------------

/// Event called in [instructions::initialize_controller::handler].
#[event]
pub struct InitializeControllerEvent {
    /// The controller being created.
    #[index]
    pub controller: Pubkey,
    /// The authority.
    pub authority: Pubkey,
}

/// Event called in [instructions::set_redeemable_global_supply_cap::handler].
#[event]
pub struct SetRedeemableGlobalSupplyCapEvent {
    /// The controller.
    #[index]
    pub controller: Pubkey,
    // The new cap.
    pub redeemable_global_supply_cap: u128,
}

/// Event called in [instructions::set_mango_depository_redeemable_soft_cap::handler].
#[event]
pub struct SetMangoDepositoryRedeemableSoftCapEvent {
    /// The controller.
    #[index]
    pub controller: Pubkey,
    // The redeemable mint.
    pub redeemable_mint: Pubkey,
    // The redeemable mint decimals.
    pub redeemable_mint_decimals: u8,
    // The new cap.
    pub redeemable_soft_cap: u64,
}

// - Mango Depositories -------------------------------------------------------

/// Event called in [instructions::mango_dex::deposit_insurance_to_mango_depository::handler].
#[event]
pub struct DepositInsuranceToMangoDepositoryEvent {
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    // The insurance mint.
    pub insurance_mint: Pubkey,
    // The insurance mint decimals.
    pub insurance_mint_decimals: u8,
    // The deposited amount in native units.
    pub deposited_amount: u64,
}

/// Event called in [instructions::mango_dex::withdraw_insurance_from_mango_depository::handler].
#[event]
pub struct WithdrawInsuranceFromMangoDeposirotyEvent {
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    // The insurance mint.
    pub insurance_mint: Pubkey,
    // The insurance mint decimals.
    pub insurance_mint_decimals: u8,
    // The withdrawn amount in native units.
    pub withdrawn_amount: u64,
}

/// Event called in [instructions::mango_dex::mint_with_mango_depository::handler].
#[event]
pub struct MintWithMangoDepositoryEvent {
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    // The collateral amount in native units.
    pub collateral_amount: u64,
    // The user selected slippage.
    pub slippage: u32,
    // The different deltas after successful minting operation.
    pub order_delta: OrderDelta,
}
