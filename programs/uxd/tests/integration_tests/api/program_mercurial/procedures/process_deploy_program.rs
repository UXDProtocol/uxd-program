use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_deploy_program(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_mercurial::accounts::ProgramKeys,
) -> Result<(), program_test_context::ProgramTestError> {
    // Airdrop funds to the mercurial authority and admin wallet (acting as payer)
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &program_keys.authority.pubkey(),
        1_000_000_000_000,
    )
    .await?;
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &program_keys.admin.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create associated token accounts for the authority and admin wallets
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        &program_keys.authority,
        &program_keys.token_mint,
        &program_keys.authority.pubkey(),
    )
    .await?;
program_spl::instructions::process_associated_token_account_get_or_init(
    program_test_context,
    &program_keys.admin,
    &program_keys.token_mint,
    &program_keys.admin.pubkey(),
)
.await?;

    // Create the lp mint
    program_spl::instructions::process_token_mint_init(
        program_test_context,
        &program_keys.authority,
        &program_keys.lp_mint,
        program_keys.lp_mint_decimals,
        &program_keys.authority.pubkey(),
    )
    .await?;

    // Vault init
    program_mercurial::instructions::process_initialize(program_test_context, program_keys).await?;

    // TODO - make it work for the rest of the mercurial keys

    // Ready to use
    Ok(())
}
