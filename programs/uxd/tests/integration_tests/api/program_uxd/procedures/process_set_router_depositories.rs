use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditRouterDepositories;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_set_router_depositories(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find identity_depository
    let identity_depository = program_uxd::accounts::find_identity_depository_pda().0;

    // Find mercurial_vault_depository
    let mercurial_base = program_mercurial::accounts::find_base();
    let mercurial_vault =
        program_mercurial::accounts::find_vault_pda(collateral_mint, &mercurial_base.pubkey()).0;
    let mercurial_vault_depository = program_uxd::accounts::find_mercurial_vault_depository_pda(
        collateral_mint,
        &mercurial_vault,
    )
    .0;

    // Find credix_lp_depository_marketplace
    let credix_market_seeds_marketplace = program_credix::accounts::find_market_seeds_marketplace();
    let credix_global_market_state_marketplace =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds_marketplace).0;
    let credix_lp_depository_marketplace = program_uxd::accounts::find_credix_lp_depository_pda(
        collateral_mint,
        &credix_global_market_state_marketplace,
    )
    .0;

    // Find credix_lp_depository_receivables
    let credix_market_seeds_receivables = program_credix::accounts::find_market_seeds_receivables();
    let credix_global_market_state_receivables =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds_receivables).0;
    let credix_lp_depository_receivables = program_uxd::accounts::find_credix_lp_depository_pda(
        collateral_mint,
        &credix_global_market_state_receivables,
    )
    .0;

    // Set the controller's depositories addresses
    program_uxd::instructions::process_edit_controller(
        program_test_context,
        payer,
        authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: None,
            router_depositories: Some(EditRouterDepositories {
                identity_depository,
                mercurial_vault_depository,
                credix_lp_depository_marketplace,
                credix_lp_depository_receivables,
            }),
        },
    )
    .await?;

    // Done
    Ok(())
}
