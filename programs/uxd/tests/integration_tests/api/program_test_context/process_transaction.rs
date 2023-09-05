use solana_program::message::VersionedMessage;
use solana_program_test::ProgramTestBanksClientExt;
use solana_program_test::ProgramTestContext;
use solana_sdk::message::v0::Message as MessageV0;
use solana_sdk::message::Message as MessageLegacy;
use solana_sdk::transaction::VersionedTransaction;

use crate::integration_tests::api::program_test_context;

use super::move_clock_forward;

fn process_transaction_message_v0(message: &MessageV0) {
    let account_keys = &message.account_keys;
    return;
    // Inspect all instructions one by one
    for instruction in &message.instructions {
        // Log the instruction program_id and data
        println!(" - instruction");
        println!(
            " - instruction.program_id: {:?}",
            account_keys[usize::from(instruction.program_id_index)].to_string()
        );
        println!(" - instruction.data: {:?}", instruction.data);
        // Log the callers for quickly glace over the flow of IXs using minified backtrace
        let backtrace_data = std::backtrace::Backtrace::force_capture();
        let backtrace_formatted = std::format!("{}", backtrace_data);
        let backtrace_lines = backtrace_formatted.lines();
        for backtrace_line in backtrace_lines {
            if backtrace_line.contains("at ./tests/integration_tests")
                && !(backtrace_line.contains("process_instruction")
                    || backtrace_line.contains("process_transaction"))
            {
                println!(" - instruction.from: {}", backtrace_line.trim());
            }
        }
        // Log accounts (only visible when the test fails)
        let mut idx = 0;
        for account_index in &instruction.accounts {
            idx += 1;
            println!(
                " - instruction.account: #{:?} {:?}",
                idx,
                account_keys[usize::from(*account_index)].to_string()
            );
        }
    }
}

fn process_transaction_message_legacy(message: &MessageLegacy) {
    let account_keys = &message.account_keys;
    // Inspect all instructions one by one
    for instruction in &message.instructions {
        // Log the instruction program_id and data
        println!(" - instruction");
        println!(
            " - instruction.program_id: {:?}",
            account_keys[usize::from(instruction.program_id_index)].to_string()
        );
        println!(" - instruction.data: {:?}", instruction.data);
        // Log the callers for quickly glace over the flow of IXs using minified backtrace
        let backtrace_data = std::backtrace::Backtrace::force_capture();
        let backtrace_formatted = std::format!("{}", backtrace_data);
        let backtrace_lines = backtrace_formatted.lines();
        for backtrace_line in backtrace_lines {
            if backtrace_line.contains("at ./tests/integration_tests")
                && !(backtrace_line.contains("process_instruction")
                    || backtrace_line.contains("process_transaction"))
            {
                println!(" - instruction.from: {}", backtrace_line.trim());
            }
        }
        // Log accounts (only visible when the test fails)
        let mut idx = 0;
        for account_index in &instruction.accounts {
            idx += 1;
            println!(
                " - instruction.account: #{:?} {:?}",
                idx,
                account_keys[usize::from(*account_index)].to_string()
            );
        }
    }
}

fn process_transaction_logs(transaction: &VersionedTransaction) {
    println!(" -------- PROCESSING TRANSACTION --------");
    match &transaction.message {
        VersionedMessage::Legacy(message) => process_transaction_message_legacy(message),
        VersionedMessage::V0(message) => process_transaction_message_v0(message),
    }
}

pub async fn process_transaction(
    program_test_context: &mut ProgramTestContext,
    transaction: impl Into<VersionedTransaction>,
) -> Result<(), program_test_context::ProgramTestError> {
    let versionned_transaction: VersionedTransaction = transaction.into();
    // Log details about the transaction, useful for debugging as STDOUT is displayed only when a test fails
    process_transaction_logs(&versionned_transaction);
    move_clock_forward(program_test_context, 1, 1).await?;
    // Actually process the transaction
    let result = program_test_context
        .banks_client
        .process_transaction(versionned_transaction)
        .await
        .map_err(program_test_context::ProgramTestError::BanksClient);
    // Increment the blockhash, so that the next transaction can run sequentially
    program_test_context.last_blockhash = program_test_context
        .banks_client
        .get_new_latest_blockhash(&program_test_context.last_blockhash)
        .await
        .map_err(program_test_context::ProgramTestError::Io)?;
    // Log the result
    if result.is_ok() {
        println!(" - transaction.result: {:?}", "OK");
    } else {
        println!(" - transaction.result: {:?}", "ERROR");
    }
    // Done
    result
}
