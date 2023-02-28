use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_controller(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    redeemable_global_supply_cap: Option<u128>,
) -> Result<(), String> {
    let accounts = uxd::accounts::EditController {
        authority: program_keys.authority.pubkey(),
        controller: program_keys.controller,
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
        &program_keys.authority,
    )
    .await
}
