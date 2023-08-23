use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;

pub async fn process_deploy_program(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    let market_seeds = program_credix::accounts::find_market_seeds();
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let treasury = program_credix::accounts::find_treasury(&multisig.pubkey());

    // Create associated token accounts for the authorities wallets
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_context,
        multisig,
        base_token_mint,
        &signing_authority,
    )
    .await?;
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_context,
        multisig,
        base_token_mint,
        &treasury,
    )
    .await?;

    // Initialize the program state
    program_credix::instructions::process_initialize_program_state(program_context, multisig)
        .await?;

    // Initialize the global market state
    program_credix::instructions::process_initialize_market(
        program_context,
        multisig,
        base_token_mint,
    )
    .await?;

    // Turn on the withdrawal epochs
    program_credix::instructions::process_update_global_market_state(
        program_context,
        multisig,
        true,
    )
    .await?;

    // Ready to use
    Ok(())
}
