use solana_program_test::ProgramTestContext;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

pub async fn program_test_context_execute_instruction_with_signer(
    program_test_ctx: &mut ProgramTestContext,
    instruction: Instruction,
    signer: &Keypair,
) -> Option<()> {
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        program_test_ctx.last_blockhash,
    );
    program_test_ctx
        .banks_client
        .process_transaction(transaction)
        .await
        .ok()
}
