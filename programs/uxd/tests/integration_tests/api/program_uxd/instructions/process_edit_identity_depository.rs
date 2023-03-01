use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_identity_depository(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    redeemable_amount_under_management_cap: Option<u128>,
    minting_disabled: Option<bool>,
) -> Result<(), String> {
    // Read state before
    let identity_depository_before =
        program_test_context::read_account_anchor::<uxd::state::IdentityDepository>(
            program_test_context,
            &program_keys.identity_depository_keys.depository,
        )
        .await?;

    let redeemable_amount_under_management_cap_before =
        identity_depository_before.redeemable_amount_under_management_cap;
    let minting_disabled_before = identity_depository_before.minting_disabled;

    // Execute IX
    let accounts = uxd::accounts::EditIdentityDepository {
        authority: program_keys.authority.pubkey(),
        controller: program_keys.controller,
        depository: program_keys.identity_depository_keys.depository,
    };
    let payload = uxd::instruction::EditIdentityDepository {
        fields: uxd::instructions::EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap,
            minting_disabled,
        },
    };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        &program_keys.authority,
    )
    .await?;

    // Read state after
    let identity_depository_after =
        program_test_context::read_account_anchor::<uxd::state::IdentityDepository>(
            program_test_context,
            &program_keys.identity_depository_keys.depository,
        )
        .await?;

    let redeemable_amount_under_management_cap_after =
        identity_depository_after.redeemable_amount_under_management_cap;
    let minting_disabled_after = identity_depository_after.minting_disabled;

    // Check result
    if redeemable_amount_under_management_cap.is_some() {
        assert_eq!(
            redeemable_amount_under_management_cap_after,
            redeemable_amount_under_management_cap.unwrap()
        );
    } else {
        assert_eq!(
            redeemable_amount_under_management_cap_after,
            redeemable_amount_under_management_cap_before
        );
    }
    if minting_disabled.is_some() {
        assert_eq!(minting_disabled_after, minting_disabled.unwrap());
    } else {
        assert_eq!(minting_disabled_after, minting_disabled_before);
    }

    // Done
    Ok(())
}
