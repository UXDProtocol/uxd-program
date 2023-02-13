use solana_program::program_pack::Pack;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;

pub async fn get_token_account(
    program_test_context: &mut ProgramTestContext,
    token_account: &Pubkey,
) -> Result<spl_token::state::Account, String> {
    let raw_account = program_test_context
        .banks_client
        .get_account(*token_account)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("AccountDoesNotExist")?;
    if raw_account.owner != spl_token::ID {
        return Err(String::from("InvalidTokenAccount"));
    }
    Ok(spl_token::state::Account::unpack(&raw_account.data).map_err(|e| e.to_string())?)
}
