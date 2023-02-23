use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_uxd;

pub async fn process_program_setup_init(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    program_setup: &program_uxd::accounts::ProgramSetup,
    redeemable_mint_decimals: u8,
    redeemable_global_supply_cap: u128,
    identity_depository_redeemable_amount_under_management_cap: u128,
    identity_depository_minting_disabled: bool,
    credix_lp_depository_minting_fee_in_bps: u8,
    credix_lp_depository_redeeming_fee_in_bps: u8,
    credix_lp_depository_redeemable_amount_under_management_cap: u128,
) -> Result<(), String> {
    // Controller setup
    program_uxd::instructions::process_initialize_controller(
        program_test_context,
        payer,
        program_setup,
        redeemable_mint_decimals,
    )
    .await?;
    program_uxd::instructions::process_edit_controller(
        program_test_context,
        payer,
        program_setup,
        Some(redeemable_global_supply_cap),
    )
    .await?;

    // Identity depository setup
    program_uxd::instructions::process_initialize_identity_depository(
        program_test_context,
        payer,
        program_setup,
    )
    .await?;
    program_uxd::instructions::process_edit_identity_depository(
        program_test_context,
        payer,
        program_setup,
        Some(identity_depository_redeemable_amount_under_management_cap),
        Some(identity_depository_minting_disabled),
    )
    .await?;

    // Credix dependency program setup
    program_credix::procedures::process_program_setup_init(
        program_test_context,
        &program_setup
            .credix_lp_depository_setup
            .credix_program_setup,
    )
    .await?;

    // Credix lp depository setup
    program_uxd::instructions::process_register_credix_lp_depository(
        program_test_context,
        &payer,
        program_setup,
        credix_lp_depository_minting_fee_in_bps,
        credix_lp_depository_redeeming_fee_in_bps,
        credix_lp_depository_redeemable_amount_under_management_cap,
    )
    .await?;

    // Credix pass creation
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
        &program_setup
            .credix_lp_depository_setup
            .credix_program_setup,
        &program_setup.credix_lp_depository_setup.depository,
        &program_setup.credix_lp_depository_setup.credix_pass,
        true,
        false,
        0,
        true,
    )
    .await?;

    // TODO - initialize credix/mercurial too here

    // Redeemable tokens ready to be minted/redeemed
    Ok(())
}
