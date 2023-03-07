use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

const CREDIX_MARKETPLACE_SEED: &str = "this-can-be-whatever";

pub fn find_market_seeds() -> String {
    String::from(CREDIX_MARKETPLACE_SEED)
}

pub fn find_program_state_pda() -> (Pubkey, u8) {
    credix_client::ProgramState::generate_pda()
}

pub fn find_global_market_state_pda(market_seeds: &String) -> (Pubkey, u8) {
    credix_client::GlobalMarketState::generate_pda(market_seeds)
}

pub fn find_market_admins_pda(global_market_state: &Pubkey) -> (Pubkey, u8) {
    credix_client::MarketAdmins::generate_pda(*global_market_state)
}

pub fn find_lp_token_mint_pda(market_seeds: &String) -> (Pubkey, u8) {
    credix_client::GlobalMarketState::generate_lp_token_mint_pda(market_seeds)
}

pub fn find_signing_authority_pda(market_seeds: &String) -> (Pubkey, u8) {
    credix_client::GlobalMarketState::generate_signing_authority_pda(market_seeds)
}

pub fn find_liquidity_pool_token_account(
    signing_authority: &Pubkey,
    base_token_mint: &Pubkey,
) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(signing_authority, base_token_mint)
}

pub fn find_treasury(authority: &Keypair) -> Pubkey {
    authority.pubkey()
}

pub fn find_treasury_pool_token_account(treasury: &Pubkey, base_token_mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(treasury, base_token_mint)
}

pub fn find_credix_pass_pda(global_market_state: &Pubkey, pass_holder: &Pubkey) -> (Pubkey, u8) {
    credix_client::CredixPass::generate_pda(*global_market_state, *pass_holder)
}
