use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

#[tokio::test]
async fn test_token_utils() -> Result<(), String> {
    let program_test = ProgramTest::default();
    let mut program_test_context: ProgramTestContext = program_test.start_with_context().await;

    let payer = Keypair::new();

    let mint = Keypair::new();
    let authority = Keypair::new();

    // Fund payer
    crate::integration_tests::program_spl::instructions::process_lamports_airdrop(
        &mut program_test_context,
        &payer.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // create mint
    crate::integration_tests::program_spl::instructions::process_token_mint_init(
        &mut program_test_context,
        &payer,
        &mint,
        6,
        &authority.pubkey(),
    )
    .await?;

    // create account
    let wallet = Keypair::new();
    let token_account =
        crate::integration_tests::program_spl::instructions::process_associated_token_account_init(
            &mut program_test_context,
            &payer,
            &mint.pubkey(),
            &wallet.pubkey(),
        )
        .await?;

    // mint some
    crate::integration_tests::program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &authority,
        &token_account,
        41,
    )
    .await?;

    // mint more
    crate::integration_tests::program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &authority,
        &token_account,
        42,
    )
    .await?;

    // check total minted amount
    let token_account = crate::integration_tests::program_spl::accounts::get_token_account(
        &mut program_test_context,
        &token_account,
    )
    .await?;
    assert_eq!(token_account.amount, 83);

    Ok(())
}
