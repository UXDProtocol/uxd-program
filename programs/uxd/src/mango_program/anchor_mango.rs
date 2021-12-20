use anchor_lang::prelude::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(Clone)]
pub struct Mango;

pub mod anchor_mango {
    // if the mango program use declare_id we can get ride of that
    use solana_program::declare_id;

    // Swap back if releasing
    declare_id!("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");
    //     #[cfg(feature = "devnet")]
    //     declare_id!("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");
    //     #[cfg(not(feature = "devnet"))]
    //     declare_id!("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");
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
        anchor_mango::ID
    }
}

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(mango_program_id: &Pubkey) -> ProgramResult {
    if mango_program_id != &anchor_mango::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
