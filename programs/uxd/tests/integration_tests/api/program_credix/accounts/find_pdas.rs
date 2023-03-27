use solana_program::pubkey::Pubkey;

const CREDIX_MARKETPLACE_SEED: &str = "this-can-be-whatever";

const CREDIX_BORROWER_INFO_SEED: &str = "borrower-info";

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

pub fn find_treasury(multisig: &Pubkey) -> Pubkey {
    *multisig // The treasury is the same key as the multisig on mainnet
}

pub fn find_treasury_pool_token_account(treasury: &Pubkey, base_token_mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(treasury, base_token_mint)
}

pub fn find_credix_pass_pda(global_market_state: &Pubkey, pass_holder: &Pubkey) -> (Pubkey, u8) {
    credix_client::CredixPass::generate_pda(*global_market_state, *pass_holder)
}

pub fn find_borrower_info_pda(global_market_state: &Pubkey, borrower: &Pubkey) -> (Pubkey, u8) {
    // credix_client::BorrowerInfo::generate_pda(market_seeds, *borrower) // doesnt work
    Pubkey::find_program_address(
        &[
            global_market_state.as_ref(),
            borrower.as_ref(),
            CREDIX_BORROWER_INFO_SEED.as_bytes(),
        ],
        &credix_client::id(),
    )
}

pub fn find_deal_pda(
    global_market_state: &Pubkey,
    borrower: &Pubkey,
    deal_number: u16,
) -> (Pubkey, u8) {
    credix_client::Deal::generate_pda(*global_market_state, *borrower, deal_number)
}

pub fn find_deal_token_account_pda(global_market_state: &Pubkey, deal: &Pubkey) -> (Pubkey, u8) {
    credix_client::Deal::generate_deal_token_account_pda(*global_market_state, *deal)
}

pub fn find_deal_tranches_pda(global_market_state: &Pubkey, deal: &Pubkey) -> (Pubkey, u8) {
    credix_client::DealTranches::generate_pda(*global_market_state, *deal)
}

pub fn find_repayment_schedule_pda(global_market_state: &Pubkey, deal: &Pubkey) -> (Pubkey, u8) {
    credix_client::RepaymentSchedule::generate_pda(*global_market_state, *deal)
}
