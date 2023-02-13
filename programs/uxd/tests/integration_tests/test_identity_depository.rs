use solana_program_test::processor;
use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::program_spl::accounts::get_token_account;

#[tokio::test]
async fn test_identity_depository() -> Result<(), String> {
    let mut program_test = ProgramTest::default();
    program_test.add_program("uxd", uxd::id(), processor!(uxd::entry));

    let mut program_test_context: ProgramTestContext = program_test.start_with_context().await;

    // Fund payer
    let payer = Keypair::new();
    crate::integration_tests::program_spl::instructions::process_lamports_airdrop(
        &mut program_test_context,
        &payer.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Main actors
    let uxd_authority = Keypair::new();
    let user = Keypair::new();

    // Create the collateral mint
    let collateral_authority = Keypair::new();
    let collateral_mint = Keypair::new();
    crate::integration_tests::program_spl::instructions::process_token_mint_init(
        &mut program_test_context,
        &payer,
        &collateral_mint,
        6,
        &collateral_authority.pubkey(),
    )
    .await?;

    // Give some collateral to our user and create its account
    let user_collateral =
        crate::integration_tests::program_spl::instructions::process_associated_token_account_init(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &user.pubkey(),
        )
        .await?;
    crate::integration_tests::program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &collateral_authority,
        &user_collateral,
        1_000_000,
    )
    .await?;

    // Initialize basic UXD program state
    crate::integration_tests::program_uxd::instructions::process_initialize_controller(
        &mut program_test_context,
        &payer,
        &uxd_authority,
        6,
    )
    .await?;
    crate::integration_tests::program_uxd::instructions::process_initialize_identity_depository(
        &mut program_test_context,
        &payer,
        &uxd_authority,
        &collateral_mint.pubkey(),
    )
    .await?;
    crate::integration_tests::program_uxd::instructions::process_edit_identity_depository(
        &mut program_test_context,
        &payer,
        &uxd_authority,
        Some(1_000_000),
        Some(false),
    )
    .await?;

    // Create a redeemable account for our user
    let redeemable_mint =
        crate::integration_tests::program_uxd::accounts::find_redeemable_mint_address();
    let user_redeemable =
        crate::integration_tests::program_spl::instructions::process_associated_token_account_init(
            &mut program_test_context,
            &payer,
            &redeemable_mint,
            &user.pubkey(),
        )
        .await?;

    // Check user collateral original amount
    assert_eq!(
        1_000_000,
        get_token_account(&mut program_test_context, &user_collateral)
            .await?
            .amount
    );
    // Check user redeemable original amount
    assert_eq!(
        0,
        get_token_account(&mut program_test_context, &user_redeemable)
            .await?
            .amount
    );

    // Mint using identity depository
    crate::integration_tests::program_uxd::instructions::process_mint_with_identity_depository(
        &mut program_test_context,
        &payer,
        &user,
        &user_collateral,
        &user_redeemable,
        500_000,
    )
    .await?;

    // Check user collateral decreased
    assert_eq!(
        1_000_000 - 500_000,
        get_token_account(&mut program_test_context, &user_collateral)
            .await?
            .amount
    );
    // Check user redeemable increased
    assert_eq!(
        0 + 500_000,
        get_token_account(&mut program_test_context, &user_redeemable)
            .await?
            .amount
    );

    // Redeem using identity depository
    crate::integration_tests::program_uxd::instructions::process_redeem_from_identity_depository(
        &mut program_test_context,
        &payer,
        &user,
        &user_collateral,
        &user_redeemable,
        250_000,
    )
    .await?;

    // Check user collateral increased
    assert_eq!(
        1_000_000 - 500_000 + 250_000,
        get_token_account(&mut program_test_context, &user_collateral)
            .await?
            .amount
    );
    // Check user redeemable decreased
    assert_eq!(
        0 + 500_000 - 250_000,
        get_token_account(&mut program_test_context, &user_redeemable)
            .await?
            .amount
    );

    Ok(())
}
