use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_create_credix_pass(
    program_test_context: &mut ProgramTestContext,
    market_seeds: &String,
    multisig: &Keypair,
    pass_holder: &Pubkey,
    fields: &credix_client::instruction::CreateCredixPass,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let program_state = program_credix::accounts::find_program_state_pda().0;
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;
    let credix_pass =
        program_credix::accounts::find_credix_pass_pda(&global_market_state, pass_holder).0;

    // Execute IX
    let accounts = credix_client::accounts::CreateCredixPass {
        owner: multisig.pubkey(),
        pass_holder: *pass_holder,
        program_state,
        global_market_state,
        credix_pass,
        market_admins,
        system_program: anchor_lang::system_program::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: fields.data(),
    };
    program_test_context::process_instruction(program_test_context, instruction, multisig).await
}
