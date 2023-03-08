use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

pub fn find_base() -> Keypair {
    // Hard-coding a static keypair,
    // since we only need one to create as many vault as we want:
    // - we only need it to initialize a vault
    // - we dont need any test to create a custom base
    // (no need to have 2 mercurial programs running in the same cluster)
    // Unfortunately the vault PDA depends on base, which is everywhere
    // thus using a dynamic value require us to pass it as parameters EVERYWHERE (which bloats everything)
    // Also we are above the maximum function parameter limit in many places which makes it even more difficult
    Keypair::from_base58_string(
        "5MaiiCavjCmn9Hs1o3eznqDEhRwxo7pXiAYez7keQUviUkauRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG",
    )
}

pub fn find_treasury() -> Pubkey {
    mercurial_vault::get_treasury_address()
}

pub fn find_vault_pda(token_mint: &Pubkey, base: &Pubkey) -> (Pubkey, u8) {
    mercurial_vault::utils::derive_vault_address(*token_mint, *base)
}

pub fn find_token_vault_pda(vault: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            mercurial_vault::seed::TOKEN_VAULT_PREFIX.as_ref(),
            vault.as_ref(),
        ],
        &mercurial_vault::ID,
    )
}

pub fn find_fee_vault(treasury: &Pubkey, lp_mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(treasury, lp_mint)
}
