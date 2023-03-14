use std::str::FromStr;

use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

const CREDIX_MARKETPLACE_SEED: &str = "this-can-be-whatever";

const CREDIX_WITHDRAW_EPOCH_SEED: &str = "withdraw-epoch";
const CREDIX_WITHDRAW_REQUEST_SEED: &str = "withdraw-request";

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

pub fn find_treasury() -> Pubkey {
    // The treasury used by credix, we're not supposed to have access to it.
    Pubkey::from_str("4u825MpPxsRSxnvAJ8jJRsvAtbXByLhZZTWjEg1Kcjkd").unwrap()
}

pub fn find_treasury_pool_token_account(treasury: &Pubkey, base_token_mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(treasury, base_token_mint)
}

pub fn find_multisig() -> Pubkey {
    // Credix use the same address for treasury and multisig on mainnet and devnet
    find_treasury()
}

pub fn find_multisig_token_account(multisig: &Pubkey, base_token_mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(multisig, base_token_mint)
}

pub fn find_credix_pass_pda(global_market_state: &Pubkey, pass_holder: &Pubkey) -> (Pubkey, u8) {
    credix_client::CredixPass::generate_pda(*global_market_state, *pass_holder)
}

pub fn find_withdraw_epoch_pda(global_market_state: &Pubkey, epoch_idx: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            global_market_state.as_ref(),
            &epoch_idx.to_be_bytes(),
            CREDIX_WITHDRAW_EPOCH_SEED.as_ref(),
        ],
        &mercurial_vault::ID,
    )
}

pub fn find_withdraw_request_pda(
    global_market_state: &Pubkey,
    epoch_idx: u32,
    investor: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            global_market_state.as_ref(),
            investor.as_ref(),
            &epoch_idx.to_be_bytes(),
            CREDIX_WITHDRAW_REQUEST_SEED.as_ref(),
        ],
        &mercurial_vault::ID,
    )
}
