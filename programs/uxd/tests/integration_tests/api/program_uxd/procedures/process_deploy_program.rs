use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::api::program_test_context;

pub async fn process_deploy_program(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    // Use restictive default values for all tests
    // Can be modified in individual test cases through edits
    // This forces all tests be explicit about their requirements
    let redeemable_mint_decimals = 6;
    let redeemable_global_supply_cap = 0;
    let identity_depository_redeemable_amount_under_management_cap = 0;
    let identity_depository_minting_disabled = true;

    // Create the collateral mint
    program_spl::instructions::process_token_mint_init(
        program_test_context,
        &payer,
        &program_keys.collateral_mint,
        redeemable_mint_decimals,
        &program_keys.collateral_authority.pubkey(),
    )
    .await?;

    // Controller setup
    program_uxd::instructions::process_initialize_controller(
        program_test_context,
        program_keys,
        payer,
        redeemable_mint_decimals,
    )
    .await?;
    program_uxd::instructions::process_edit_controller(
        program_test_context,
        program_keys,
        payer,
        Some(redeemable_global_supply_cap),
    )
    .await?;

    // Identity depository setup
    program_uxd::instructions::process_initialize_identity_depository(
        program_test_context,
        program_keys,
        payer,
    )
    .await?;
    program_uxd::instructions::process_edit_identity_depository(
        program_test_context,
        program_keys,
        payer,
        Some(identity_depository_redeemable_amount_under_management_cap),
        Some(identity_depository_minting_disabled),
    )
    .await?;

    // Done
    Ok(())
}
