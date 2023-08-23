use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_update_global_market_state(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    multisig: &Keypair,
    has_withdrawal_epochs: bool,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;

    // Execute IX
    let accounts = credix_client::accounts::UpdateGlobalMarketState {
        owner: multisig.pubkey(),
        global_market_state,
        market_admins,
    };
    let payload = credix_client::instruction::UpdateGlobalMarketState {
        _treasury_pool_token_account: None,
        _withdrawal_fee: None,
        _pool_size_limit_percentage: None,
        _withdraw_epoch_request_seconds: None,
        _withdraw_epoch_redeem_seconds: None,
        _withdraw_epoch_available_liquidity_seconds: None,
        _has_withdraw_epochs: Some(has_withdrawal_epochs),
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_runner, instruction, multisig).await
}
