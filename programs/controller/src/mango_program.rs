use std::str::FromStr;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

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
        #[cfg(feature = "devnet")]
        return Pubkey::from_str("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA").unwrap();
        #[cfg(not(feature = "devnet"))]
        return Pubkey::from_str("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68").unwrap();
    }
}
