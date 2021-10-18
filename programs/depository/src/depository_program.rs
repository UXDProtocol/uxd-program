use crate::*;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(Clone)]
pub struct Depository;

impl anchor_lang::AccountDeserialize for Depository {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Depository::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Ok(Depository)
    }
}

impl anchor_lang::Id for Depository {
    fn id() -> Pubkey {
        ID
    }
}