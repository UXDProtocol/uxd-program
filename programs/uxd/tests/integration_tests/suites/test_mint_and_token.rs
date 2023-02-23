use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;

#[tokio::test]
async fn test_mint_and_token() -> Result<(), String> {
    let program_test = ProgramTest::default();

    let mut program_test_context = program_test.start_with_context().await;

    let payer = Keypair::new();

    let mint = Keypair::new();
    let authority = Keypair::new();
    let user1 = Keypair::new();
    let user2 = Keypair::new();

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

    // create user token accounts
    let user1_token_account = program_spl::instructions::process_associated_token_account_init(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &user1.pubkey(),
    )
    .await?;
    let user2_token_account = program_spl::instructions::process_associated_token_account_init(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &user2.pubkey(),
    )
    .await?;

    // mint some
    program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &authority,
        &user1_token_account,
        41,
    )
    .await?;

    // mint more
    program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &mint.pubkey(),
        &authority,
        &user1_token_account,
        42,
    )
    .await?;

    // transfer to other user
    program_spl::instructions::process_token_transfer(
        &mut program_test_context,
        &payer,
        &user1,
        &user1_token_account,
        &user2_token_account,
        13,
    )
    .await?;

    // Check user 1 should have 41+42-13 = 70
    let user1_token_amount =
        program_spl::accounts::read_token_account(&mut program_test_context, &user1_token_account)
            .await?
            .amount;
    assert_eq!(user1_token_amount, 70);

    // Check user 2 should have 13
    let user2_token_amount =
        program_spl::accounts::read_token_account(&mut program_test_context, &user2_token_account)
            .await?
            .amount;
    assert_eq!(user2_token_amount, 13);

    Ok(())
}
