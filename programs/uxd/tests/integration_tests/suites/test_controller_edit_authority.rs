use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use uxd::instructions::EditControllerFields;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_controller_edit_authority() -> Result<(), program_context::ProgramError> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Setup basic context and accounts needed for this test suite
    // ---------------------------------------------------------------------

    let mut program_context: Box<dyn program_context::ProgramContext> =
        Box::new(program_context::create_program_test_context().await);

    // Fund payer
    let payer = Keypair::new();
    program_context
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    // Hardcode mints decimals
    let collateral_mint_decimals = 6;
    let redeemable_mint_decimals = 6;

    // Important account keys
    let authority = Keypair::new();
    let collateral_mint = Keypair::new();
    let mercurial_vault_lp_mint = Keypair::new();
    let credix_multisig = Keypair::new();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint,
        &credix_multisig,
        collateral_mint_decimals,
        redeemable_mint_decimals,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Change the controller authority back and forth
    // ---------------------------------------------------------------------

    let old_authority = authority;
    let new_authority = Keypair::new();

    // Using the wrong authority should fail
    assert!(
        program_uxd::instructions::process_edit_controller_authority(
            &mut program_context,
            &payer,
            &payer,
            &new_authority.pubkey(),
        )
        .await
        .is_err()
    );

    // Using the correct authority should succeed
    program_uxd::instructions::process_edit_controller_authority(
        &mut program_context,
        &payer,
        &old_authority,
        &new_authority.pubkey(),
    )
    .await?;

    // After changing the authority we cant use it again
    assert!(
        program_uxd::instructions::process_edit_controller_authority(
            &mut program_context,
            &payer,
            &old_authority,
            &new_authority.pubkey(),
        )
        .await
        .is_err()
    );

    // The new authority can use it now
    program_uxd::instructions::process_edit_controller_authority(
        &mut program_context,
        &payer,
        &new_authority,
        &new_authority.pubkey(),
    )
    .await?;

    // The old authority can not use the program anymore, but the new one can
    assert!(program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &old_authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await
    .is_err());
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &new_authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // The new authority can send the authority again back to the old one
    program_uxd::instructions::process_edit_controller_authority(
        &mut program_context,
        &payer,
        &new_authority,
        &old_authority.pubkey(),
    )
    .await?;

    // The new authority can not use the program anymore, but the old one can
    assert!(program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &new_authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await
    .is_err());
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &old_authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Done
    Ok(())
}
