use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_repay_deal(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    borrower: &Keypair,
    borrower_token_account: &Pubkey,
    multisig: &Pubkey,
    deal_number: u16,
    base_token_mint: &Pubkey,
    amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let program_state = program_credix::accounts::find_program_state_pda().0;
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let liquidity_pool_token_account = program_credix::accounts::find_liquidity_pool_token_account(
        &signing_authority,
        base_token_mint,
    );
    let treasury = program_credix::accounts::find_treasury(multisig);
    let treasury_pool_token_account =
        program_credix::accounts::find_treasury_pool_token_account(&treasury, base_token_mint);
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

    let credix_multisig_token_account =
        spl_associated_token_account::get_associated_token_address(multisig, base_token_mint);

    // Execute IX
    let accounts = credix_client::accounts::RepayDeal {
        global_market_state,
        market_admins,
        signing_authority,
        program_state,
        liquidity_pool_token_account,
        signer: borrower.pubkey(),
        borrower: borrower.pubkey(),
        borrower_token_account: *borrower_token_account,
        base_token_mint: *base_token_mint,
        treasury_pool_token_account,
        deal,
        deal_token_account,
        deal_tranches,
        repayment_schedule,
        credix_multisig_key: *multisig,
        credix_multisig_token_account,
        credix_pass,
        variable_interest_rates: credix_client::id(), // Optional, Not set
        system_program: solana_sdk::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
    };
    let payload = credix_client::instruction::RepayDeal { _amount: amount };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_runner, instruction, borrower).await
}
