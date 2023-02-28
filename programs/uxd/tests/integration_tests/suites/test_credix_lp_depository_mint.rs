use solana_program::pubkey::Pubkey;
use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_credix_lp_depository_mint() -> Result<(), String> {
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
        6,
        1_000_000,
    )
    .await?;

    // Enable minting by editing credix depository configuration
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &program_keys,
        &payer,
        Some(1_000_000),
        Some(0),
        Some(0),
        Some(false),
        Some(Pubkey::default()),
    )
    .await?;

    // Main actor
    let user = Keypair::new();

    // Give some collateral to our user and create its account
    let user_collateral = program_spl::instructions::process_associated_token_account_init(
        &mut program_test_context,
        &payer,
        &program_keys.collateral_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;
    program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &program_keys.collateral_mint.pubkey(),
        &program_keys.collateral_authority,
        &user_collateral,
        1_000_000,
    )
    .await?;

    // Create a redeemable account for our user
    let user_redeemable = program_spl::instructions::process_associated_token_account_init(
        &mut program_test_context,
        &payer,
        &program_keys.redeemable_mint,
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

    // Mint using credix depository
    program_uxd::instructions::process_mint_with_credix_lp_depository(
        &mut program_test_context,
        &program_keys,
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

    Ok(())
}
