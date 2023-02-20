use solana_program_test::processor;
use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;

const PAYER: usize = 0;
const UXD_AUTHORITY: usize = 1;
const CREDIX_AUTHORITY: usize = 3;

#[tokio::test]
async fn test_credix() -> Result<(), String> {
    let keypairs = [
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
    ];

    let mut program_test = ProgramTest::default();

    program_test.add_program("uxd", uxd::id(), processor!(uxd::entry));

    program_test.prefer_bpf(true);
    program_test.add_program(
        "tests/integration_tests/api/program_credix/binaries/devnet",
        credix_client::id(),
        None,
    );

    let mut program_test_context: ProgramTestContext = program_test.start_with_context().await;

    for keypair in &keypairs {
        program_spl::instructions::process_lamports_airdrop(
            &mut program_test_context,
            &keypair.pubkey(),
            1_000_000_000_000,
        )
        .await?;
    }

    let collateral_mint = Keypair::new();
    program_spl::instructions::process_token_mint_init(
        &mut program_test_context,
        &keypairs[PAYER],
        &collateral_mint,
        6,
        &keypairs[UXD_AUTHORITY].pubkey(),
    )
    .await?;

    program_credix::procedures::run_basic_setup(
        &mut program_test_context,
        &keypairs[CREDIX_AUTHORITY],
        &collateral_mint.pubkey(),
    )
    .await?;

    program_uxd::procedures::run_basic_setup(
        &mut program_test_context,
        &keypairs[PAYER],
        &keypairs[UXD_AUTHORITY],
        &collateral_mint.pubkey(),
        6,
        1_000_000,
        1_000_000,
        false,
    )
    .await?;

    Ok(())
}
