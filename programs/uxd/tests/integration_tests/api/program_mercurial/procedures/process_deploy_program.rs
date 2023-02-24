use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;

pub async fn process_deploy_program(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_mercurial::accounts::ProgramKeys,
) -> Result<(), String> {
    // Airdrop funds to the mercurial authority wallet (acting as payer)
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &program_keys.authority.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Global init

    // TODO - make it work for the rest of the mercurial keys

    // Ready to use
    Ok(())
}
