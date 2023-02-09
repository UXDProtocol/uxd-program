use solana_sdk::pubkey::Pubkey;

pub fn find_signing_authority_address() -> Pubkey {
    let (signing_authority, _) = credix_client::GlobalMarketState::generate_signing_authority_pda(
        &crate::integration_tests::program_credix::accounts::get_market_seeds(),
    );
    assert_eq!(
        "HzukpMez1Cuz7emNW7XS6PK2ydnq85yDCapt8rrYKyGy",
        signing_authority.to_string()
    );
    return signing_authority;
}
