use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_initialize_program_state(
    program_test_context: &mut ProgramTestContext,
    authority: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let program_state = program_credix::accounts::find_program_state();
    let treasury = program_credix::accounts::find_treasury(authority);
    let accounts = credix_client::accounts::InitializeProgramState {
        owner: authority.pubkey(),
        program_state: program_state,
        system_program: anchor_lang::system_program::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = credix_client::instruction::InitializeProgramState {
        _credix_managers: [
            treasury, treasury, treasury, treasury, treasury, treasury, treasury, treasury,
            treasury, treasury,
        ],
        _credix_multisig_key: treasury,
        _credix_service_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _credix_performance_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_test_context, instruction, &authority).await
}
