use anchor_lang::prelude::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

// Temporary, the one I opened PR for when merged https://github.com/blockworks-foundation/mango-v3/pull/67a
#[derive(Clone)]
pub struct Mango;

// if the mango program use declare_id we can get ride of that
const MANGO_ID: &str = "4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA";

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
        return Pubkey::from_str(MANGO_ID).unwrap();
    }
}

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(mango_program_id: &Pubkey) -> ProgramResult {
    if mango_program_id != &Pubkey::from_str(MANGO_ID).unwrap() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
