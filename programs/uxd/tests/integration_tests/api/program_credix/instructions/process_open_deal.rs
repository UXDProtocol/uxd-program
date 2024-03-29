use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;

pub async fn process_open_deal(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
    borrower: &Pubkey,
    deal_number: u16,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;
    let deal =
        program_credix::accounts::find_deal_pda(&global_market_state, borrower, deal_number).0;
    let deal_tranches =
        program_credix::accounts::find_deal_tranches_pda(&global_market_state, &deal).0;
    let repayment_schedule =
        program_credix::accounts::find_repayment_schedule_pda(&global_market_state, &deal).0;

    // Execute IX
    let accounts = credix_client::accounts::OpenDeal {
        owner: multisig.pubkey(),
        global_market_state,
        deal,
        deal_tranches,
        repayment_schedule,
        market_admins,
    };
    let payload = credix_client::instruction::OpenDeal {};
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, multisig).await
}
