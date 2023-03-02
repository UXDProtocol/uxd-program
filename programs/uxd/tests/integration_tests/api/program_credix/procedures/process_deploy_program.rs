use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_deploy_program(
    program_test_context: &mut ProgramTestContext,
    program_info: &program_credix::accounts::ProgramInfo,
) -> Result<(), program_test_context::ProgramTestError> {
    // Airdrop funds to the credix authority wallet (acting as payer)
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &program_info.authority.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create associated token accounts for the authorities wallets
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        &program_info.authority,
        &program_info.base_token_mint,
        &program_info.signing_authority,
    )
    .await?;
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        &program_info.authority,
        &program_info.base_token_mint,
        &program_info.treasury,
    )
    .await?;

    // Global init
    program_credix::instructions::process_initialize_program_state(
        program_test_context,
        &program_info,
    )
    .await?;
    program_credix::instructions::process_initialize_market(program_test_context, &program_info)
        .await?;

    // Ready to use
    Ok(())
}
