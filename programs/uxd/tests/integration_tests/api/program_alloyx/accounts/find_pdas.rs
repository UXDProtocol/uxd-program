use solana_sdk::pubkey::Pubkey;

const VAULT_INFO_SEED: &[u8] = b"vault_info";
const USDC_TOKEN_SEED: &[u8] = b"usdc_token";
const ALLOYX_TOKEN_SEED: &[u8] = b"alloyx_token";
const PASS_SEED: &[u8] = b"pass";

pub fn find_vault_id() -> String {
    String::from("uxd-debug")
}

pub fn find_vault_info(vault_id: &String) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[vault_id.as_bytes(), VAULT_INFO_SEED], &alloyx_cpi::ID)
}

pub fn find_vault_usdc_token(vault_id: &String) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[vault_id.as_bytes(), USDC_TOKEN_SEED], &alloyx_cpi::ID)
}

pub fn find_vault_alloyx_token(vault_id: &String) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[vault_id.as_bytes(), ALLOYX_TOKEN_SEED], &alloyx_cpi::ID)
}

pub fn find_investor_pass(vault_id: &String, investor: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[vault_id.as_bytes(), PASS_SEED, investor.as_ref()],
        &alloyx_cpi::ID,
    )
}
