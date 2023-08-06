use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::clock::Clock;
use solana_program::clock::SECONDS_PER_DAY;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_set_repayment_schedule(
    program_test_context: &mut ProgramTestContext,
    multisig: &Keypair,
    borrower: &Pubkey,
    deal_number: u16,
    principal: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;
    let deal =
        program_credix::accounts::find_deal_pda(&global_market_state, borrower, deal_number).0;
    let repayment_schedule =
        program_credix::accounts::find_repayment_schedule_pda(&global_market_state, &deal).0;

    // Start the deal now
    let unix_timestamp_now = program_test_context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .map_err(program_test_context::ProgramTestError::BanksClient)?
        .unix_timestamp;

    // Execute IX
    let accounts = credix_client::accounts::SetRepaymentSchedule {
        owner: multisig.pubkey(),
        deal,
        market_admins,
        repayment_schedule,
        global_market_state,
        system_program: anchor_lang::system_program::ID,
    };
    let payload = credix_client::instruction::SetRepaymentSchedule {
        _offset: 0,
        _total_periods: 1,
        _start_ts: unix_timestamp_now,
        _daycount_convention: credix_client::DaycountConvention::Act365,
        _repayment_period_inputs: vec![credix_client::RepaymentPeriodInput {
            waterfall_index: 0,
            accrual_in_days: 30,
            calculation_waterfall_index: 0,
            principal_expected: Some(principal),
            time_frame: credix_client::TimeFrame {
                start: unix_timestamp_now,
                end: unix_timestamp_now + 30 * i64::try_from(SECONDS_PER_DAY).unwrap(),
            },
        }],
        _waterfall_definitions: vec![credix_client::DistributionWaterfall {
            waterfall_type: credix_client::DistributionWaterfallType::Amortization,
            tiers: vec![
                credix_client::WaterfallTier {
                    allocations: vec![credix_client::RepaymentAllocation::Interest],
                    tranche_indices: vec![0],
                    charge: true,
                    slash: false,
                },
                credix_client::WaterfallTier {
                    allocations: vec![credix_client::RepaymentAllocation::Principal],
                    tranche_indices: vec![0],
                    charge: true,
                    slash: false,
                },
                credix_client::WaterfallTier {
                    allocations: vec![
                        credix_client::RepaymentAllocation::LateInterestFee,
                        credix_client::RepaymentAllocation::LatePrincipalFee,
                        credix_client::RepaymentAllocation::EarlyPrincipalFee,
                    ],
                    tranche_indices: vec![0],
                    charge: true,
                    slash: false,
                },
            ],
        }],
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_test_context, instruction, multisig).await
}
