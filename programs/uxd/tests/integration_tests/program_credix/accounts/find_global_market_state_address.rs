use solana_sdk::pubkey::Pubkey;

use crate::integration_tests::program_credix;

pub fn find_global_market_state_address() -> Pubkey {
    let (global_market_state, _) = credix_client::GlobalMarketState::generate_pda(
        &program_credix::accounts::get_market_seeds(),
    );
    assert_eq!(
        "FKspd43n91ZqvEGWfSGBiG72F2t1Vuumj2zaG1z6nwbF",
        global_market_state.to_string()
    );
    return global_market_state;
}
