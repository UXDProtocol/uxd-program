use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_deploy_program(
    program_test_context: &mut ProgramTestContext,
    authority: &Keypair,
    base_token_mint: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    let market_seeds = program_credix::accounts::find_market_seeds();
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let treasury = program_credix::accounts::find_treasury(authority);

    // Airdrop funds to the credix authority wallet (acting as payer)
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &authority.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create associated token accounts for the authorities wallets
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        authority,
        base_token_mint,
        &signing_authority,
    )
    .await?;
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        authority,
        base_token_mint,
        &treasury,
    )
    .await?;

    // Initialize the program state
    program_credix::instructions::process_initialize_program_state(program_test_context, authority)
        .await?;

    // Initialize the global market state
    program_credix::instructions::process_initialize_market(
        program_test_context,
        authority,
        base_token_mint,
    )
    .await?;

    // Ready to use
    Ok(())
}