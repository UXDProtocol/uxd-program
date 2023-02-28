use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_identity_depository_mint_and_redeem() -> Result<(), String> {
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

    // Minting should fail because the user doesnt have collateral yet
    assert!(
        crate::integration_tests::cases::test_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
            &user,
            &user_collateral,
            &user_redeemable,
            500_000,
        )
        .await
        .is_err()
    );

    // Airdrop collateral to our user
    program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &program_keys.collateral_mint.pubkey(),
        &program_keys.collateral_authority,
        &user_collateral,
        500_000,
    )
    .await?;

    // Minting should fail because the controller cap is too low
    assert!(
        crate::integration_tests::cases::test_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
            &user,
            &user_collateral,
            &user_redeemable,
            500_000,
        )
        .await
        .is_err()
    );

    // Set the controller cap
    crate::integration_tests::cases::test_edit_controller(
        &mut program_test_context,
        &program_keys,
        &payer,
        Some(500_000),
    )
    .await?;

    // Minting should fail because the depository cap is too low
    assert!(
        crate::integration_tests::cases::test_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
            &user,
            &user_collateral,
            &user_redeemable,
            500_000,
        )
        .await
        .is_err()
    );

    // Set the depository cap and make sure minting is not disabled
    crate::integration_tests::cases::test_edit_identity_depository(
        &mut program_test_context,
        &program_keys,
        &payer,
        Some(500_000),
        Some(false),
    )
    .await?;

    // Minting too much should fail (above cap)
    assert!(
        crate::integration_tests::cases::test_mint_with_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
            &user,
            &user_collateral,
            &user_redeemable,
            1_000_000,
        )
        .await
        .is_err()
    );

    // Minting zero should fail
    assert!(
        crate::integration_tests::cases::test_mint_with_identity_depository(
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

    // Minting should work now that everything is set
    crate::integration_tests::cases::test_mint_with_identity_depository(
        &mut program_test_context,
        &program_keys,
        &payer,
        &user,
        &user_collateral,
        &user_redeemable,
        100_000,
    )
    .await?;

    // Redeeming too much should fail
    assert!(
        crate::integration_tests::cases::test_redeem_from_identity_depository(
            &mut program_test_context,
            &program_keys,
            &payer,
            &user,
            &user_collateral,
            &user_redeemable,
            1_000_000,
        )
        .await
        .is_err()
    );

    // Redeeming zero should fail
    assert!(
        crate::integration_tests::cases::test_redeem_from_identity_depository(
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

    // Redeeming the correct amount should succeed
    crate::integration_tests::cases::test_redeem_from_identity_depository(
        &mut program_test_context,
        &program_keys,
        &payer,
        &user,
        &user_collateral,
        &user_redeemable,
        100_000,
    )
    .await?;

    Ok(())
}
