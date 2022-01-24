use anchor_lang::prelude::*;

// - Global Events ------------------------------------------------------------

/// Event called in [instructions::initialize_controller::handler].
#[event]
pub struct InitializeControllerEvent {
    /// The program version.
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
    /// The program version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    // The new cap.
    pub redeemable_global_supply_cap: u128,
}

/// Event called in [instructions::set_mango_depository_redeemable_soft_cap::handler].
#[event]
pub struct RegisterMangoDepositoryEvent {
    /// The program version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The depository.
    #[index]
    pub depository: Pubkey,
    // The collateral mint.
    pub collateral_mint: Pubkey,
    // The insurance mint.
    pub insurance_mint: Pubkey,
    // The MangoAccount PDA.
    pub mango_account: Pubkey,
}

/// Event called in [instructions::set_mango_depository_redeemable_soft_cap::handler].
#[event]
pub struct SetMangoDepositoryRedeemableSoftCapEvent {
    /// The program version.
    #[index]
    pub version: u8,
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
    /// The program version.
    #[index]
    pub version: u8,
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
pub struct WithdrawInsuranceFromMangoDepositoryEvent {
    /// The program version.
    #[index]
    pub version: u8,
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
    /// The program version.
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
    // The collateral amount in native units.
    pub collateral_amount: u64,
    // The user selected slippage.
    pub slippage: u32,
    // The different deltas after successful minting operation.
    pub collateral_delta: u64,
    pub redeemable_delta: u64,
    pub fee_delta: u64,
}

/// Event called in [instructions::mango_dex::redeem_from_mango_depository::handler].
#[event]
pub struct RedeemFromMangoDepositoryEvent {
    /// The program version.
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
    // The redeemable amount in native units.
    pub redeemable_amount: u64,
    // The user selected slippage.
    pub slippage: u32,
    // The different deltas after successful minting operation.
    pub collateral_delta: u64,
    pub redeemable_delta: u64,
    pub fee_delta: u64,
}
