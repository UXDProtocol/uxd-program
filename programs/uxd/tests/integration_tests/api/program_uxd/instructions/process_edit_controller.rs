use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::state::Controller;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_controller(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    redeemable_global_supply_cap: Option<u128>,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller();

    // Read state before
    let controller_before =
        program_test_context::read_account_anchor::<Controller>(program_test_context, &controller)
            .await?;

    let redeemable_global_supply_cap_before = controller_before.redeemable_global_supply_cap;

    // Execute IX
    let accounts = uxd::accounts::EditController {
        authority: authority.pubkey(),
        controller,
    };
    let payload = uxd::instruction::EditController {
        fields: uxd::instructions::EditControllerFields {
            redeemable_global_supply_cap,
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
        authority,
    )
    .await?;

    // Read state after
    let controller_after =
        program_test_context::read_account_anchor::<Controller>(program_test_context, &controller)
            .await?;

    let redeemable_global_supply_cap_after = controller_after.redeemable_global_supply_cap;

    // Check result
    assert_eq!(
        redeemable_global_supply_cap_after,
        redeemable_global_supply_cap.unwrap_or(redeemable_global_supply_cap_before)
    );

    // Done
    Ok(())
}
