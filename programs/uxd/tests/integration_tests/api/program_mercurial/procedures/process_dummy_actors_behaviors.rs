use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;

pub async fn process_dummy_actors_behaviors(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    token_mint: &Keypair,
    lp_mint: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    let dummy_investor = Keypair::new();

    // Airdrop lamports to the dummy investor wallet
    program_context
        .process_airdrop(&dummy_investor.pubkey(), 1_000_000_000_000)
        .await?;

    // Create dummy investor ATAs
    let dummy_investor_token =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_context,
            &dummy_investor,
            &token_mint.pubkey(),
            &dummy_investor.pubkey(),
        )
        .await?;
    let dummy_investor_lp =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_context,
            &dummy_investor,
            lp_mint,
            &dummy_investor.pubkey(),
        )
        .await?;

    // Airdrop some token to our dummy investor
    program_spl::instructions::process_token_mint_to(
        program_context,
        &dummy_investor,
        &token_mint.pubkey(),
        token_mint,
        &dummy_investor_token,
        1_000_000_000,
    )
    .await?;

    // The dummy investor will do a dummy deposit to initialize the lp-pool
    program_mercurial::instructions::process_deposit(
        program_context,
        &token_mint.pubkey(),
        lp_mint,
        &dummy_investor,
        &dummy_investor_token,
        &dummy_investor_lp,
        1_000_000,
    )
    .await?;

    // Done
    Ok(())
}
