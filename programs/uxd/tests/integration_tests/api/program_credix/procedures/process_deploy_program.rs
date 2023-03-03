use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_deploy_program(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_credix::accounts::ProgramKeys,
) -> Result<(), program_test_context::ProgramTestError> {
    // Airdrop funds to the credix authority wallet (acting as payer)
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &program_keys.authority.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create associated token accounts for the authorities wallets
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        &program_keys.authority,
        &program_keys.base_token_mint,
        &program_keys.signing_authority,
    )
    .await?;
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        &program_keys.authority,
        &program_keys.base_token_mint,
        &program_keys.treasury,
    )
    .await?;

    // Initialize the program state
    program_credix::instructions::process_initialize_program_state(
        program_test_context,
        &program_keys,
    )
    .await?;

    // Initialize the global market state
    program_credix::instructions::process_initialize_market(program_test_context, &program_keys)
        .await?;

    // Ready to use
    Ok(())
}
