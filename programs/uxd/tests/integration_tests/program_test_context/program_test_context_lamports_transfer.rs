use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn program_test_context_transfer_lamports(
    program_test_context: &mut ProgramTestContext,
    from: &Keypair,
    to: &Pubkey,
    lamports: u64,
) -> Result<(), String> {
    let instruction = solana_sdk::system_instruction::transfer(&from.pubkey(), &to, lamports);
    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[instruction],
        Some(&from.pubkey()),
        &[from],
        program_test_context.last_blockhash,
    );
    program_test_context
        .banks_client
        .process_transaction(transaction)
        .await
        .map_err(|e| e.to_string())
}
