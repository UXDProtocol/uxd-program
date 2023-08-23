use solana_program_test::tokio;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditDepositoriesRoutingWeightBps;
use uxd::instructions::EditRouterDepositories;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_controller_edit() -> Result<(), program_context::ProgramError> {
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
    // -- Change the controller fields
    // ---------------------------------------------------------------------

    // Using the wrong authority should fail
    assert!(program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &payer,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(42),
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await
    .is_err());

    // Using the correct authority should succeed
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(100),
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Setting weights that dont add up to 100% should fail
    assert!(program_uxd::instructions::process_edit_controller(
        &mut program_context,
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
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await
    .is_err());

    // Setting weights that add up to 100% should succeed
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
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
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Using the correct authority should allow to edit depositories addresses
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
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
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Using None should succeed
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
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

    // Setting everything at once should succeed
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
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
            outflow_limit_per_epoch_amount: Some(42),
            outflow_limit_per_epoch_bps: Some(42),
            slots_per_epoch: Some(42),
        },
    )
    .await?;

    // Done
    Ok(())
}
