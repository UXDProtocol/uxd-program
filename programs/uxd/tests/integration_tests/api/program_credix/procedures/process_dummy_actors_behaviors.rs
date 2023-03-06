use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_dummy_actors_behaviors(
    program_test_context: &mut ProgramTestContext,
    authority: &Keypair,
    base_token_mint: &Pubkey,
    base_token_authority: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state = program_credix::accounts::find_global_market_state(&market_seeds);
    let lp_token_mint = program_credix::accounts::find_lp_token_mint(&market_seeds);

    let dummy_investor = Keypair::new();

    // Airdrop lamports to the dummy investor wallet
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &dummy_investor.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create the investor ATAs
    let dummy_investor_token_account =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_test_context,
            &dummy_investor,
            &base_token_mint,
            &dummy_investor.pubkey(),
        )
        .await?;
    let dummy_investor_lp_token_account =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_test_context,
            &dummy_investor,
            &lp_token_mint,
            &dummy_investor.pubkey(),
        )
        .await?;

    // Give some collateral (base token) to our dummy investor and create its token account
    program_spl::instructions::process_token_mint_to(
        program_test_context,
        &dummy_investor,
        &base_token_mint,
        base_token_authority,
        &dummy_investor_token_account,
        1_000_000_000,
    )
    .await?;

    // Create the credix-pass for the dummy investor
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
        authority,
        &dummy_investor.pubkey(),
        true,
        false,
        0,
        false,
    )
    .await?;

    // The dummy investor will do a dummy deposit to initialize the lp-pool
    program_credix::instructions::process_deposit_funds(
        program_test_context,
        authority,
        base_token_mint,
        &dummy_investor,
        &dummy_investor_token_account,
        &dummy_investor_lp_token_account,
        1_000_000,
    )
    .await?;

    // Done
    Ok(())
}
