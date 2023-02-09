use solana_program::rent::Rent;
use solana_program_test::ProgramTestContext;

pub async fn get_rent(program_test_context: &mut ProgramTestContext) -> Result<Rent, String> {
    program_test_context
        .banks_client
        .get_rent()
        .await
        .map_err(|e| e.to_string())
}
