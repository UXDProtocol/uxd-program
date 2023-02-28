use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;

use crate::integration_tests::api::program_uxd;

pub async fn test_edit_identity_depository(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    redeemable_amount_under_management_cap: Option<u128>,
    minting_disabled: Option<bool>,
) -> Result<(), String> {
    // Execute
    program_uxd::instructions::process_edit_identity_depository(
        program_test_context,
        program_keys,
        payer,
        redeemable_amount_under_management_cap,
        minting_disabled,
    )
    .await?;

    // Read state after
    let identity_depository_after = program_uxd::accounts::read_identity_depository(
        program_test_context,
        &program_keys.identity_depository_keys.depository,
    )
    .await?;

    // Check result
    if redeemable_amount_under_management_cap.is_some() {
        let redeemable_amount_under_management_cap_after =
            identity_depository_after.redeemable_amount_under_management_cap;
        assert_eq!(
            redeemable_amount_under_management_cap_after,
            redeemable_amount_under_management_cap.unwrap()
        );
    }
    if minting_disabled.is_some() {
        let minting_disabled_after =
            identity_depository_after.minting_disabled;
        assert_eq!(
            minting_disabled_after,
            minting_disabled.unwrap()
        );
    }

    // Done
    Ok(())
}
