use solana_program_test::ProgramTestContext;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::integration_tests::api::program_test_context;

use super::process_transaction;

pub async fn process_instruction(
    program_test_context: &mut ProgramTestContext,
    instruction: Instruction,
    payer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.partial_sign(&[payer], program_test_context.last_blockhash);
    process_transaction(program_test_context, transaction).await
}

pub async fn process_instruction_with_signer(
    program_test_context: &mut ProgramTestContext,
    instruction: Instruction,
    payer: &Keypair,
    signer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.partial_sign(&[payer, signer], program_test_context.last_blockhash);
    process_transaction(program_test_context, transaction).await
}

pub async fn process_instruction_with_signers(
    program_test_context: &mut ProgramTestContext,
    instruction: Instruction,
    payer: &Keypair,
    signers: &[&Keypair],
) -> Result<(), program_test_context::ProgramTestError> {
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    let mut keypairs = signers.to_owned();
    keypairs.push(payer);
    transaction.partial_sign(&keypairs, program_test_context.last_blockhash);
    process_transaction(program_test_context, transaction).await
}
