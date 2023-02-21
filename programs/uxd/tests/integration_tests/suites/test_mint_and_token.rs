use solana_program_test::tokio;
use solana_program_test::ProgramTestContext;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;

#[tokio::test]
async fn test_mint_and_token() -> Result<(), String> {
    let program_test = ProgramTest::default();
    let mut program_test_context: ProgramTest = program_test.start_with_context().await;

    let payer = Keypair::new();

    let mint = Keypair::new();
    let authority = Keypair::new();

    // Fund payer
    program_spl::instructions::process_lamports_airdrop(
        &mut program_test_context,
        &payer.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // create mint
    program_spl::instructions::process_token_mint_init(
        &mut program_test_context,
        &payer,
        &mint,
        6,
        &authority.pubkey(),
    )
    .await?;

    // create account
    let wallet = Keypair::new();
    let token_account = program_spl::instructions::process_associated_token_account_init(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &wallet.pubkey(),
    )
    .await?;

    // mint some
    program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &authority,
        &token_account,
        41,
    )
    .await?;

    // mint more
    program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &authority,
        &token_account,
        42,
    )
    .await?;

    // check total minted amount
    let token_account =
        program_spl::accounts::read_token_account(&mut program_test_context, &token_account)
            .await?;
    assert_eq!(token_account.amount, 83);

    Ok(())
}
