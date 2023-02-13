use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::program_credix;

pub async fn run_basic_setup(
    program_test_context: &mut ProgramTestContext,
    authority: &Keypair,
    collateral_mint: &Pubkey,
) -> Result<(), String> {
    // Global init
    program_credix::instructions::process_initialize_program_state(program_test_context, authority)
        .await?;
    program_credix::instructions::process_initialize_market(
        program_test_context,
        authority,
        collateral_mint,
    )
    .await?;

    // Ready to use
    Ok(())
}
