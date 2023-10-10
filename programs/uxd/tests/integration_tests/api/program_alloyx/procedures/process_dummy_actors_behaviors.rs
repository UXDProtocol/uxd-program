use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_spl;

pub async fn process_dummy_actors_behaviors(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    authority: &Keypair,
    collateral_mint: &Keypair,
    alloyx_vault_mint: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    let dummy_investor = Keypair::new();

    // Airdrop lamports to the dummy investor wallet
    program_context
        .process_airdrop(&dummy_investor.pubkey(), 1_000_000_000_000)
        .await?;

    // Create dummy investor ATAs
    let dummy_investor_collateral =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_context,
            &dummy_investor,
            &collateral_mint.pubkey(),
            &dummy_investor.pubkey(),
        )
        .await?;
    let dummy_investor_alloyx =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_context,
            &dummy_investor,
            alloyx_vault_mint,
            &dummy_investor.pubkey(),
        )
        .await?;

    // Airdrop some collateral to our dummy investor
    program_spl::instructions::process_token_mint_to(
        program_context,
        &dummy_investor,
        &collateral_mint.pubkey(),
        collateral_mint,
        &dummy_investor_collateral,
        1_000_000_000,
    )
    .await?;

    // Whitelist our dummy investor
    program_alloyx::instructions::process_whitelist(
        program_context,
        authority,
        &dummy_investor.pubkey(),
    )
    .await?;

    // The dummy investor will do a dummy deposit to initialize the vault
    program_alloyx::instructions::process_deposit(
        program_context,
        &collateral_mint.pubkey(),
        alloyx_vault_mint,
        &dummy_investor,
        &dummy_investor_collateral,
        &dummy_investor_alloyx,
        1_000_000_000,
    )
    .await?;

    // Done
    Ok(())
}
