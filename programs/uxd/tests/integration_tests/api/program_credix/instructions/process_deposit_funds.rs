use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_deposit_funds(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    base_token_mint: &Pubkey,
    investor: &Keypair,
    investor_token_account: &Pubkey,
    investor_lp_token_account: &Pubkey,
    amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let lp_token_mint = program_credix::accounts::find_lp_token_mint_pda(&market_seeds).0;
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let liquidity_pool_token_account = program_credix::accounts::find_liquidity_pool_token_account(
        &signing_authority,
        base_token_mint,
    );
    let credix_pass =
        program_credix::accounts::find_credix_pass_pda(&global_market_state, &investor.pubkey()).0;

    // Execute IX
    let accounts = credix_client::accounts::DepositFunds {
        investor: investor.pubkey(),
        investor_token_account: *investor_token_account,
        investor_lp_token_account: *investor_lp_token_account,
        credix_pass,
        global_market_state,
        signing_authority,
        liquidity_pool_token_account,
        base_token_mint: *base_token_mint,
        lp_token_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        rent: solana_sdk::sysvar::rent::ID,
    };
    let payload = credix_client::instruction::DepositFunds { _amount: amount };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_runner, instruction, investor).await
}
