use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_deploy_market(
    program_test_context: &mut ProgramTestContext,
    market_seeds: &String,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    let signing_authority = program_credix::accounts::find_signing_authority_pda(market_seeds).0;

    // Create associated token accounts for the authorities wallets
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        multisig,
        base_token_mint,
        &signing_authority,
    )
    .await?;

    // Initialize the global market state
    program_credix::instructions::process_initialize_market(
        program_test_context,
        market_seeds,
        multisig,
        base_token_mint,
    )
    .await?;

    // Turn on the withdrawal epochs
    program_credix::instructions::process_update_global_market_state(
        program_test_context,
        market_seeds,
        multisig,
        true,
    )
    .await?;

    // Ready to use
    Ok(())
}
