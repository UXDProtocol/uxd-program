use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;

use crate::integration_tests::api::program_test_context;

pub async fn read_token_mint(
    program_test_context: &mut ProgramTestContext,
    token_mint: &Pubkey,
) -> Result<spl_token::state::Mint, String> {
    let raw_account_data =
        program_test_context::read_account(program_test_context, token_mint, &spl_token::id())
            .await?;
    spl_token::state::Mint::unpack(&raw_account_data).map_err(|e| e.to_string())
}
