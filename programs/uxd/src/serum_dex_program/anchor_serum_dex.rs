use crate::declare_check_assert_macros;
use crate::error::check_assert;
use crate::error::SourceFileId;
use crate::UxdErrorCode;
use anchor_lang::prelude::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

declare_check_assert_macros!(SourceFileId::SerumDexProgramAnchorSerumDex);

#[derive(Clone)]
pub struct SerumDex;

pub mod anchor_serum_dex {
    #[cfg(feature = "development")]
    solana_program::declare_id!("DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY");
    #[cfg(feature = "production")]
    solana_program::declare_id!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
}

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
        anchor_serum_dex::ID
    }
}

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(serum_dex_program_id: &Pubkey) -> ProgramResult {
    check_eq!(
        serum_dex_program_id,
        &anchor_serum_dex::ID,
        UxdErrorCode::Default
    )?;
    Ok(())
}
