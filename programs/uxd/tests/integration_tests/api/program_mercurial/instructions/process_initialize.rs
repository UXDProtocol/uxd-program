use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;

pub async fn process_initialize_program_state(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_mercurial::accounts::ProgramKeys,
) -> Result<(), String> {
    let accounts = mercurial_vault::accounts::Initialize {};
    let payload = mercurial_vault::instruction::Initialize {};
    let instruction = Instruction {
        program_id: mercurial_vault::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(
        program_test_context,
        instruction,
        &program_keys.authority,
    )
    .await
}
