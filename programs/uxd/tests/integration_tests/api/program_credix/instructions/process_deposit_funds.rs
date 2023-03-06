use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_deposit_funds(
    program_test_context: &mut ProgramTestContext,
    authority: &Keypair,
    base_token_mint: &Pubkey,
    investor: &Keypair,
    investor_token_account: &Pubkey,
    investor_lp_token_account: &Pubkey,
    amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    let market_seeds = program_credix::accounts::find_market_seeds();
    let program_state = program_credix::accounts::find_program_state();
    let global_market_state = program_credix::accounts::find_global_market_state(&market_seeds);
    let market_admins = program_credix::accounts::find_market_admins(&global_market_state);
    let lp_token_mint = program_credix::accounts::find_lp_token_mint(&market_seeds);
    let signing_authority = program_credix::accounts::find_signing_authority(&market_seeds);
    let liquidity_pool_token_account = program_credix::accounts::find_liquidity_pool_token_account(
        &signing_authority,
        base_token_mint,
    );
    let credix_pass =
        program_credix::accounts::find_credix_pass(&global_market_state, &investor.pubkey());
    let accounts = credix_client::accounts::DepositFunds {
        investor: investor.pubkey(),
        investor_token_account: *investor_token_account,
        investor_lp_token_account: *investor_lp_token_account,
        credix_pass: credix_pass,
        global_market_state: global_market_state,
        signing_authority: signing_authority,
        liquidity_pool_token_account: liquidity_pool_token_account,
        base_token_mint: *base_token_mint,
        lp_token_mint: lp_token_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = credix_client::instruction::DepositFunds { _amount: amount };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_test_context, instruction, &investor).await
}
