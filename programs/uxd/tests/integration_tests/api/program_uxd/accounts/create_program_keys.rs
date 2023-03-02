use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_uxd;

pub struct ProgramKeys {
    pub collateral_mint: Keypair,
    pub collateral_mint_decimals: u8,
    pub collateral_mint_authority: Keypair,
    pub authority: Keypair,
    pub controller: Pubkey,
    pub redeemable_mint: Pubkey,
    pub redeemable_mint_decimals: u8,
    pub identity_depository_keys: program_uxd::accounts::IdentityDepositoryKeys,
    pub mercurial_vault_depository_keys: program_uxd::accounts::MercurialVaultDepositoryKeys,
    pub credix_lp_depository_keys: program_uxd::accounts::CredixLpDepositoryKeys,
}

impl ProgramKeys {
    pub fn collateral_amount_ui_to_native(&self, ui_amount: u64) -> u64 {
        ui_amount * 10u64.pow(self.collateral_mint_decimals.into())
    }
    pub fn redeemable_amount_ui_to_native(&self, ui_amount: u64) -> u64 {
        ui_amount * 10u64.pow(self.redeemable_mint_decimals.into())
    }
}

pub fn create_program_keys() -> ProgramKeys {
    let authority = Keypair::new();

    let collateral_mint = Keypair::new();
    let collateral_mint_decimals = 6;
    let collateral_mint_authority = Keypair::new();

    let controller =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id()).0;

    let redeemable_mint =
        Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE.as_ref()], &uxd::id()).0;
    let redeemable_mint_decimals = 6;

    let identity_depository_keys = program_uxd::accounts::create_identity_depository_keys();

    let mercurial_vault_depository_keys =
        program_uxd::accounts::create_mercurial_vault_depository_keys(&collateral_mint.pubkey());

    let credix_lp_depository_keys =
        program_uxd::accounts::create_credix_lp_depository_keys(&collateral_mint.pubkey());

    ProgramKeys {
        collateral_mint,
        collateral_mint_decimals,
        collateral_mint_authority,
        authority,
        controller,
        redeemable_mint,
        redeemable_mint_decimals,
        identity_depository_keys,
        mercurial_vault_depository_keys,
        credix_lp_depository_keys,
    }
}
