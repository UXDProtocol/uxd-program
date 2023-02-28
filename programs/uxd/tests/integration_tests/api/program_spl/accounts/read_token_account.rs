use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;

use crate::integration_tests::api::program_test_context;

pub async fn read_token_account(
    program_test_context: &mut ProgramTestContext,
    token_account: &Pubkey,
) -> Result<spl_token::state::Account, String> {
    let data =
        program_test_context::read_account(program_test_context, token_account, &spl_token::id())
            .await?;
    Ok(spl_token::state::Account::unpack(&data).map_err(|e| e.to_string())?)
}
