use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::integration_tests::api::program_test_context;

async fn process_instruction_result(
    program_test_context: &mut ProgramTestContext,
    instruction: Instruction,
    result: Result<(), program_test_context::ProgramTestError>,
) -> Result<(), program_test_context::ProgramTestError> {
    // Increment the blockhash, so that the next transaction can run sequentially
    program_test_context.last_blockhash = program_test_context
        .get_new_latest_blockhash()
        .await
        .map_err(program_test_context::ProgramTestError::Io)?;
    // Log the result, useful for debugging as STDOUT is displayed when a test fails
    println!(" -------- PROCESSING INSTRUCTION --------");
    println!(
        " - instruction.program_id: {:?}",
        instruction.program_id.to_string()
    );
    println!(" - instruction.data: {:?}", instruction.data);
    println!(" - instruction.result: {:?}", result);
    // Log the callers for quickly glace over the flow of IXs using minified backtrace
    let backtrace_data = std::backtrace::Backtrace::force_capture();
    let backtrace_formatted = std::format!("{}", backtrace_data);
    let backtrace_lines = backtrace_formatted.lines();
    for backtrace_line in backtrace_lines {
        if backtrace_line.contains("at ./tests/integration_tests")
            && !backtrace_line.contains("process_instruction")
        {
            println!(" - instruction.from: {}", backtrace_line.trim());
        }
    }
    // Done
    result
}

pub async fn process_instruction(
    program_test_context: &mut ProgramTestContext,
    instruction: Instruction,
    payer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction.clone()], Some(&payer.pubkey()));
    transaction.partial_sign(&[payer], program_test_context.last_blockhash);
    let result = program_test_context
        .banks_client
        .process_transaction(transaction)
        .await
        .map_err(program_test_context::ProgramTestError::BanksClient);
    process_instruction_result(program_test_context, instruction.clone(), result).await
}

pub async fn process_instruction_with_signer(
    program_test_context: &mut ProgramTestContext,
    instruction: Instruction,
    payer: &Keypair,
    signer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction.clone()], Some(&payer.pubkey()));
    transaction.partial_sign(&[payer, signer], program_test_context.last_blockhash);
    let result = program_test_context
        .banks_client
        .process_transaction(transaction)
        .await
        .map_err(program_test_context::ProgramTestError::BanksClient);
    process_instruction_result(program_test_context, instruction.clone(), result).await
}
