use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn process_lamports_transfer(
    program_test_context: &mut ProgramTestContext,
    from: &Keypair,
    to: &Pubkey,
    lamports: u64,
) -> Result<(), String> {
    let instruction = solana_sdk::system_instruction::transfer(&from.pubkey(), &to, lamports);
    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        from,
        from,
    )
    .await
}