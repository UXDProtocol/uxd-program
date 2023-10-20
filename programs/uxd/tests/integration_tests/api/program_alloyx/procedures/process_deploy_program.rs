use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_spl;

pub async fn process_deploy_program(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    alloyx_vault_mint: &Keypair,
    alloyx_vault_mint_decimals: u8,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let vault_id = program_alloyx::accounts::find_vault_id();
    let vault_alloyx_token = program_alloyx::accounts::find_vault_alloyx_token(&vault_id).0;

    // Create the vault mint
    program_spl::instructions::process_token_mint_init(
        program_context,
        authority,
        alloyx_vault_mint,
        alloyx_vault_mint_decimals,
        &vault_alloyx_token,
    )
    .await?;
    // Vault initialize
    program_alloyx::instructions::process_initialize(
        program_context,
        authority,
        collateral_mint,
        &alloyx_vault_mint.pubkey(),
    )
    .await?;
    // Ready to use
    Ok(())
}
