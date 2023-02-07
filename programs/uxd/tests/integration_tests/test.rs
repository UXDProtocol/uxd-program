use crate::integration_tests::execute_instruction::execute_instruction_initialize_controller;
use crate::integration_tests::program_test_context::program_test_context_execute_instruction_with_signer;
use crate::integration_tests::program_test_context::program_test_context_lamports_airdrop;
use crate::integration_tests::program_test_context::program_test_context_lamports_transfer;
use solana_program_test::processor;
use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

const ROOT: usize = 0;
const PAYER: usize = 1;
const AUTHORITY: usize = 2;

#[tokio::test]
async fn test_integration() -> Result<(), String> {
    let redeemable_mint_decimals = 6;

    let program_test = ProgramTest::new("uxd", uxd::ID, processor!(uxd::entry));

    let mut program_test_context: ProgramTestContext = program_test.start_with_context().await;

    let keypairs = [
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
    ];

    for keypair in &keypairs {
        program_test_context_lamports_airdrop(
            &mut program_test_context,
            &keypair.pubkey(),
            1_000_000_000_000,
        )
        .await?;
    }

    /*
    let (collateral_mint_key, collateral_mint) =
        program_test_add_mint(&mut program_test, None, 6, &master_key.pubkey());

    let (uxp_mint_key, uxp_mint) =
        program_test_add_mint(&mut program_test, None, 9, &master_key.pubkey());
     */

    let success2 = execute_instruction_initialize_controller(
        &mut program_test_context,
        &keypairs[AUTHORITY],
        &keypairs[AUTHORITY],
        redeemable_mint_decimals,
    )
    .await;

    assert_eq!(success2.is_ok(), true, "inited controller");

    Ok(())
}
