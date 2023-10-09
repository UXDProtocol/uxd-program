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
    // Create the vault mint
    program_spl::instructions::process_token_mint_init(
        program_context,
        authority,
        alloyx_vault_mint,
        alloyx_vault_mint_decimals,
        &authority.pubkey(),
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
