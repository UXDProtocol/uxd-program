use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditIdentityDepositoryFields;
use uxd::state::IdentityDepository;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_identity_depository(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    fields: &EditIdentityDepositoryFields,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let identity_depository = program_uxd::accounts::find_identity_depository_pda().0;

    // Read state before
    let identity_depository_before = program_context::read_account_anchor::<IdentityDepository>(
        program_context,
        &identity_depository,
    )
    .await?;

    // Execute IX
    let accounts = uxd::accounts::EditIdentityDepository {
        authority: authority.pubkey(),
        controller,
        depository: identity_depository,
    };
    let payload = uxd::instruction::EditIdentityDepository { fields: *fields };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction_with_signer(
        program_context,
        instruction,
        payer,
        authority,
    )
    .await?;

    // Read state after
    let identity_depository_after = program_context::read_account_anchor::<IdentityDepository>(
        program_context,
        &identity_depository,
    )
    .await?;

    // redeemable_amount_under_management_cap must have been updated if specified in fields
    let redeemable_amount_under_management_cap_before =
        identity_depository_before.redeemable_amount_under_management_cap;
    let redeemable_amount_under_management_cap_after =
        identity_depository_after.redeemable_amount_under_management_cap;
    assert_eq!(
        redeemable_amount_under_management_cap_after,
        fields
            .redeemable_amount_under_management_cap
            .unwrap_or(redeemable_amount_under_management_cap_before)
    );

    // minting_disabled must have been updated if specified in fields
    let minting_disabled_before = identity_depository_before.minting_disabled;
    let minting_disabled_after = identity_depository_after.minting_disabled;
    assert_eq!(
        minting_disabled_after,
        fields.minting_disabled.unwrap_or(minting_disabled_before)
    );

    // Done
    Ok(())
}
