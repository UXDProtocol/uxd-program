use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub struct ProgramKeys {
    pub authority: Keypair,
    pub admin: Keypair,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub lp_mint: Keypair,
    pub lp_mint_decimals: u8,
}

pub fn create_program_keys(collateral_mint: &Pubkey) -> ProgramKeys {
    let authority = Keypair::new();
    let admin = Keypair::new();

    let token_mint = *collateral_mint;

    let vault = mercurial_vault::utils::derive_vault_address(token_mint, authority.pubkey()).0;

    let token_vault = Pubkey::find_program_address(
        &[
            mercurial_vault::seed::TOKEN_VAULT_PREFIX.as_ref(),
            vault.as_ref(),
        ],
        &mercurial_vault::ID,
    )
    .0;

    let fee_vault =
        spl_associated_token_account::get_associated_token_address(&admin.pubkey(), &token_mint);

    let lp_mint = Keypair::new();
    let lp_mint_decimals = 6;

    /*
    let fee_vault = Pubkey::find_program_address(
        &[
            mercurial_vault::seed::FEE_VAULT_PREFIX.as_ref(),
            vault.as_ref(),
        ],
        &mercurial_vault::ID,
    )
    .0;
     */

    // TODO - implement correct account finding

    ProgramKeys {
        authority,
        admin,
        token_mint,
        vault,
        token_vault,
        fee_vault,
        lp_mint,
        lp_mint_decimals,
    }
}
