use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

const CREDIX_MARKETPLACE_SEED: &str = "this-can-be-whatever";

pub struct ProgramKeys {
    pub authority: Keypair,
    pub program_state: Pubkey,
    pub market_seeds: String,
    pub global_market_state: Pubkey,
    pub market_admins: Pubkey,
    pub signing_authority: Pubkey,
    pub lp_token_mint: Pubkey,
    pub base_token_mint: Pubkey,
    pub liquidity_pool_token_account: Pubkey,
    pub treasury: Pubkey,
    pub treasury_pool_token_account: Pubkey,
}

pub fn create_program_keys(collateral_mint: &Pubkey) -> ProgramKeys {
    let authority = Keypair::new();

    let program_state = credix_client::ProgramState::generate_pda().0;

    let market_seeds = String::from(CREDIX_MARKETPLACE_SEED);

    let global_market_state = credix_client::GlobalMarketState::generate_pda(&market_seeds).0;

    let market_admins = credix_client::MarketAdmins::generate_pda(global_market_state).0;

    let signing_authority =
        credix_client::GlobalMarketState::generate_signing_authority_pda(&market_seeds).0;

    let lp_token_mint =
        credix_client::GlobalMarketState::generate_lp_token_mint_pda(&market_seeds).0;

    let base_token_mint = *collateral_mint;

    let liquidity_pool_token_account = spl_associated_token_account::get_associated_token_address(
        &signing_authority,
        &base_token_mint,
    );

    let treasury = authority.pubkey();
    let treasury_pool_token_account =
        spl_associated_token_account::get_associated_token_address(&treasury, &base_token_mint);

    ProgramKeys {
        authority,
        program_state,
        market_seeds,
        global_market_state,
        market_admins,
        signing_authority,
        lp_token_mint,
        base_token_mint,
        liquidity_pool_token_account,
        treasury,
        treasury_pool_token_account,
    }
}
