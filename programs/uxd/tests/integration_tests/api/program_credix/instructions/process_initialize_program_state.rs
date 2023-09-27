use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;

pub async fn process_initialize_program_state(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let program_state = program_credix::accounts::find_program_state_pda().0;
    let treasury_pool = program_credix::accounts::find_treasury_pool(&multisig.pubkey());

    // Execute IX
    let accounts = credix_client::accounts::InitializeProgramState {
        owner: multisig.pubkey(),
        program_state,
        system_program: solana_sdk::system_program::ID,
        rent: solana_sdk::sysvar::rent::ID,
    };
    let payload = credix_client::instruction::InitializeProgramState {
        _credix_managers: [
            treasury_pool,
            treasury_pool,
            treasury_pool,
            treasury_pool,
            treasury_pool,
            treasury_pool,
            treasury_pool,
            treasury_pool,
            treasury_pool,
            treasury_pool,
        ],
        _credix_multisig_key: multisig.pubkey(),
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, multisig).await
}
