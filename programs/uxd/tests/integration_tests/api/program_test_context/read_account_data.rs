use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;

pub async fn read_account_data(
    program_test_context: &mut ProgramTestContext,
    address: &Pubkey,
) -> Result<Vec<u8>, String> {
    let raw_account = program_test_context
        .banks_client
        .get_account(*address)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("AccountDoesNotExist")?;
    Ok(raw_account.data)
}
