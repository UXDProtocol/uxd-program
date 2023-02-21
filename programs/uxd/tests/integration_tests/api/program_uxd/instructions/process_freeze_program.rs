use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_freeze_program(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    program_setup: &program_uxd::accounts::ProgramSetup,
    freeze: bool,
) -> Result<(), String> {
    let controller =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id()).0;

    let accounts = uxd::accounts::FreezeProgram {
        authority: program_setup.authority.pubkey(),
        controller: program_setup.controller,
    };
    let payload = uxd::instruction::FreezeProgram { freeze };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        &program_setup.authority,
    )
    .await
}
