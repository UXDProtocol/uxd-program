use crate::integration_tests::create_instruction::create_instruction_initialize_controller;
use crate::integration_tests::program_test_utils::program_test_add_account_with_lamports;
use crate::integration_tests::program_test_utils::program_test_add_mint;
use crate::integration_tests::program_test_utils::program_test_context_execute_instruction_with_signer;
use crate::integration_tests::program_test_utils::program_test_context_transfer_lamports;
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
async fn test_integration() {
    let redeemable_mint_decimals = 6;

    let mut program_test = ProgramTest::new("uxd", uxd::ID, processor!(uxd::entry));

    let keypairs = [
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
    ];

    keypairs.iter().for_each(|keypair| {
        program_test_add_account_with_lamports(
            &mut program_test,
            &keypair.pubkey(),
            1_000_000_000_000,
        );
    });

    /*
    let (collateral_mint_key, collateral_mint) =
        program_test_add_mint(&mut program_test, None, 6, &master_key.pubkey());

    let (uxp_mint_key, uxp_mint) =
        program_test_add_mint(&mut program_test, None, 9, &master_key.pubkey());
     */

    // Start and process transactions on the test network
    let mut program_test_context: ProgramTestContext = program_test.start_with_context().await;

    let instruction_initialize_controller = create_instruction_initialize_controller(
        &keypairs[AUTHORITY],
        &keypairs[AUTHORITY],
        redeemable_mint_decimals,
    );
    let success2 = program_test_context_execute_instruction_with_signer(
        &mut program_test_context,
        instruction_initialize_controller,
        &keypairs[AUTHORITY],
        &keypairs[AUTHORITY],
    )
    .await;

    assert_eq!(success2, true, "inited controller");
}
