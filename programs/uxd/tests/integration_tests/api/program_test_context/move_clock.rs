use solana_program_test::ProgramTestContext;
use solana_sdk::clock::Clock;

use crate::integration_tests::api::program_test_context;

pub async fn move_clock_forward(
    program_test_context: &mut ProgramTestContext,
    unix_timestamp_delta: u64,
    slot_delta: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Read the context sysvar clock
    let current_clock = program_test_context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .map_err(program_test_context::ProgramTestError::BanksClient)?;

    // Move the clock
    move_clock_to(
        program_test_context,
        current_clock.unix_timestamp + i64::try_from(unix_timestamp_delta).unwrap(),
        current_clock.slot + slot_delta,
    )
    .await
}

pub async fn move_clock_to(
    program_test_context: &mut ProgramTestContext,
    unix_timestamp: i64,
    slot: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Read the context sysvar clock
    let current_clock = program_test_context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .map_err(program_test_context::ProgramTestError::BanksClient)?;

    // Update the clock fields
    let mut forwarded_clock = current_clock;
    forwarded_clock.epoch += 1;
    forwarded_clock.slot = slot;
    forwarded_clock.unix_timestamp = unix_timestamp;

    // Update the context sysvar clock
    program_test_context.set_sysvar::<Clock>(&forwarded_clock);

    // Done
    Ok(())
}
