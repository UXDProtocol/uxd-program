use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;

pub async fn process_lamports_airdrop(
    program_test_context: &mut ProgramTestContext,
    to: &Pubkey,
    lamports: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    let from = Keypair::from_bytes(&program_test_context.payer.to_bytes())
        .map_err(|e| program_test_context::ProgramTestError::SignatureError(e.to_string()))?;
    let instruction = solana_program::system_instruction::transfer(&from.pubkey(), &to, lamports);
    program_test_context::process_instruction(program_test_context, instruction, &from).await
}
