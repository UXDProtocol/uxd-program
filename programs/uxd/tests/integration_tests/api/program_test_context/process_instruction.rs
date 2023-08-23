use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::integration_tests::api::program_test_context;

async fn process_instruction_result(
    instruction: Instruction,
    result: Result<(), program_test_context::ProgramTestError>,
) -> Result<(), program_test_context::ProgramTestError> {
    // Log the result, useful for debugging as STDOUT is displayed when a test fails
    println!(" -------- PROCESSING INSTRUCTION --------");
    println!(
        " - instruction.program_id: {:?}",
        instruction.program_id.to_string()
    );
    println!(" - instruction.data: {:?}", instruction.data);
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
    // Log accounts (only visible when the test fails)
    let mut idx = 0;
    for account in instruction.accounts {
        idx += 1;
        println!(" - instruction.account: #{:?} {:?}", idx, account.pubkey);
    }
    // Print result
    if result.is_ok() {
        println!(" - instruction.result: {:?}", "OK");
    } else {
        println!(" - instruction.result: {:?}", "ERROR");
    }
    // Done
    result
}

pub async fn process_instruction(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    instruction: Instruction,
    payer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let latest_blockhash = program_runner.get_latest_blockhash().await?;
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction.clone()], Some(&payer.pubkey()));
    transaction.partial_sign(&[payer], latest_blockhash);
    let result = program_runner.process_transaction(transaction).await;
    process_instruction_result(instruction.clone(), result).await
}

pub async fn process_instruction_with_signer(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    instruction: Instruction,
    payer: &Keypair,
    signer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let latest_blockhash = program_runner.get_latest_blockhash().await?;
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction.clone()], Some(&payer.pubkey()));
    transaction.partial_sign(&[payer, signer], latest_blockhash);
    let result = program_runner.process_transaction(transaction).await;
    process_instruction_result(instruction.clone(), result).await
}

pub async fn process_instruction_with_signers(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    instruction: Instruction,
    payer: &Keypair,
    signers: &[&Keypair],
) -> Result<(), program_test_context::ProgramTestError> {
    let latest_blockhash = program_runner.get_latest_blockhash().await?;
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction.clone()], Some(&payer.pubkey()));
    let mut keypairs = signers.to_owned();
    keypairs.push(payer);
    transaction.partial_sign(&keypairs, latest_blockhash);
    let result = program_runner.process_transaction(transaction).await;
    process_instruction_result(instruction.clone(), result).await
}
