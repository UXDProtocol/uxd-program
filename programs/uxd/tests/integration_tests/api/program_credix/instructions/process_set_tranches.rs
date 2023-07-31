use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_set_tranches(
    program_test_context: &mut ProgramTestContext,
    market_seeds: &String,
    multisig: &Keypair,
    borrower: &Pubkey,
    deal_number: u16,
    principal: u64,
    interest: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;
    let deal =
        program_credix::accounts::find_deal_pda(&global_market_state, borrower, deal_number).0;
    let deal_tranches =
        program_credix::accounts::find_deal_tranches_pda(&global_market_state, &deal).0;
    let repayment_schedule =
        program_credix::accounts::find_repayment_schedule_pda(&global_market_state, &deal).0;

    // The LP tranche should take all
    let lp_tranche = credix_client::TrancheConfig {
        index: 0,
        size: principal,
        max_deposit_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 1,
        },
        early_withdrawal_principal: false,
        funded_by_liquidity_pool: true,
        name: "A".to_string(),
        interest: credix_client::Fraction {
            numerator: u32::try_from(interest / 1_000_000).unwrap(), // yikes
            denominator: u32::try_from(principal / 1_000_000).unwrap(), // yikes
        },
        interest_performance_fee: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        principal_performance_fee: credix_client::Fraction {
            numerator: 0,
            denominator: 100,
        },
        membership_fee: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        late_interest: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        early_principal: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        late_principal: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        variable_rate: credix_client::VariableRate::None,
    };

    // Execute IX
    let accounts = credix_client::accounts::SetTranches {
        owner: multisig.pubkey(),
        deal,
        market_admins,
        deal_tranches,
        repayment_schedule,
        global_market_state,
        system_program: anchor_lang::system_program::ID,
    };
    let payload = credix_client::instruction::SetTranches {
        _tranche_configs: vec![lp_tranche],
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_test_context, instruction, multisig).await
}
