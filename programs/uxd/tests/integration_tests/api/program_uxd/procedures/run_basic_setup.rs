use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::integration_tests::api::program_uxd;

/**
 * Setup a simple UXP program with depositories and no deposited money
 * Returns the redeemable mint address that was created
 */
pub async fn run_basic_setup(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    redeemable_mint_decimals: u8,
    redeemable_global_supply_cap: u128,
    identity_depository_redeemable_amount_under_management_cap: u128,
    identity_depository_minting_disabled: bool,
) -> Result<Pubkey, String> {
    let redeemable_mint =
        Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE.as_ref()], &uxd::id()).0;

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

    // TODO - initialize credix/mercurial too here

    // Redeemable tokens ready to be minted/redeemed
    Ok(redeemable_mint)
}
