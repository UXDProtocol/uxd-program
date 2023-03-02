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
    program_info: &program_credix::accounts::ProgramInfo,
    investor: &Keypair,
    investor_pass: &Pubkey,
    investor_token_account: &Pubkey,
    investor_lp_token_account: &Pubkey,
    amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    let accounts = credix_client::accounts::DepositFunds {
        investor: investor.pubkey(),
        investor_token_account: *investor_token_account,
        investor_lp_token_account: *investor_lp_token_account,
        credix_pass: *investor_pass,
        global_market_state: program_info.global_market_state,
        signing_authority: program_info.signing_authority,
        liquidity_pool_token_account: program_info.liquidity_pool_token_account,
        base_token_mint: program_info.base_token_mint,
        lp_token_mint: program_info.lp_token_mint,
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
