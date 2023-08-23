use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_withdraw_from_deal(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    borrower: &Keypair,
    borrower_token_account: &Pubkey,
    deal_number: u16,
    base_token_mint: &Pubkey,
    amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let credix_pass =
        program_credix::accounts::find_credix_pass_pda(&global_market_state, &borrower.pubkey()).0;
    let deal = program_credix::accounts::find_deal_pda(
        &global_market_state,
        &borrower.pubkey(),
        deal_number,
    )
    .0;
    let deal_token_account =
        program_credix::accounts::find_deal_token_account_pda(&global_market_state, &deal).0;
    let deal_tranches =
        program_credix::accounts::find_deal_tranches_pda(&global_market_state, &deal).0;
    let repayment_schedule =
        program_credix::accounts::find_repayment_schedule_pda(&global_market_state, &deal).0;

    // Execute IX
    let accounts = credix_client::accounts::WithdrawFromDeal {
        global_market_state,
        market_admins,
        signing_authority,
        signer: borrower.pubkey(),
        borrower: borrower.pubkey(),
        borrower_token_account: *borrower_token_account,
        base_token_mint: *base_token_mint,
        deal,
        deal_token_account,
        deal_tranches,
        repayment_schedule,
        credix_pass,
        off_ramp_token_account: credix_client::id(), // Optional, Not set
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = credix_client::instruction::WithdrawFromDeal { _amount: amount };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_runner, instruction, borrower).await
}
