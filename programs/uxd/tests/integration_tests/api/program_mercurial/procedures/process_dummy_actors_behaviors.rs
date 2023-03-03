use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_dummy_actors_behaviors(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_mercurial::accounts::ProgramKeys,
    token_mint_authority: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    // Create a dummy investor
    let dummy_investor = Keypair::new();

    // Airdrop lamports to the dummy investor wallet
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &dummy_investor.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create dummy investor ATAs
    let dummy_investor_token =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_test_context,
            &dummy_investor,
            &program_keys.token_mint,
            &dummy_investor.pubkey(),
        )
        .await?;
    let dummy_investor_lp =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_test_context,
            &dummy_investor,
            &program_keys.lp_mint.pubkey(),
            &dummy_investor.pubkey(),
        )
        .await?;

    // Airdrop some token to our dummy investor
    program_spl::instructions::process_token_mint_to(
        program_test_context,
        &dummy_investor,
        &program_keys.token_mint,
        token_mint_authority,
        &dummy_investor_token,
        1_000_000_000,
    )
    .await?;

    // The dummy investor will do a dummy deposit to initialize the lp-pool
    program_mercurial::instructions::process_deposit(
        program_test_context,
        &program_keys,
        &dummy_investor,
        &dummy_investor_token,
        &dummy_investor_lp,
        1_000_000,
    )
    .await?;

    // Done
    Ok(())
}
