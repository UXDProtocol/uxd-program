use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;

pub async fn process_create_withdraw_epoch(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
    epoch_idx: u32,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;

    // Find the next withdraw epoch account
    let withdraw_epoch =
        program_credix::accounts::find_withdraw_epoch_pda(&global_market_state, epoch_idx).0;

    // Execute IX
    let accounts = credix_client::accounts::CreateWithdrawEpoch {
        owner: multisig.pubkey(),
        global_market_state,
        withdraw_epoch,
        market_admins,
        system_program: solana_sdk::system_program::ID,
    };
    let payload = credix_client::instruction::CreateWithdrawEpoch {};
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, multisig).await
}
