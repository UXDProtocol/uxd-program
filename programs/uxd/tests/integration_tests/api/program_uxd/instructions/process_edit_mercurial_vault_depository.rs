use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditMercurialVaultDepositoryFields;
use uxd::state::MercurialVaultDepository;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_mercurial_vault_depository(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    fields: &EditMercurialVaultDepositoryFields,
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

    // Read state before
    let mercurial_vault_depository_before = program_test_context::read_account_anchor::<
        MercurialVaultDepository,
    >(program_runner, &mercurial_vault_depository)
    .await?;

    // Execute IX
    let accounts = uxd::accounts::EditMercurialVaultDepository {
        authority: authority.pubkey(),
        controller,
        depository: mercurial_vault_depository,
    };
    let payload = uxd::instruction::EditMercurialVaultDepository { fields: *fields };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(
        program_runner,
        instruction,
        payer,
        authority,
    )
    .await?;

    // Read state after
    let mercurial_vault_depository_after = program_test_context::read_account_anchor::<
        MercurialVaultDepository,
    >(program_runner, &mercurial_vault_depository)
    .await?;

    // redeemable_amount_under_management_cap must have been updated if specified in fields
    let redeemable_amount_under_management_cap_before =
        mercurial_vault_depository_before.redeemable_amount_under_management_cap;
    let redeemable_amount_under_management_cap_after =
        mercurial_vault_depository_after.redeemable_amount_under_management_cap;
    assert_eq!(
        redeemable_amount_under_management_cap_after,
        fields
            .redeemable_amount_under_management_cap
            .unwrap_or(redeemable_amount_under_management_cap_before)
    );

    // minting_fee_in_bps must have been updated if specified in fields
    let minting_fee_in_bps_before = mercurial_vault_depository_before.minting_fee_in_bps;
    let minting_fee_in_bps_after = mercurial_vault_depository_after.minting_fee_in_bps;
    assert_eq!(
        minting_fee_in_bps_after,
        fields
            .minting_fee_in_bps
            .unwrap_or(minting_fee_in_bps_before)
    );

    // redeeming_fee_in_bps must have been updated if specified in fields
    let redeeming_fee_in_bps_before = mercurial_vault_depository_before.redeeming_fee_in_bps;
    let redeeming_fee_in_bps_after = mercurial_vault_depository_after.redeeming_fee_in_bps;
    assert_eq!(
        redeeming_fee_in_bps_after,
        fields
            .redeeming_fee_in_bps
            .unwrap_or(redeeming_fee_in_bps_before)
    );

    // minting_disabled must have been updated if specified in fields
    let minting_disabled_before = mercurial_vault_depository_before.minting_disabled;
    let minting_disabled_after = mercurial_vault_depository_after.minting_disabled;
    assert_eq!(
        minting_disabled_after,
        fields.minting_disabled.unwrap_or(minting_disabled_before)
    );

    // profits_beneficiary_collateral must have been updated if specified in fields
    let profits_beneficiary_collateral_before =
        mercurial_vault_depository_before.profits_beneficiary_collateral;
    let profits_beneficiary_collateral_after =
        mercurial_vault_depository_after.profits_beneficiary_collateral;
    assert_eq!(
        profits_beneficiary_collateral_after,
        fields
            .profits_beneficiary_collateral
            .unwrap_or(profits_beneficiary_collateral_before)
    );

    // Done
    Ok(())
}
