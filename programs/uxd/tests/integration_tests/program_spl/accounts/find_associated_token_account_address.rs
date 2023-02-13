use solana_sdk::pubkey::Pubkey;

pub fn find_associated_token_account_address(mint: &Pubkey, wallet: &Pubkey) -> Pubkey {
    anchor_spl::associated_token::get_associated_token_address(&wallet, mint)
}
