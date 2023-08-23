use solana_sdk::pubkey::Pubkey;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_deploy_program(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    let market_seeds = program_credix::accounts::find_market_seeds();
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let treasury = program_credix::accounts::find_treasury(&multisig.pubkey());

    // Airdrop funds to the credix authority wallet, the multisig (acting as payer)
    program_spl::instructions::process_lamports_airdrop(
        program_runner,
        &multisig.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create associated token accounts for the authorities wallets
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_runner,
        multisig,
        base_token_mint,
        &signing_authority,
    )
    .await?;
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_runner,
        multisig,
        base_token_mint,
        &treasury,
    )
    .await?;

    // Initialize the program state
    program_credix::instructions::process_initialize_program_state(program_runner, multisig)
        .await?;

    // Initialize the global market state
    program_credix::instructions::process_initialize_market(
        program_runner,
        multisig,
        base_token_mint,
    )
    .await?;

    // Turn on the withdrawal epochs
    program_credix::instructions::process_update_global_market_state(
        program_runner,
        multisig,
        true,
    )
    .await?;

    // Ready to use
    Ok(())
}
