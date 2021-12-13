use anchor_lang::prelude::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Clone)]
pub struct SerumDex;

// if the serum-dex program use declare_id we can get ride of that
// TODO "Change this to mainnet before release"
const SERUM_DEX_ID: &str = "DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY"; // Mainnet 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin

impl anchor_lang::AccountDeserialize for SerumDex {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        SerumDex::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Ok(SerumDex)
    }
}

impl anchor_lang::Id for SerumDex {
    fn id() -> Pubkey {
        return Pubkey::from_str(SERUM_DEX_ID).unwrap();
    }
}

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(serum_dex_program_id: &Pubkey) -> ProgramResult {
    if serum_dex_program_id != &Pubkey::from_str(SERUM_DEX_ID).unwrap() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
