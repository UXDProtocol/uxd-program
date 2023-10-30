use solana_program_test::tokio;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditDepositoriesRoutingWeightBps;
use uxd::instructions::EditRouterDepositories;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::utils::ui_amount_to_native_amount;

#[tokio::test]
async fn test_mint_and_redeem() -> Result<(), program_context::ProgramError> {
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
    let alloyx_vault_mint = Keypair::new();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint,
        &credix_multisig,
        &alloyx_vault_mint,
        collateral_mint_decimals,
        redeemable_mint_decimals,
    )
    .await?;

    // Main actor
    let user = Keypair::new();

    // Create a collateral account for our user
    let user_collateral = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;
    // Create a redeemable account for our user
    let user_redeemable = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &program_uxd::accounts::find_redeemable_mint_pda().0,
        &user.pubkey(),
    )
    .await?;

    // Useful amounts used during testing scenario
    let amount_we_use_as_supply_cap =
        ui_amount_to_native_amount(10_000_000, redeemable_mint_decimals);

    let amount_for_first_mint = ui_amount_to_native_amount(1_000_000, collateral_mint_decimals);
    let amount_for_second_mint = ui_amount_to_native_amount(2_000_000, collateral_mint_decimals);

    let amount_for_first_redeem = ui_amount_to_native_amount(200_000, redeemable_mint_decimals);
    let amount_for_second_redeem = ui_amount_to_native_amount(1_500_000, redeemable_mint_decimals);
    let amount_for_third_redeem = ui_amount_to_native_amount(1_000_000, redeemable_mint_decimals);

    let amount_of_collateral_airdropped_to_user = amount_for_first_mint + amount_for_second_mint; // Just enough money to mint

    // Outflow epoch slot count
    let slots_per_epoch = 1000;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Airdrop collateral to our user, so we will be able to mint
    // -- Also configure and enable controller and all depositories
    // ---------------------------------------------------------------------

    // Airdrop collateral to our user
    program_spl::instructions::process_token_mint_to(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &collateral_mint,
        &user_collateral,
        amount_of_collateral_airdropped_to_user,
    )
    .await?;

    // Set the controller cap and weights
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(amount_we_use_as_supply_cap.into()),
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 25 * 100,        // 25%
                mercurial_vault_depository_weight_bps: 25 * 100, // 25%
                credix_lp_depository_weight_bps: 25 * 100,       // 25%
                alloyx_vault_depository_weight_bps: 25 * 100,    // 25%
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Setup the fees, caps and profits beneficiary for router depositories
    program_uxd::procedures::process_setup_router_depositories_fields(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        amount_we_use_as_supply_cap,
        Some(0),
        Some(0),
        Some(false),
        None,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Verify that mint is not possible until we set the depositories address on controller
    // ---------------------------------------------------------------------

    // Artificially unset the router_depositories
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: None,
            router_depositories: Some(EditRouterDepositories {
                identity_depository: Pubkey::default(),
                mercurial_vault_depository: Pubkey::default(),
                credix_lp_depository: Pubkey::default(),
                alloyx_vault_depository: Pubkey::default(),
            }),
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Minting should fail now, as the depositories are not set yet
    assert!(program_uxd::instructions::process_mint(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_first_mint,
        None, // We just cares that it fails to execute, we dont care the amounts
    )
    .await
    .is_err());

    // Now we set the router depositories to the correct PDAs
    program_uxd::procedures::process_set_controller_router_depositories(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Minting should now work and respect the weights
    // ---------------------------------------------------------------------

    // Post mint supply should match the configured weights
    let identity_depository_supply_after_first_mint = amount_for_first_mint / 3;
    let mercurial_vault_depository_supply_after_first_mint = amount_for_first_mint / 3;
    let credix_lp_depository_supply_after_first_mint = amount_for_first_mint / 3;

    // Minting should work now that everything is set, weights should be respected
    program_uxd::instructions::process_mint(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_first_mint,
        Some(program_uxd::instructions::ProcessMintExpectedMints {
            identity_depository_collateral_amount: identity_depository_supply_after_first_mint,
            mercurial_vault_depository_collateral_amount:
                mercurial_vault_depository_supply_after_first_mint,
            credix_lp_depository_collateral_amount: credix_lp_depository_supply_after_first_mint,
        }),
    )
    .await?;

    // Set the controller weights to new values
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 50 * 100,        // 50%
                mercurial_vault_depository_weight_bps: 20 * 100, // 20%
                credix_lp_depository_weight_bps: 30 * 100,       // 30%
                alloyx_vault_depository_weight_bps: 0,           // 0%
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Post mint supply should match the configured weights
    let total_supply_after_second_mint = amount_for_first_mint + amount_for_second_mint;
    let identity_depository_supply_after_second_mint = total_supply_after_second_mint * 50 / 100 - 2; // Precision loss as a consequence of roundings
    let mercurial_vault_depository_supply_after_second_mint =
        total_supply_after_second_mint * 20 / 100 - 1; // Precision loss as a consequence of the first mint rounding
    let credix_lp_depository_supply_after_second_mint = total_supply_after_second_mint * 30 / 100;

    // Minting should now respect the new weights
    program_uxd::instructions::process_mint(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_second_mint,
        Some(program_uxd::instructions::ProcessMintExpectedMints {
            identity_depository_collateral_amount: identity_depository_supply_after_second_mint
                - identity_depository_supply_after_first_mint,
            mercurial_vault_depository_collateral_amount:
                mercurial_vault_depository_supply_after_second_mint
                    - mercurial_vault_depository_supply_after_first_mint,
            credix_lp_depository_collateral_amount: credix_lp_depository_supply_after_second_mint
                - credix_lp_depository_supply_after_first_mint,
        }),
    )
    .await?;

    // Set the controller weights to 100% to mercurial_vault_depository
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 0,
                mercurial_vault_depository_weight_bps: 100 * 100,
                credix_lp_depository_weight_bps: 0,
                alloyx_vault_depository_weight_bps: 0,
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: Some(amount_for_first_redeem / 2), // Outflows configured too low on purpose
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: Some(slots_per_epoch),
        },
    )
    .await?;

    // Redeeming now should fail because that's too much outflow
    assert!(program_uxd::instructions::process_redeem(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_first_redeem,
        None, // We just cares that it fails to execute, we dont care the amounts
    )
    .await
    .is_err());

    // Increase the outflow limit to over what we want to redeem next
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: Some(amount_for_first_redeem),
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Redeeming now should work and not touch mercurial at all since it is underflowing
    // Meaning that other depositories are overflowing and should be prioritized
    program_uxd::instructions::process_redeem(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_first_redeem,
        Some(program_uxd::instructions::ProcessRedeemExpectedRedeems {
            identity_depository_redeemable_amount: amount_for_first_redeem,
            mercurial_vault_depository_redeemable_amount: 0,
        }),
    )
    .await?;

    // Increase the outflow limit to over what we want to redeem next
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: Some(amount_for_second_redeem),
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Redeeming immediately should fail because of outflow limit
    assert!(program_uxd::instructions::process_redeem(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_second_redeem,
        None, // We just cares that it fails to execute, we dont care the amounts
    )
    .await
    .is_err());

    // Move 1 epoch forward (bypass outflow limit)
    program_context
        .move_clock_forward(1, slots_per_epoch)
        .await?;

    // Redeeming after we exhausted the identity depository should fallback to mercurial depository
    // Even if mercurial is underflowing, it is the last liquid redeemable available, so we use it.
    let identity_depository_supply_after_first_redeem =
        identity_depository_supply_after_second_mint - amount_for_first_redeem;

    // It should now succeed doing the same thing after waiting a day
    program_uxd::instructions::process_redeem(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_second_redeem,
        Some(program_uxd::instructions::ProcessRedeemExpectedRedeems {
            // It should completely empty the identity depository
            identity_depository_redeemable_amount: identity_depository_supply_after_first_redeem,
            // Then the rest should be taken from mercurial
            mercurial_vault_depository_redeemable_amount: amount_for_second_redeem
                - identity_depository_supply_after_first_redeem,
        }),
    )
    .await?;

    // Move 1 epoch forward (bypass outflow limit)
    program_context
        .move_clock_forward(1, slots_per_epoch)
        .await?;

    // Any more redeeming will fail as all the liquid redeem source have been exhausted now
    assert!(program_uxd::instructions::process_redeem(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_third_redeem,
        None, // We just cares that it fails to execute, we dont care the amounts
    )
    .await
    .is_err());

    // Done
    Ok(())
}
