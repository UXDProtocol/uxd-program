use solana_program::clock::Clock;
use solana_program_test::ProgramTestContext;

use crate::integration_tests::api::program_test_context;

#[allow(dead_code)] // This will be used by credix rebalancing tests logic soon
pub async fn move_clock_forward(
    program_test_context: &mut ProgramTestContext,
    unix_timestamp_delta: i64,
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
        current_clock.unix_timestamp + unix_timestamp_delta,
    )
    .await
}

#[allow(dead_code)] // This will be used by credix rebalancing tests logic soon
pub async fn move_clock_to(
    program_test_context: &mut ProgramTestContext,
    unix_timestamp: i64,
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
    forwarded_clock.unix_timestamp = unix_timestamp;

    // Update the context sysvar clock
    program_test_context.set_sysvar::<Clock>(&forwarded_clock);

    // Done
    Ok(())
}
