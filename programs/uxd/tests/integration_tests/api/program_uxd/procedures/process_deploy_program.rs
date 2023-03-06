use credix_client::credix;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_deploy_program(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Keypair,
    credix_authority: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let collateral_mint_decimals = 6;
    let redeemable_mint_decimals = 6;

    // Use restictive default values for all tests
    // Can be modified in individual test cases through edits
    // This forces all tests be explicit about their requirements
    let redeemable_global_supply_cap = 0;
    let identity_depository_redeemable_amount_under_management_cap = 0;
    let identity_depository_minting_disabled = true;
    let credix_lp_depository_redeemable_amount_under_management_cap = 0;
    let credix_lp_depository_minting_fee_in_bps = 255;
    let credix_lp_depository_redeeming_fee_in_bps = 255;
    let credix_lp_depository_minting_disabled = true;
    let credix_lp_depository_profits_beneficiary_collateral = Pubkey::default();

    // Create the collateral mint
    program_spl::instructions::process_token_mint_init(
        program_test_context,
        &payer,
        collateral_mint,
        collateral_mint_decimals,
        &collateral_mint.pubkey(),
    )
    .await?;

    // Controller setup
    program_uxd::instructions::process_initialize_controller(
        program_test_context,
        payer,
        authority,
        redeemable_mint_decimals,
    )
    .await?;
    program_uxd::instructions::process_edit_controller(
        program_test_context,
        payer,
        authority,
        Some(redeemable_global_supply_cap),
    )
    .await?;

    // Identity depository setup
    program_uxd::instructions::process_initialize_identity_depository(
        program_test_context,
        payer,
        authority,
        &collateral_mint.pubkey(),
    )
    .await?;
    program_uxd::instructions::process_edit_identity_depository(
        program_test_context,
        payer,
        authority,
        Some(identity_depository_redeemable_amount_under_management_cap),
        Some(identity_depository_minting_disabled),
    )
    .await?;

    // Credix onchain dependency program deployment
    program_credix::procedures::process_deploy_program(
        program_test_context,
        &credix_authority,
        &collateral_mint.pubkey(),
    )
    .await?;
    program_credix::procedures::process_dummy_actors_behaviors(
        program_test_context,
        &credix_authority,
        &collateral_mint.pubkey(),
        collateral_mint,
    )
    .await?;

    // Credix lp depository setup
    program_uxd::instructions::process_register_credix_lp_depository(
        program_test_context,
        payer,
        authority,
        &collateral_mint.pubkey(),
        0,
        0,
        0,
    )
    .await?;
    program_uxd::instructions::process_edit_credix_lp_depository(
        program_test_context,
        payer,
        authority,
        &collateral_mint.pubkey(),
        Some(credix_lp_depository_redeemable_amount_under_management_cap),
        Some(credix_lp_depository_minting_fee_in_bps),
        Some(credix_lp_depository_redeeming_fee_in_bps),
        Some(credix_lp_depository_minting_disabled),
        Some(credix_lp_depository_profits_beneficiary_collateral),
    )
    .await?;

    // Credix pass creation
    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        &program_credix::accounts::find_global_market_state(&credix_market_seeds);
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository(
        &collateral_mint.pubkey(),
        &credix_global_market_state,
    );
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
        authority,
        &credix_lp_depository,
        true,
        false,
        0,
        true,
    )
    .await?;

    // TODO - initialize mercurial too here

    // Done
    Ok(())
}
