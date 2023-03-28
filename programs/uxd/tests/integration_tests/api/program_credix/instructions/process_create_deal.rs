use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_create_deal(
    program_test_context: &mut ProgramTestContext,
    multisig: &Keypair,
    borrower: &Pubkey,
    deal_number: u16,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;
    let credix_pass =
        program_credix::accounts::find_credix_pass_pda(&global_market_state, borrower).0;
    let borrower_info =
        program_credix::accounts::find_borrower_info_pda(&global_market_state, borrower).0;
    let deal =
        program_credix::accounts::find_deal_pda(&global_market_state, borrower, deal_number).0;

    // Execute IX
    let accounts = credix_client::accounts::CreateDeal {
        owner: multisig.pubkey(),
        borrower: *borrower,
        borrower_info,
        credix_pass,
        global_market_state,
        deal,
        market_admins,
        system_program: anchor_lang::system_program::ID,
    };
    let payload = credix_client::instruction::CreateDeal {
        _max_funding_duration: 128,
        _deal_name: std::format!("Hello I am deal: {}", deal_number),
        _true_waterfall: true,
        _slash_principal_to_interest: true,
        _slash_interest_to_principal: false,
        _service_fees: 1,
        _fixed_late_fee_percentage: Some(credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        }),
        _performance_fee_percentage: Some(credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        }),
        _grace_period: Some(100),
        _variable_late_fee_percentage: Some(credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        }),
        _service_fee_percentage: Some(credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        }),
        _migrated: false,
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_test_context, instruction, multisig).await
}
