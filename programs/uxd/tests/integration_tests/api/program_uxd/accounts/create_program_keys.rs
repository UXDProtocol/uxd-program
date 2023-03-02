use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::integration_tests::api::program_uxd;

pub struct ProgramKeys {
    pub authority: Keypair,
    pub collateral_authority: Keypair,
    pub collateral_mint: Keypair,
    pub controller: Pubkey,
    pub redeemable_mint: Pubkey,
    pub identity_depository_keys: program_uxd::accounts::IdentityDepositoryKeys,
}

pub fn create_program_keys() -> ProgramKeys {
    let authority = Keypair::new();

    let collateral_authority = Keypair::new();
    let collateral_mint = Keypair::new();

    let controller =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id()).0;

    let redeemable_mint =
        Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE.as_ref()], &uxd::id()).0;

    let identity_depository_keys = program_uxd::accounts::create_identity_depository_keys();

    ProgramKeys {
        authority,
        controller,
        collateral_authority,
        collateral_mint,
        redeemable_mint,
        identity_depository_keys,
    }
}
