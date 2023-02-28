use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;

use crate::integration_tests::api::program_uxd;

pub async fn test_edit_controller(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    redeemable_global_supply_cap: Option<u128>,
) -> Result<(), String> {
    // Execute
    program_uxd::instructions::process_edit_controller(
        program_test_context,
        program_keys,
        payer,
        redeemable_global_supply_cap,
    )
    .await?;

    // Read state after
    let controller_after =
        program_uxd::accounts::read_controller(program_test_context, &program_keys.controller)
            .await?;

    // Check result
    if redeemable_global_supply_cap.is_some() {
        let redeemable_global_supply_cap_after = controller_after.redeemable_global_supply_cap;
        assert_eq!(
            redeemable_global_supply_cap_after,
            redeemable_global_supply_cap.unwrap()
        );
    }

    // Done
    Ok(())
}
