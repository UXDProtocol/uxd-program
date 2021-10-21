use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

// Temporary, the one I opened PR for when merged https://github.com/blockworks-foundation/mango-v3/pull/67
#[derive(Clone)]
pub struct Mango;

impl anchor_lang::AccountDeserialize for Mango {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Mango::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Ok(Mango)
    }
}

impl anchor_lang::Id for Mango {
    fn id() -> Pubkey {
        return Pubkey::from_str("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA").unwrap();
    }
}