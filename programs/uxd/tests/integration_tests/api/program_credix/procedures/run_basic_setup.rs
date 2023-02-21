use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;

pub async fn run_basic_setup(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    collateral_mint: &Pubkey,
) -> Result<program_credix::accounts::Context, String> {
    let context = program_credix::accounts::create_context(collateral_mint);

    // Airdrop funds to the credix owner wallet
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &context.owner.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Global init
    program_credix::instructions::process_initialize_program_state(
        program_test_context,
        payer,
        &context,
    )
    .await?;
    /*
    program_credix::instructions::process_initialize_market(
        program_test_context,
        payer,
        &context,
    )
    .await?;
     */

    // TODO - make it work for the rest of the credix setup

    // Ready to use
    Ok(context)
}
