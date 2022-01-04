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
