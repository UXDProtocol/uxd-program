use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_edit_controller() -> Result<(), program_test_context::ProgramTestError> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Setup basic context and accounts needed for this test suite
    // ---------------------------------------------------------------------

    let mut program_test_context = program_test_context::create_program_test_context().await;

    // Fund payer
    let payer = Keypair::new();
    program_spl::instructions::process_lamports_airdrop(
        &mut program_test_context,
        &payer.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Hardcode mints decimals
    let collateral_mint_decimals = 6;
    let redeemable_mint_decimals = 6;

    // Important account keys
    let authority = Keypair::new();
    let collateral_mint = Keypair::new();
    let mercurial_vault_lp_mint = Keypair::new();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint,
        collateral_mint_decimals,
        redeemable_mint_decimals,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Change the controller fields
    // ---------------------------------------------------------------------

    // Using the wrong authority should fail
    assert!(program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &payer,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(42),
        },
    )
    .await
    .is_err());

    // Using the correct authority should succeed
    program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(100),
        },
    )
    .await?;

    // Using None should succeed
    program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
        },
    )
    .await?;

    // Done
    Ok(())
}
