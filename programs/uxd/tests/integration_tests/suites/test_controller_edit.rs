use anchor_lang::prelude::Pubkey;
use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditDepositoriesRoutingWeightBps;
use uxd::instructions::EditRouterDepositories;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_controller_edit() -> Result<(), program_test_context::ProgramTestError> {
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
    let credix_multisig = Keypair::new();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_test_context,
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
    // -- Change the controller fields
    // ---------------------------------------------------------------------

    // Using the wrong authority should fail
    assert!(program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &payer,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(42),
            depositories_routing_weight_bps: None,
            router_depositories: None,
            limit_outflow_amount_per_day: None,
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
            depositories_routing_weight_bps: None,
            router_depositories: None,
            limit_outflow_amount_per_day: None,
        },
    )
    .await?;

    // Setting weights that dont add up to 100% should fail
    assert!(program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 1,
                mercurial_vault_depository_weight_bps: 1,
                credix_lp_depository_weight_bps: 1,
            }),
            router_depositories: None,
            limit_outflow_amount_per_day: None,
        },
    )
    .await
    .is_err());

    // Setting weights that add up to 100% should succeed
    program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 25 * 100,        // 25%
                mercurial_vault_depository_weight_bps: 35 * 100, // 35%
                credix_lp_depository_weight_bps: 40 * 100,       // 40%
            }),
            router_depositories: None,
            limit_outflow_amount_per_day: None,
        },
    )
    .await?;

    // Using the correct authority should allow to edit depositories addresses
    program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(100),
            depositories_routing_weight_bps: None,
            router_depositories: Some(EditRouterDepositories {
                identity_depository: Pubkey::default(),
                mercurial_vault_depository: Pubkey::default(),
                credix_lp_depository: Pubkey::default(),
            }),
            limit_outflow_amount_per_day: None,
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
            depositories_routing_weight_bps: None,
            router_depositories: None,
            limit_outflow_amount_per_day: None,
        },
    )
    .await?;

    // Setting everything at once should succeed
    program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(100),
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 20 * 100,        // 20%
                mercurial_vault_depository_weight_bps: 30 * 100, // 30%
                credix_lp_depository_weight_bps: 50 * 100,       // 50%
            }),
            router_depositories: Some(EditRouterDepositories {
                identity_depository: Pubkey::default(),
                mercurial_vault_depository: Pubkey::default(),
                credix_lp_depository: Pubkey::default(),
            }),
            limit_outflow_amount_per_day: Some(42),
        },
    )
    .await?;

    // Done
    Ok(())
}
