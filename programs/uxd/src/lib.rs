use crate::instructions::*;
use crate::state::*;
use anchor_lang::prelude::*;
use error::UxdError;
#[macro_use]
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod test;
pub mod utils;

// CI Uses F3UToS4WKQkyAAs5TwM_21ANq2xNfDRB7tGRWx4DxapaR on Devnet
// (it's auto swapped by the script, keypair are held in target/deployment)
#[cfg(feature = "development")]
solana_program::declare_id!("36N7GZDMv3xkabgWMc2fJqrxm15cgp6oykPs53qDn5Ju");
#[cfg(feature = "production")]
solana_program::declare_id!("UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr");

// Version used for accounts structure and future migrations
pub const MERCURIAL_VAULT_DEPOSITORY_ACCOUNT_VERSION: u8 = 1;
pub const CONTROLLER_ACCOUNT_VERSION: u8 = 1;
pub const IDENTITY_DEPOSITORY_ACCOUNT_VERSION: u8 = 1;
pub const CREDIX_LP_DEPOSITORY_ACCOUNT_VERSION: u8 = 1;

// These are just "namespaces" seeds for the PDA creations.
pub const REDEEMABLE_MINT_NAMESPACE: &[u8] = b"REDEEMABLE";
pub const CONTROLLER_NAMESPACE: &[u8] = b"CONTROLLER";
pub const MERCURIAL_VAULT_DEPOSITORY_NAMESPACE: &[u8] = b"MERCURIALVAULTDEPOSITORY";
pub const MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE: &[u8] =
    b"MERCURIALVAULTDEPOSITORYLPVAULT";
pub const IDENTITY_DEPOSITORY_NAMESPACE: &[u8] = b"IDENTITYDEPOSITORY";
pub const IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE: &[u8] = b"IDENTITYDEPOSITORYCOLLATERAL";

pub const CREDIX_LP_DEPOSITORY_NAMESPACE: &[u8] = b"CREDIX_LP_DEPOSITORY";

pub const MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = u128::MAX;
pub const DEFAULT_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = 1_000_000; // 1 Million redeemable UI units
pub const DEFAULT_REDEEMABLE_UNDER_MANAGEMENT_CAP: u128 = 1_000_000; // 1 Million redeemable UI units

const BPS_POW: u8 = 4; // Raise a number to BPS_POW to get order of magnitude of
pub const BPS_UNIT_CONVERSION: u64 = (10u64).pow(BPS_POW as u32);

const SOLANA_MAX_MINT_DECIMALS: u8 = 9;

#[program]
#[deny(unused_must_use)]
pub mod uxd {

    use super::*;

    /// Initialize a Controller on chain account.
    ///
    /// Parameters:
    ///     - redeemable_mint_decimals: the decimals of the redeemable mint.
    ///
    /// Note:
    ///  Only one Controller on chain account will ever exist due to the
    ///  PDA derivation seed having no variations.
    ///
    /// Note:
    ///  In the case of UXDProtocol this is the one in charge of the UXD mint,
    ///  and it has been locked to a single Controller to ever exist by only
    ///  having one possible derivation. (but it's been made generic, and we
    ///  could have added the authority to the seed generation for instance).
    ///
    #[access_control(ctx.accounts.validate(redeemable_mint_decimals))]
    pub fn initialize_controller(
        ctx: Context<InitializeController>,
        redeemable_mint_decimals: u8,
    ) -> Result<()> {
        msg!("[initialize_controller]");
        instructions::initialize_controller::handler(ctx, redeemable_mint_decimals)
    }

    #[access_control(ctx.accounts.validate(&fields))]
    pub fn edit_controller(
        ctx: Context<EditController>,
        fields: EditControllerFields,
    ) -> Result<()> {
        instructions::edit_controller::handler(ctx, &fields)
    }

    pub fn edit_mercurial_vault_depository(
        ctx: Context<EditMercurialVaultDepository>,
        fields: EditMercurialVaultDepositoryFields,
    ) -> Result<()> {
        instructions::edit_mercurial_vault_depository::handler(ctx, &fields)
    }

    pub fn edit_identity_depository(
        ctx: Context<EditIdentityDepository>,
        fields: EditIdentityDepositoryFields,
    ) -> Result<()> {
        instructions::edit_identity_depository::handler(ctx, &fields)
    }

    pub fn edit_credix_lp_depository(
        ctx: Context<EditCredixLpDepository>,
        fields: EditCredixLpDepositoryFields,
    ) -> Result<()> {
        instructions::edit_credix_lp_depository::handler(ctx, &fields)
    }

    // Mint Redeemable tokens by depositing Collateral to mercurial vault.
    #[access_control(
        ctx.accounts.validate(collateral_amount)
    )]
    pub fn mint_with_mercurial_vault_depository(
        ctx: Context<MintWithMercurialVaultDepository>,
        collateral_amount: u64,
    ) -> Result<()> {
        msg!("[mint_with_mercurial_vault_depository]");
        instructions::mint_with_mercurial_vault_depository::handler(ctx, collateral_amount)
    }

    // Create and Register a new `MercurialVaultDepository` to the `Controller`.
    // Each `Depository` account manages a specific collateral mint.
    #[access_control(
        ctx.accounts.validate(minting_fee_in_bps, redeeming_fee_in_bps, redeemable_amount_under_management_cap)
    )]
    pub fn register_mercurial_vault_depository(
        ctx: Context<RegisterMercurialVaultDepository>,
        minting_fee_in_bps: u8,
        redeeming_fee_in_bps: u8,
        redeemable_amount_under_management_cap: u128,
    ) -> Result<()> {
        msg!("[register_mercurial_vault_depository]");
        instructions::register_mercurial_vault_depository::handler(
            ctx,
            minting_fee_in_bps,
            redeeming_fee_in_bps,
            redeemable_amount_under_management_cap,
        )
    }

    #[access_control(
        ctx.accounts.validate(redeemable_amount)
    )]
    pub fn redeem_from_mercurial_vault_depository(
        ctx: Context<RedeemFromMercurialVaultDepository>,
        redeemable_amount: u64,
    ) -> Result<()> {
        msg!("[redeem_from_mercurial_vault]");
        instructions::redeem_from_mercurial_vault_depository::handler(ctx, redeemable_amount)
    }

    #[access_control(ctx.accounts.validate())]
    pub fn initialize_identity_depository(
        ctx: Context<InitializeIdentityDepository>,
    ) -> Result<()> {
        msg!("[initialize_identity_depository]");
        instructions::initialize_identity_depository::handler(ctx)
    }

    #[access_control(
        ctx.accounts.validate(collateral_amount)
    )]
    pub fn mint_with_identity_depository(
        ctx: Context<MintWithIdentityDepository>,
        collateral_amount: u64,
    ) -> Result<()> {
        msg!(
            "[mint_with_identity_depository] collateral_amount {}",
            collateral_amount,
        );
        instructions::mint_with_identity_depository::handler(ctx, collateral_amount)
    }

    #[access_control(
        ctx.accounts.validate(redeemable_amount)
    )]
    pub fn redeem_from_identity_depository(
        ctx: Context<RedeemFromIdentityDepository>,
        redeemable_amount: u64,
    ) -> Result<()> {
        msg!(
            "[redeem_from_identity_depository] redeemable_amount {}",
            redeemable_amount,
        );
        instructions::redeem_from_identity_depository::handler(ctx, redeemable_amount)
    }

    // Create and Register a new `CredixLpDepository` to the `Controller`.
    // Each `Depository` account manages a specific credix lp.
    #[access_control(
        ctx.accounts.validate(
            minting_fee_in_bps,
            redeeming_fee_in_bps,
            redeemable_amount_under_management_cap,
        )
    )]
    pub fn register_credix_lp_depository(
        ctx: Context<RegisterCredixLpDepository>,
        minting_fee_in_bps: u8,
        redeeming_fee_in_bps: u8,
        redeemable_amount_under_management_cap: u128,
    ) -> Result<()> {
        msg!("[register_credix_lp_depository]");
        instructions::register_credix_lp_depository::handler(
            ctx,
            minting_fee_in_bps,
            redeeming_fee_in_bps,
            redeemable_amount_under_management_cap,
        )
    }

    // Mint Redeemable tokens by depositing Collateral to credix lp.
    #[access_control(
        ctx.accounts.validate(collateral_amount)
    )]
    pub fn mint_with_credix_lp_depository(
        ctx: Context<MintWithCredixLpDepository>,
        collateral_amount: u64,
    ) -> Result<()> {
        msg!("[mint_with_credix_lp_depository]");
        instructions::mint_with_credix_lp_depository::handler(ctx, collateral_amount)
    }

    // Redeem collateral tokens by burning redeemable from credix lp.
    #[access_control(
        ctx.accounts.validate(redeemable_amount)
    )]
    pub fn redeem_from_credix_lp_depository(
        ctx: Context<RedeemFromCredixLpDepository>,
        redeemable_amount: u64,
    ) -> Result<()> {
        msg!("[redeem_from_credix_lp_depository]");
        instructions::redeem_from_credix_lp_depository::handler(ctx, redeemable_amount)
    }

    // Collect collateral tokens when locked value exceed liabilities (profit).
    #[access_control(
        ctx.accounts.validate()
    )]
    pub fn collect_profit_of_credix_lp_depository(
        ctx: Context<CollectProfitOfCredixLpDepository>,
    ) -> Result<()> {
        msg!("[collect_profit_of_credix_lp_depository]");
        instructions::collect_profit_of_credix_lp_depository::handler(ctx)
    }
}
