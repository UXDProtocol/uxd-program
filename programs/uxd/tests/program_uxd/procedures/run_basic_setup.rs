use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::program_uxd;

pub async fn run_basic_setup(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    redeemable_mint_decimals: u8,
    redeemable_global_supply_cap: u128,
    identity_depository_redeemable_amount_under_management_cap: u128,
    identity_depository_minting_disabled: bool,
) -> Result<(), String> {
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
        collateral_mint,
    )
    .await?;
    program_uxd::instructions::process_edit_identity_depository(
        program_test_context,
        &payer,
        &authority,
        Some(identity_depository_redeemable_amount_under_management_cap),
        Some(identity_depository_minting_disabled),
    )
    .await?;

    // Ready to use
    Ok(())
}
