use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

pub async fn program_test_context_lamports_airdrop(
    program_test_context: &mut ProgramTestContext,
    to: &Pubkey,
    lamports: u64,
) -> Result<(), String> {
    let instruction = solana_sdk::system_instruction::transfer(
        &program_test_context.payer.pubkey(),
        &to,
        lamports,
    );
    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[instruction],
        Some(&program_test_context.payer.pubkey()),
        &[&program_test_context.payer],
        program_test_context.last_blockhash,
    );
    program_test_context
        .banks_client
        .process_transaction(transaction)
        .await
        .map_err(|e| e.to_string())
}
