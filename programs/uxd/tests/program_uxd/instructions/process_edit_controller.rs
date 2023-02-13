use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test_context;
use crate::program_uxd;

pub async fn process_edit_controller(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    redeemable_global_supply_cap: Option<u128>,
) -> Result<(), String> {
    let controller = program_uxd::accounts::find_controller_address();

    let accounts = uxd::accounts::EditController {
        authority: authority.pubkey(),
        controller,
    };
    let payload = uxd::instruction::EditController {
        fields: uxd::instructions::EditControllerFields {
            redeemable_global_supply_cap,
        },
    };
    let instruction = solana_sdk::instruction::Instruction {
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
    .await
}
