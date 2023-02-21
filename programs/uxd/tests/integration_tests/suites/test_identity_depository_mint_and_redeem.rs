use solana_program_test::processor;
use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_identity_depository() -> Result<(), String> {
    let mut program_test = ProgramTest::default();

    program_test.add_program("uxd", uxd::id(), processor!(uxd::entry));

    let mut program_test_context = program_test.start_with_context().await;

    // Fund payer
    let payer = Keypair::new();
    program_spl::instructions::process_lamports_airdrop(
        &mut program_test_context,
        &payer.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Main actor
    let user = Keypair::new();

    // Create the collateral mint
    let collateral_authority = Keypair::new();
    let collateral_mint = Keypair::new();
    program_spl::instructions::process_token_mint_init(
        &mut program_test_context,
        &payer,
        &collateral_mint,
        6,
        &collateral_authority.pubkey(),
    )
    .await?;

    // Give some collateral to our user and create its account
    let user_collateral = program_spl::instructions::process_associated_token_account_init(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;
    program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &collateral_authority,
        &user_collateral,
        1_000_000,
    )
    .await?;

    // Create the program setup structure (find/create all important keys)
    let program_setup = program_uxd::accounts::create_program_setup(&collateral_mint.pubkey());

    // Initialize basic UXD program state
    program_uxd::procedures::process_program_setup_init(
        &mut program_test_context,
        &payer,
        &program_setup,
        6,
        1_000_000,
        1_000_000,
        false,
        1,
        1,
        1_000_000,
    )
    .await?;

    // Create a redeemable account for our user
    let user_redeemable = program_spl::instructions::process_associated_token_account_init(
        &mut program_test_context,
        &payer,
        &program_setup.redeemable_mint,
        &user.pubkey(),
    )
    .await?;

    // Check user collateral original amount
    assert_eq!(
        1_000_000,
        program_spl::accounts::read_token_account(&mut program_test_context, &user_collateral)
            .await?
            .amount
    );
    // Check user redeemable original amount
    assert_eq!(
        0,
        program_spl::accounts::read_token_account(&mut program_test_context, &user_redeemable)
            .await?
            .amount
    );

    // Mint using identity depository
    program_uxd::instructions::process_mint_with_identity_depository(
        &mut program_test_context,
        &payer,
        &program_setup,
        &user,
        &user_collateral,
        &user_redeemable,
        500_000,
    )
    .await?;

    // Check user collateral decreased
    assert_eq!(
        1_000_000 - 500_000,
        program_spl::accounts::read_token_account(&mut program_test_context, &user_collateral)
            .await?
            .amount
    );
    // Check user redeemable increased
    assert_eq!(
        0 + 500_000,
        program_spl::accounts::read_token_account(&mut program_test_context, &user_redeemable)
            .await?
            .amount
    );

    // Redeem using identity depository
    program_uxd::instructions::process_redeem_from_identity_depository(
        &mut program_test_context,
        &payer,
        &program_setup,
        &user,
        &user_collateral,
        &user_redeemable,
        250_000,
    )
    .await?;

    // Check user collateral increased
    assert_eq!(
        1_000_000 - 500_000 + 250_000,
        program_spl::accounts::read_token_account(&mut program_test_context, &user_collateral)
            .await?
            .amount
    );
    // Check user redeemable decreased
    assert_eq!(
        0 + 500_000 - 250_000,
        program_spl::accounts::read_token_account(&mut program_test_context, &user_redeemable)
            .await?
            .amount
    );

    Ok(())
}
