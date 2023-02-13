use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test_context;

pub async fn process_lamports_airdrop(
    program_test_context: &mut ProgramTestContext,
    to: &Pubkey,
    lamports: u64,
) -> Result<(), String> {
    let from =
        Keypair::from_bytes(&program_test_context.payer.to_bytes()).map_err(|e| e.to_string())?;
    let instruction = solana_sdk::system_instruction::transfer(&from.pubkey(), &to, lamports);
    program_test_context::process_instruction(program_test_context, instruction, &from).await
}
