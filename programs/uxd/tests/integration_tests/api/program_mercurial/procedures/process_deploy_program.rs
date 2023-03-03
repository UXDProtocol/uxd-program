use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_deploy_program(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_mercurial::accounts::ProgramKeys,
) -> Result<(), program_test_context::ProgramTestError> {
    // Airdrop funds to the mercurial admin wallet (acting as payer)
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &program_keys.admin.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create the lp mint
    program_spl::instructions::process_token_mint_init(
        program_test_context,
        &program_keys.admin,
        &program_keys.lp_mint,
        program_keys.lp_mint_decimals,
        &program_keys.vault,
    )
    .await?;

    // Create the fee_vault, which is the treasury ATA
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        &program_keys.admin,
        &program_keys.lp_mint.pubkey(),
        &program_keys.treasury,
    )
    .await?;

    // Vault init
    program_mercurial::instructions::process_initialize(program_test_context, program_keys).await?;

    // Ready to use
    Ok(())
}
