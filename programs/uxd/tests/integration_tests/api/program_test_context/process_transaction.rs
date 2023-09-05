use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_program::message::VersionedMessage;
use solana_program_test::ProgramTestBanksClientExt;
use solana_program_test::ProgramTestContext;
use solana_sdk::message::v0::Message as MessageV0;
use solana_sdk::message::Message as MessageLegacy;
use solana_sdk::transaction::VersionedTransaction;

use crate::integration_tests::api::program_test_context;

use super::move_clock_forward;

async fn process_transaction_message_v0(
    program_test_context: &mut ProgramTestContext,
    message: &MessageV0,
) -> Result<(), program_test_context::ProgramTestError> {
    let account_keys = &message.account_keys;

    for address_table_lookup in &message.address_table_lookups {
        let dudu = program_test_context::read_account_data(
            program_test_context,
            &address_table_lookup.account_key,
        )
        .await?;
        let lookup = AddressLookupTable::deserialize(&dudu)
            .map_err(program_test_context::ProgramTestError::Instruction)?;

        let mut idx = 0;
        for address in lookup.addresses.to_vec() {
            println!(
                " - lookup-table.address: #{:?} {:?}",
                idx,
                address.to_string()
            );
            idx += 1;
        }
    }

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

    return Ok(());
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

async fn process_transaction_logs(
    program_test_context: &mut ProgramTestContext,
    transaction: &VersionedTransaction,
) -> Result<(), program_test_context::ProgramTestError> {
    println!(" -------- PROCESSING TRANSACTION --------");
    match &transaction.message {
        VersionedMessage::Legacy(message) => Ok(process_transaction_message_legacy(message)),
        VersionedMessage::V0(message) => {
            process_transaction_message_v0(program_test_context, message).await
        }
    }
}

pub async fn process_transaction(
    program_test_context: &mut ProgramTestContext,
    transaction: impl Into<VersionedTransaction>,
) -> Result<(), program_test_context::ProgramTestError> {
    let versionned_transaction: VersionedTransaction = transaction.into();
    // Log details about the transaction, useful for debugging as STDOUT is displayed only when a test fails
    process_transaction_logs(program_test_context, &versionned_transaction).await?;
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
