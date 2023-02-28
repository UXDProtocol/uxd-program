use anchor_lang::AccountDeserialize;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;

use crate::integration_tests::api::program_test_context;

pub async fn read_identity_depository(
    program_test_context: &mut ProgramTestContext,
    identity_depository: &Pubkey,
) -> Result<uxd::state::IdentityDepository, String> {
    let data =
        program_test_context::read_account(program_test_context, identity_depository, &uxd::id())
            .await?;
    let mut slice: &[u8] = &data;
    Ok(uxd::state::IdentityDepository::try_deserialize(&mut slice).map_err(|e| e.to_string())?)
}
