use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;

pub async fn process_set_locked_liquidity(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let liquidity_pool_token_account = program_credix::accounts::find_liquidity_pool_token_account(
        &signing_authority,
        base_token_mint,
    );

    // Find the current withdraw request account
    let latest_withdraw_epoch_idx = program_context::read_account_anchor::<
        credix_client::GlobalMarketState,
    >(program_context, &global_market_state)
    .await?
    .latest_withdraw_epoch_idx;
    let withdraw_epoch = program_credix::accounts::find_withdraw_epoch_pda(
        &global_market_state,
        latest_withdraw_epoch_idx,
    )
    .0;

    // Execute IX
    let accounts = credix_client::accounts::SetLockedLiquidity {
        owner: multisig.pubkey(),
        global_market_state,
        withdraw_epoch,
        signing_authority,
        liquidity_pool_token_account,
        base_token_mint: *base_token_mint,
    };
    let payload = credix_client::instruction::SetLockedLiquidity {};
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, multisig).await
}
