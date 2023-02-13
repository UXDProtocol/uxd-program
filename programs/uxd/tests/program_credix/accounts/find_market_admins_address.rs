use solana_sdk::pubkey::Pubkey;

pub fn find_market_admins_address(global_market_state: Pubkey) -> Pubkey {
    let (market_admins, _) = credix_client::MarketAdmins::generate_pda(global_market_state);
    assert_eq!(
        "EZ9TgyMCdXashDbtpeGR3camCvw53164esph2cpiY8nv",
        market_admins.to_string()
    );
    return market_admins;
}
