use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditCredixLpDepositoryFields;
use uxd::instructions::EditDepositoriesRoutingWeightBps;
use uxd::instructions::EditIdentityDepositoryFields;
use uxd::instructions::EditMercurialVaultDepositoryFields;

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
    let amount_we_use_as_supply_cap = ui_amount_to_native_amount(1000, redeemable_mint_decimals);

    let amount_for_first_mint = ui_amount_to_native_amount(100, collateral_mint_decimals);
    let amount_for_second_mint = ui_amount_to_native_amount(200, collateral_mint_decimals);

    let amount_for_first_redeem = ui_amount_to_native_amount(20, redeemable_mint_decimals);
    let amount_for_second_redeem = ui_amount_to_native_amount(120, redeemable_mint_decimals);
    let amount_for_third_redeem = ui_amount_to_native_amount(10, redeemable_mint_decimals);

    let amount_of_collateral_airdropped_to_user = amount_for_first_mint + amount_for_second_mint; // Just enough money to mint

    // Post mint supply should match the configured weights
    let identity_depository_supply_after_first_mint = amount_for_first_mint * 10 / 100;
    let mercurial_vault_depository_supply_after_first_mint = amount_for_first_mint * 50 / 100;
    let credix_lp_depository_supply_after_first_mint = amount_for_first_mint * 40 / 100;

    // Post mint supply should match the configured weights
    let total_supply_after_second_mint = amount_for_first_mint + amount_for_second_mint;
    let identity_depository_supply_after_second_mint =
        total_supply_after_second_mint * 10 / 100 - 1; // Precision loss as a consequence of the first mint rounding
    let mercurial_vault_depository_supply_after_second_mint =
        total_supply_after_second_mint * 40 / 100 - 1; // Precision loss as a consequence of the first mint rounding
    let credix_lp_depository_supply_after_second_mint = total_supply_after_second_mint * 50 / 100;

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
                identity_depository_weight_bps: 10 * 100,
                mercurial_vault_depository_weight_bps: 50 * 100,
                credix_lp_depository_weight_bps: 40 * 100,
                alloyx_vault_depository_weight_bps: 0,
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Set the depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_context,
        &payer,
        &authority,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_disabled: Some(false),
        },
    )
    .await?;

    // Set the depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_mercurial_vault_depository(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditMercurialVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(0),
            redeeming_fee_in_bps: Some(0),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: None,
        },
    )
    .await?;

    // Set the depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(0),
            redeeming_fee_in_bps: Some(0),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: None,
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Verify that mint is not possible until we set the depositories address on controller
    // ---------------------------------------------------------------------

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
        identity_depository_supply_after_first_mint,
        mercurial_vault_depository_supply_after_first_mint,
        credix_lp_depository_supply_after_first_mint,
    )
    .await
    .is_err());

    // Now we set the router depositories to the correct PDAs
    program_uxd::procedures::process_set_router_depositories(
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
        identity_depository_supply_after_first_mint,
        mercurial_vault_depository_supply_after_first_mint,
        credix_lp_depository_supply_after_first_mint,
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
                identity_depository_weight_bps: 10 * 100,
                mercurial_vault_depository_weight_bps: 40 * 100,
                credix_lp_depository_weight_bps: 50 * 100,
                alloyx_vault_depository_weight_bps: 0,
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Minting should now respect the new weights
    // Note: due to the precision loss from the first mint, we need to adjust by 1 in some places
    program_uxd::instructions::process_mint(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_for_second_mint,
        identity_depository_supply_after_second_mint - identity_depository_supply_after_first_mint,
        mercurial_vault_depository_supply_after_second_mint
            - mercurial_vault_depository_supply_after_first_mint,
        credix_lp_depository_supply_after_second_mint
            - credix_lp_depository_supply_after_first_mint,
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
        amount_for_first_redeem,
        0,
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
        amount_for_first_redeem,
        0,
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

    // Redeeming after we exhausted the identity depository should fallback to mercurial depository
    // Even if mercurial is underflowing, it is the last liquid redeemable available, so we use it.
    let identity_depository_supply_after_first_redeem =
        identity_depository_supply_after_second_mint - amount_for_first_redeem;

    // It should completely empty the identity depository
    let expected_identity_depository_redeemable_amount =
        identity_depository_supply_after_first_redeem;
    // Then the rest should be taken from mercurial
    let expected_mercurial_vault_depository_redeemable_amount =
        amount_for_second_redeem - expected_identity_depository_redeemable_amount;

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
        expected_identity_depository_redeemable_amount,
        expected_mercurial_vault_depository_redeemable_amount,
    )
    .await
    .is_err());

    // Move 1 epoch forward (bypass outflow limit)
    program_context
        .move_clock_forward(1, slots_per_epoch)
        .await?;

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
        expected_identity_depository_redeemable_amount,
        expected_mercurial_vault_depository_redeemable_amount,
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
        0,
        0,
    )
    .await
    .is_err());

    // Done
    Ok(())
}
