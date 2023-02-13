use solana_program_test::processor;
use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

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

    // Mint
    crate::integration_tests::program_uxd::instructions::process_mint_with_identity_depository(
        &mut program_test_context,
        &payer,
        &user,
        &collateral_mint.pubkey(),
        500_000,
    )
    .await?;
    /*
     */

    Ok(())
}
