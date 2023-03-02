use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_identity_depository_mint_and_redeem(
) -> Result<(), program_test_context::ProgramTestError> {
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

    // Create the program keys structure (find/create all important keys)
    let program_keys = program_uxd::accounts::create_program_keys();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_test_context,
        &program_keys,
        &payer,
    )
    .await?;

    // Main actor
    let user = Keypair::new();

    // Create a collateral account for our user
    let user_collateral = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_test_context,
        &payer,
        &program_keys.collateral_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;
    // Create a redeemable account for our user
    let user_redeemable = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_test_context,
        &payer,
        &program_keys.redeemable_mint,
        &user.pubkey(),
    )
    .await?;

    // Useful amounts used during testing scenario
    let amount_we_use_as_supply_cap = program_keys.redeemable_amount_ui_to_native(50);
    let amount_bigger_than_the_supply_cap = program_keys.redeemable_amount_ui_to_native(300);

    let amount_of_collateral_airdropped_to_user = program_keys.collateral_amount_ui_to_native(1000);
    let amount_the_user_should_be_able_to_mint = program_keys.collateral_amount_ui_to_native(50);

    let amount_the_user_should_be_able_to_redeem = program_keys.redeemable_amount_ui_to_native(50);

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
        program_uxd::instructions::process_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
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
        &mut program_test_context,
        &payer,
        &program_keys.collateral_mint.pubkey(),
        &program_keys.collateral_mint_authority,
        &user_collateral,
        amount_of_collateral_airdropped_to_user,
    )
    .await?;

    // Minting should fail because the controller cap is too low
    assert!(
        program_uxd::instructions::process_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
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
        &mut program_test_context,
        &program_keys,
        &payer,
        Some(amount_we_use_as_supply_cap.into()),
    )
    .await?;

    // Minting should fail because the depository cap is too low
    assert!(
        program_uxd::instructions::process_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
            &user,
            &user_collateral,
            &user_redeemable,
            amount_the_user_should_be_able_to_mint,
        )
        .await
        .is_err()
    );

    // Set the depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_test_context,
        &program_keys,
        &payer,
        Some(amount_we_use_as_supply_cap.into()),
        Some(false),
    )
    .await?;

    // Minting too much should fail (above cap, but enough collateral)
    assert!(
        program_uxd::instructions::process_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
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
        program_uxd::instructions::process_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
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
    // -- After minting, we redeem (and it should succeed)
    // -- We also test invalid redeem amounts (and it should fail)
    // ---------------------------------------------------------------------

    // Minting should work now that everything is set
    program_uxd::instructions::process_mint_with_identity_depository(
        &mut program_test_context,
        &program_keys,
        &payer,
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_mint,
    )
    .await?;

    // Redeeming the correct amount should succeed
    program_uxd::instructions::process_redeem_from_identity_depository(
        &mut program_test_context,
        &program_keys,
        &payer,
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_redeem,
    )
    .await?;

    // Redeeming too much should fail
    assert!(
        program_uxd::instructions::process_redeem_from_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
            &user,
            &user_collateral,
            &user_redeemable,
            amount_bigger_than_the_supply_cap,
        )
        .await
        .is_err()
    );

    // Redeeming zero should fail
    assert!(
        program_uxd::instructions::process_redeem_from_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
            &user,
            &user_collateral,
            &user_redeemable,
            0,
        )
        .await
        .is_err()
    );

    Ok(())
}
