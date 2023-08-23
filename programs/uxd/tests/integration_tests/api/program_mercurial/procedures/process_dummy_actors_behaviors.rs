use solana_program::pubkey::Pubkey;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_dummy_actors_behaviors(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    token_mint: &Keypair,
    lp_mint: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    let dummy_investor = Keypair::new();

    // Airdrop lamports to the dummy investor wallet
    program_spl::instructions::process_lamports_airdrop(
        program_runner,
        &dummy_investor.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create dummy investor ATAs
    let dummy_investor_token =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_runner,
            &dummy_investor,
            &token_mint.pubkey(),
            &dummy_investor.pubkey(),
        )
        .await?;
    let dummy_investor_lp =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_runner,
            &dummy_investor,
            lp_mint,
            &dummy_investor.pubkey(),
        )
        .await?;

    // Airdrop some token to our dummy investor
    program_spl::instructions::process_token_mint_to(
        program_runner,
        &dummy_investor,
        &token_mint.pubkey(),
        token_mint,
        &dummy_investor_token,
        1_000_000_000,
    )
    .await?;

    // The dummy investor will do a dummy deposit to initialize the lp-pool
    program_mercurial::instructions::process_deposit(
        program_runner,
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
