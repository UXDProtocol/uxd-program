use solana_sdk::pubkey::Pubkey;
const CREDIX_MARKETPLACE_SEED: &str = "credix-marketplace";

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
    credix_client::BorrowerInfo::generate_pda(*global_market_state, *borrower)
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

pub fn find_withdraw_epoch_pda(global_market_state: &Pubkey, epoch_idx: u32) -> (Pubkey, u8) {
    credix_client::WithdrawEpoch::generate_pda(*global_market_state, epoch_idx)
}

pub fn find_withdraw_request_pda(
    global_market_state: &Pubkey,
    investor: &Pubkey,
    epoch_idx: u32,
) -> (Pubkey, u8) {
    credix_client::WithdrawRequest::generate_pda(*global_market_state, *investor, epoch_idx)
}
