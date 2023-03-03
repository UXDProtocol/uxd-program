use solana_program::pubkey::Pubkey;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;

pub struct MercurialVaultDepositoryKeys {
    pub depository: Pubkey,
    pub depository_lp_token_vault: Pubkey,
    pub mercurial_vault: Pubkey,
    pub mercurial_vault_lp_mint: Pubkey,
    pub mercurial_vault_collateral_token_safe: Pubkey,
    pub mercurial_program_keys: program_mercurial::accounts::ProgramKeys,
}

pub fn create_mercurial_vault_depository_keys(
    collateral_mint: &Pubkey,
) -> MercurialVaultDepositoryKeys {
    let mercurial_program_keys = program_mercurial::accounts::create_program_keys(collateral_mint);

    let mercurial_vault = mercurial_program_keys.vault;
    let mercurial_vault_lp_mint = mercurial_program_keys.lp_mint.pubkey();
    let mercurial_vault_collateral_token_safe = mercurial_program_keys.token_vault;

    let depository = Pubkey::find_program_address(
        &[
            uxd::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE.as_ref(),
            mercurial_vault.as_ref(),
            collateral_mint.as_ref(),
        ],
        &uxd::id(),
    )
    .0;

    let depository_lp_token_vault = Pubkey::find_program_address(
        &[
            uxd::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE.as_ref(),
            mercurial_vault.as_ref(),
            collateral_mint.as_ref(),
        ],
        &uxd::id(),
    )
    .0;

    MercurialVaultDepositoryKeys {
        depository,
        depository_lp_token_vault,
        mercurial_vault,
        mercurial_vault_lp_mint,
        mercurial_vault_collateral_token_safe,
        mercurial_program_keys,
    }
}
