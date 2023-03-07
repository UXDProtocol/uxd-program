use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditCredixLpDepositoryFields;
use uxd::state::CredixLpDepository;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    fields: &EditCredixLpDepositoryFields,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
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

    // Execute IX
    let accounts = uxd::accounts::EditCredixLpDepository {
        authority: authority.pubkey(),
        controller,
        depository: credix_lp_depository,
    };
    let payload = uxd::instruction::EditCredixLpDepository { fields: *fields };
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

    // Check results
    let redeemable_amount_under_management_cap_before =
        credix_lp_depository_before.redeemable_amount_under_management_cap;
    let redeemable_amount_under_management_cap_after =
        credix_lp_depository_after.redeemable_amount_under_management_cap;
    assert_eq!(
        redeemable_amount_under_management_cap_after,
        fields
            .redeemable_amount_under_management_cap
            .unwrap_or(redeemable_amount_under_management_cap_before)
    );
    let minting_fee_in_bps_before = credix_lp_depository_before.minting_fee_in_bps;
    let minting_fee_in_bps_after = credix_lp_depository_after.minting_fee_in_bps;
    assert_eq!(
        minting_fee_in_bps_after,
        fields
            .minting_fee_in_bps
            .unwrap_or(minting_fee_in_bps_before)
    );
    let redeeming_fee_in_bps_before = credix_lp_depository_before.redeeming_fee_in_bps;
    let redeeming_fee_in_bps_after = credix_lp_depository_after.redeeming_fee_in_bps;
    assert_eq!(
        redeeming_fee_in_bps_after,
        fields
            .redeeming_fee_in_bps
            .unwrap_or(redeeming_fee_in_bps_before)
    );
    let minting_disabled_before = credix_lp_depository_before.minting_disabled;
    let minting_disabled_after = credix_lp_depository_after.minting_disabled;
    assert_eq!(
        minting_disabled_after,
        fields.minting_disabled.unwrap_or(minting_disabled_before)
    );
    let profits_beneficiary_collateral_before =
        credix_lp_depository_before.profits_beneficiary_collateral;
    let profits_beneficiary_collateral_after =
        credix_lp_depository_after.profits_beneficiary_collateral;
    assert_eq!(
        profits_beneficiary_collateral_after,
        fields
            .profits_beneficiary_collateral
            .unwrap_or(profits_beneficiary_collateral_before)
    );

    // Done
    Ok(())
}
