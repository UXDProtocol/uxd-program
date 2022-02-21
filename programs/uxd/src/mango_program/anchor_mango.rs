use anchor_lang::prelude::*;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

/// This is a wrapper around mango program that does not use Anchor,
/// similar to what Anchor does around the sol_token program.
#[derive(Clone)]
pub struct Mango;

pub mod mango_program_id {
    #[cfg(feature = "development")]
    solana_program::declare_id!("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");
    #[cfg(feature = "production")]
    solana_program::declare_id!("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");
}

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
        mango_program_id::ID
    }
}

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(mango_program_id: &Pubkey) -> Result<()> {
    if mango_program_id != &mango_program_id::ID {
        error!(UxdError::Default)
    }
    Ok(())
}
