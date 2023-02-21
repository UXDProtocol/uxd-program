use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;

pub async fn process_program_setup_init(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    program_setup: &program_credix::accounts::ProgramSetup,
) -> Result<(), String> {
    // Airdrop funds to the credix owner wallet
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &program_setup.owner.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Global init
    program_credix::instructions::process_initialize_program_state(
        program_test_context,
        payer,
        &program_setup,
    )
    .await?;
    /*
    program_credix::instructions::process_initialize_market(
        program_test_context,
        payer,
        &program_setup,
    )
    .await?;
     */

    // TODO - make it work for the rest of the credix setup

    // Ready to use
    Ok(())
}
