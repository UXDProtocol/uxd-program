use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;

pub async fn process_deploy_program(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    admin: &Keypair,
    token_mint: &Pubkey,
    lp_mint: &Keypair,
    lp_mint_decimals: u8,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let base = program_mercurial::accounts::find_base();
    let vault = program_mercurial::accounts::find_vault_pda(token_mint, &base.pubkey()).0;
    let treasury = program_mercurial::accounts::find_treasury();

    // Create the lp mint
    program_spl::instructions::process_token_mint_init(
        program_context,
        admin,
        lp_mint,
        lp_mint_decimals,
        &vault,
    )
    .await?;

    // Create the fee_vault, which is the treasury ATA
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_context,
        admin,
        &lp_mint.pubkey(),
        &treasury,
    )
    .await?;

    // Vault initialize
    program_mercurial::instructions::process_initialize(
        program_context,
        admin,
        token_mint,
        &lp_mint.pubkey(),
    )
    .await?;

    // Ready to use
    Ok(())
}
