use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::state::Controller;
use uxd::state::CredixLpDepository;
use uxd::state::MercurialVaultDepository;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_compute_depositories_targets(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    collateral_mint: &Pubkey,
    mercurial_vault_depository_redeemable_amount_under_management_target_expected: u64,
    credix_lp_depository_redeemable_amount_under_management_target_expected: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let mercurial_base = program_mercurial::accounts::find_base();
    let mercurial_vault =
        program_mercurial::accounts::find_vault_pda(collateral_mint, &mercurial_base.pubkey()).0;
    let mercurial_vault_depository = program_uxd::accounts::find_mercurial_vault_depository_pda(
        collateral_mint,
        &mercurial_vault,
    )
    .0;
    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds).0;
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository_pda(
        collateral_mint,
        &credix_global_market_state,
    )
    .0;

    // Read state before
    let credix_lp_depository_before =
        program_test_context::read_account_anchor::<CredixLpDepository>(
            program_test_context,
            &credix_lp_depository,
        )
        .await?;
    let mercurial_vault_depository_before = program_test_context::read_account_anchor::<
        MercurialVaultDepository,
    >(program_test_context, &mercurial_vault_depository)
    .await?;

    // Execute IX
    let accounts = uxd::accounts::ComputeDepositoriesTargets {
        payer: payer.pubkey(),
        controller,
        mercurial_vault_depository_1: mercurial_vault_depository,
        credix_lp_depository_1: credix_lp_depository,
    };
    let payload = uxd::instruction::ComputeDepositoriesTargets {};
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        authority,
    )
    .await?;

    // Read state after
    let credix_lp_depository_after =
        program_test_context::read_account_anchor::<CredixLpDepository>(
            program_test_context,
            &credix_lp_depository,
        )
        .await?;
    let mercurial_vault_depository_after = program_test_context::read_account_anchor::<
        MercurialVaultDepository,
    >(program_test_context, &mercurial_vault_depository)
    .await?;

    // Check that the computed depositories targets match expectations
    assert_eq!(
        mercurial_vault_depository_after.redeemable_amount_under_management_target,
        mercurial_vault_depository_redeemable_amount_under_management_target_expected
    );
    assert_eq!(
        credix_lp_depository_after.redeemable_amount_under_management_target,
        credix_lp_depository_redeemable_amount_under_management_target_expected
    );

    // Done
    Ok(())
}
