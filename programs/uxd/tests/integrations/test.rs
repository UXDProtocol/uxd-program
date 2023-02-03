use crate::integrations::program_test_add_mint;
use solana_program_test::processor;
use solana_program_test::tokio;
use solana_program_test::ProgramTest;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

#[tokio::test]
async fn test_integration() {
    let mut program_test = ProgramTest::new("uxd", uxd::ID, processor!(uxd::entry));

    let master_key = Keypair::new();

    let (usdc_mint_key, usdc_mint) =
        program_test_add_mint(&mut program_test, None, 6, &master_key.pubkey());
    let (uxp_mint_key, uxp_mint) =
        program_test_add_mint(&mut program_test, None, 9, &master_key.pubkey());

    assert_eq!("this should fail all the time", "nice");
}
