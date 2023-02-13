use solana_program_test::processor;
use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

const PAYER: usize = 0;
const UXD_AUTHORITY: usize = 1;
const CREDIX_ADMIN: usize = 3;
const CREDIX_MULTISIG: usize = 4;
const USDC: usize = 5;
const DUMMY: usize = 6;

#[tokio::test]
async fn test_integration() -> Result<(), String> {
    let keypairs = [
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
    ];

    let redeemable_collateral_mint_decimals = 6;

    let mut program_test = ProgramTest::default();
    program_test.add_program("uxd", uxd::id(), processor!(uxd::entry));

    /*
    program_test.add_account_with_file_data(
        Pubkey::from_str("GMiQHsRQpdbgaRA3Y2SJUxC7wBvBoCFpKFHCnMHM4f8a").unwrap(),
        1_000_000_000_000,
        keypairs[CREDIX_ADMIN].pubkey(),
        "tests/fixtures/credix_client.so",
    );
     */
    program_test.prefer_bpf(true);
    program_test.add_program("credix_client", credix_client::id(), None);

    let mut program_test_context: ProgramTestContext = program_test.start_with_context().await;

    for keypair in &keypairs {
        crate::integration_tests::program_spl::instructions::process_lamports_airdrop(
            &mut program_test_context,
            &keypair.pubkey(),
            1_000_000_000_000,
        )
        .await?;
    }

    let collateral_mint = Keypair::new();
    crate::integration_tests::program_spl::instructions::process_token_mint_init(
        &mut program_test_context,
        &keypairs[PAYER],
        &collateral_mint,
        6,
        &keypairs[UXD_AUTHORITY].pubkey(),
    )
    .await?;

    crate::integration_tests::program_credix::instructions::process_initialize_program_state(
        &mut program_test_context,
        &keypairs[CREDIX_ADMIN],
    )
    .await?;

    /*
    crate::integration_tests::program_credix::instructions::process_initialize_market(
        &mut program_test_context,
        &keypairs[CREDIX_ADMIN],
        &collateral_mint.pubkey(),
    )
    .await?;
     */

    crate::integration_tests::program_uxd::instructions::process_initialize_controller(
        &mut program_test_context,
        &keypairs[PAYER],
        &keypairs[UXD_AUTHORITY],
        redeemable_collateral_mint_decimals,
    )
    .await?;

    crate::integration_tests::program_uxd::instructions::process_initialize_identity_depository(
        &mut program_test_context,
        &keypairs[PAYER],
        &keypairs[UXD_AUTHORITY],
        &collateral_mint.pubkey(),
    )
    .await?;

    Ok(())
}
