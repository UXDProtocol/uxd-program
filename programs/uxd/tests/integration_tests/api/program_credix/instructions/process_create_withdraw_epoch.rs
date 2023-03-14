use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_create_withdraw_epoch(
    program_test_context: &mut ProgramTestContext,
    authority: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;

    // Find the current withdraw request account
    let global_market_state_data = program_test_context::read_account_anchor::<
        credix_client::GlobalMarketState,
    >(program_test_context, &global_market_state)
    .await?;
    let withdraw_epoch = program_credix::accounts::find_withdraw_epoch_pda(
        &global_market_state,
        global_market_state_data.latest_withdraw_epoch_idx + 1,
    )
    .0;

    // Execute IX
    let accounts = credix_client::accounts::CreateWithdrawEpoch {
        owner: authority.pubkey(),
        global_market_state,
        withdraw_epoch,
        market_admins,
        system_program: anchor_lang::system_program::ID,
    };
    let payload = credix_client::instruction::CreateWithdrawEpoch {};
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_test_context, instruction, authority).await
}
