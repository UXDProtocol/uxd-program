use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::state::Controller;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_controller(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    fields: &EditControllerFields,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;

    // Read state before
    let controller_before =
        program_context::read_account_anchor::<Controller>(program_context, &controller).await?;

    // Execute IX
    let accounts = uxd::accounts::EditController {
        authority: authority.pubkey(),
        controller,
    };
    let payload = uxd::instruction::EditController { fields: *fields };
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
    let controller_after =
        program_context::read_account_anchor::<Controller>(program_context, &controller).await?;

    // redeemable_global_supply_cap must have been updated if specified in fields
    let redeemable_global_supply_cap_before = controller_before.redeemable_global_supply_cap;
    let redeemable_global_supply_cap_after = controller_after.redeemable_global_supply_cap;
    assert_eq!(
        redeemable_global_supply_cap_after,
        fields
            .redeemable_global_supply_cap
            .unwrap_or(redeemable_global_supply_cap_before)
    );

    // Done
    Ok(())
}
