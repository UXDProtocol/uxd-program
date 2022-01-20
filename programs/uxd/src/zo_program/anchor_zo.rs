use solana_program::pubkey::Pubkey;

#[derive(Clone)]
pub struct ZO;

impl anchor_lang::Id for ZO {
    fn id() -> Pubkey {
        zo_abi::ID
    }
}
