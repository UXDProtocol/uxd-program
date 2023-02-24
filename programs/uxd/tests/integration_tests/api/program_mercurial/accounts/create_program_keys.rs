use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub struct ProgramKeys {
    pub authority: Keypair,
    pub vault: Pubkey,
    pub vault_lp_mint: Pubkey,
}

pub fn create_program_keys(collateral_mint: &Pubkey) -> ProgramKeys {
    let authority = Keypair::new();

    let vault = Pubkey::default();
    let vault_lp_mint = Pubkey::default();

    // TODO - implement correct account finding

    ProgramKeys {
        authority,
        vault,
        vault_lp_mint,
    }
}
