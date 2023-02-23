use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_uxd;

pub struct ProgramSetup {
    pub authority: Keypair,
    pub controller: Pubkey,
    pub collateral_authority: Keypair,
    pub collateral_mint: Keypair,
    pub redeemable_mint: Pubkey,
    pub identity_depository_setup: program_uxd::accounts::IdentityDepositorySetup,
    pub credix_lp_depository_setup: program_uxd::accounts::CredixLpDepositorySetup,
}

pub fn create_program_setup() -> ProgramSetup {
    let authority = Keypair::new();

    let controller =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id()).0;

    let collateral_authority = Keypair::new();
    let collateral_mint = Keypair::new();

    let redeemable_mint =
        Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE.as_ref()], &uxd::id()).0;

    let identity_depository_setup = program_uxd::accounts::create_identity_depository_setup();

    let credix_lp_depository_setup =
        program_uxd::accounts::create_credix_lp_depository_setup(&collateral_mint.pubkey());

    ProgramSetup {
        authority,
        controller,
        collateral_authority,
        collateral_mint,
        redeemable_mint,
        identity_depository_setup,
        credix_lp_depository_setup,
    }
}
