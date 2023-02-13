use solana_sdk::pubkey::Pubkey;

use crate::program_credix;

pub fn find_lp_token_mint_address() -> Pubkey {
    let (lp_token_mint, _) = credix_client::GlobalMarketState::generate_lp_token_mint_pda(
        &program_credix::accounts::get_market_seeds(),
    );
    assert_eq!(
        "B5TZoPTBK2SQPYasEPb759sY5KgHooUuFzRXPBtPRh5U",
        lp_token_mint.to_string()
    );
    return lp_token_mint;
}
