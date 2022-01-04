use crate::*;

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
