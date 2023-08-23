use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditCredixLpDepositoryFields;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::utils::ui_amount_to_native_amount;

#[tokio::test]
async fn test_credix_lp_depository_mint() -> Result<(), program_test_context::ProgramTestError> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Setup basic context and accounts needed for this test suite
    // ---------------------------------------------------------------------

    let mut program_runner = program_test_context::create_program_test_context().await;

    // Fund payer
    let payer = Keypair::new();
    program_test_context::ProgramRunner::process_airdrop(
        &mut program_runner,
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
        &mut program_runner,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint,
        &credix_multisig,
        collateral_mint_decimals,
        redeemable_mint_decimals,
    )
    .await?;

    // Main actor
    let user = Keypair::new();

    // Create a collateral account for our user
    let user_collateral = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_runner,
        &payer,
        &collateral_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;
    // Create a redeemable account for our user
    let user_redeemable = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_runner,
        &payer,
        &program_uxd::accounts::find_redeemable_mint_pda().0,
        &user.pubkey(),
    )
    .await?;

    // Useful amounts used during testing scenario
    let amount_we_use_as_supply_cap = ui_amount_to_native_amount(50, redeemable_mint_decimals);
    let amount_bigger_than_the_supply_cap =
        ui_amount_to_native_amount(300, redeemable_mint_decimals);

    let amount_of_collateral_airdropped_to_user =
        ui_amount_to_native_amount(1000, collateral_mint_decimals);
    let amount_the_user_should_be_able_to_mint =
        ui_amount_to_native_amount(50, collateral_mint_decimals);

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- We try to mint (and it should fail)
    // -- and progressively set all things needed to mint one by one:
    // --  - airdrop collateral to our user
    // --  - set the supply cap in the controller
    // --  - set the redeemable cap and enable minting in the identity depository
    // -- when everything is ready, try to mint incorrect amounts (it should fail)
    // ---------------------------------------------------------------------

    // Minting should fail because the user doesnt have collateral yet
    assert!(
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_runner,
            &payer,
            &authority,
            &collateral_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_the_user_should_be_able_to_mint,
        )
        .await
        .is_err()
    );

    // Airdrop collateral to our user
    program_spl::instructions::process_token_mint_to(
        &mut program_runner,
        &payer,
        &collateral_mint.pubkey(),
        &collateral_mint,
        &user_collateral,
        amount_of_collateral_airdropped_to_user,
    )
    .await?;

    // Minting should fail because the controller cap is too low
    assert!(
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_runner,
            &payer,
            &authority,
            &collateral_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_the_user_should_be_able_to_mint,
        )
        .await
        .is_err()
    );

    // Set the controller cap
    program_uxd::instructions::process_edit_controller(
        &mut program_runner,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(amount_we_use_as_supply_cap.into()),
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Minting should fail because the depository cap is too low
    assert!(
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_runner,
            &payer,
            &authority,
            &collateral_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_the_user_should_be_able_to_mint,
        )
        .await
        .is_err()
    );

    // Set the depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_runner,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(100),
            redeeming_fee_in_bps: Some(100),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: None,
        },
    )
    .await?;

    // Minting too much should fail (above cap, but enough collateral)
    assert!(
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_runner,
            &payer,
            &authority,
            &collateral_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_bigger_than_the_supply_cap,
        )
        .await
        .is_err()
    );

    // Minting zero should fail
    assert!(
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_runner,
            &payer,
            &authority,
            &collateral_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            0,
        )
        .await
        .is_err()
    );

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Everything is ready for minting
    // -- We should now successfully be able to mint
    // ---------------------------------------------------------------------

    // Minting should work now that everything is set
    program_uxd::instructions::process_mint_with_credix_lp_depository(
        &mut program_runner,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_mint,
    )
    .await?;

    // Done
    Ok(())
}
