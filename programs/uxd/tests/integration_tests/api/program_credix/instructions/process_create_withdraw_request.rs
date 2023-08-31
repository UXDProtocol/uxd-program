use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;

pub async fn process_create_withdraw_request(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    base_token_mint: &Pubkey,
    investor: &Keypair,
    investor_lp_token_account: &Pubkey,
    amount: u64,
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
    program_credix::accounts::find_credix_pass_pda(&global_market_state, &investor.pubkey()).0;
    let epoch_idx = program_context::read_account_anchor::<credix_client::GlobalMarketState>(
        program_context,
        &global_market_state,
    )
    .await?
    .latest_withdraw_epoch_idx;
    let withdraw_epoch =
        program_credix::accounts::find_withdraw_epoch_pda(&global_market_state, epoch_idx).0;
    let withdraw_request = program_credix::accounts::find_withdraw_request_pda(
        &global_market_state,
        &investor.pubkey(),
        epoch_idx,
    )
    .0;

    // Execute IX
    let accounts = credix_client::accounts::CreateWithdrawRequest {
        payer: investor.pubkey(),
        investor: investor.pubkey(),
        global_market_state,
        signing_authority,
        credix_pass,
        withdraw_epoch,
        withdraw_request,
        investor_lp_token_account: *investor_lp_token_account,
        liquidity_pool_token_account,
        lp_token_mint,
        system_program: solana_sdk::system_program::ID,
    };
    let payload = credix_client::instruction::CreateWithdrawRequest { _amount: amount };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, investor).await
}
