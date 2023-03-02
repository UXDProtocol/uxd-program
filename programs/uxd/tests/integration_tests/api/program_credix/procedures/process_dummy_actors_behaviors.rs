use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_dummy_actors_behaviors(
    program_test_context: &mut ProgramTestContext,
    program_info: &program_credix::accounts::ProgramInfo,
    base_token_authority: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    // Create a dummy investor
    let dummy_investor = Keypair::new();
    let dummy_investor_pass = credix_client::CredixPass::generate_pda(
        program_info.global_market_state,
        dummy_investor.pubkey(),
    )
    .0;
    let dummy_investor_token_account = spl_associated_token_account::get_associated_token_address(
        &dummy_investor.pubkey(),
        &program_info.base_token_mint,
    );
    let dummy_investor_lp_token_account =
        spl_associated_token_account::get_associated_token_address(
            &dummy_investor.pubkey(),
            &program_info.lp_token_mint,
        );

    // Airdrop lamports to the dummy investor wallet
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &dummy_investor.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Give some collateral (base token) to our dummy investor and create its token account
    program_spl::instructions::process_associated_token_account_get_or_init(
        program_test_context,
        &dummy_investor,
        &program_info.base_token_mint,
        &dummy_investor.pubkey(),
    )
    .await?;
    program_spl::instructions::process_token_mint_to(
        program_test_context,
        &dummy_investor,
        &program_info.base_token_mint,
        base_token_authority,
        &dummy_investor_token_account,
        1_000_000_000,
    )
    .await?;

    // Create the credix-pass for the dummy investor
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
        &program_info,
        &dummy_investor.pubkey(),
        &dummy_investor_pass,
        true,
        false,
        0,
        false,
    )
    .await?;
    // The dummy investor will do a dummy deposit to initialize the lp-pool
    program_credix::instructions::process_deposit_funds(
        program_test_context,
        &program_info,
        &dummy_investor,
        &dummy_investor_pass,
        &dummy_investor_token_account,
        &dummy_investor_lp_token_account,
        1_000_000,
    )
    .await?;

    return Ok(());
}
