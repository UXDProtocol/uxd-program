use solana_sdk::pubkey::Pubkey;

use crate::integration_tests::api::program_test_context;

pub async fn process_lamports_airdrop(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    to: &Pubkey,
    lamports: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    program_runner.process_aidrop(to, lamports).await
}
