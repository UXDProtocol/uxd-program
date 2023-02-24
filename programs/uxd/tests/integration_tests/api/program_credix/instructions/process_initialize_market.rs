use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_initialize_market(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_credix::accounts::ProgramKeys,
) -> Result<(), String> {
    program_spl::instructions::process_associated_token_account_init(
        program_test_context,
        &program_keys.authority,
        &program_keys.base_token_mint,
        &program_keys.signing_authority,
    )
    .await?;

    program_spl::instructions::process_associated_token_account_init(
        program_test_context,
        &program_keys.authority,
        &program_keys.base_token_mint,
        &program_keys.treasury,
    )
    .await?;

    let accounts = credix_client::accounts::InitializeMarket {
        owner: program_keys.authority.pubkey(),
        global_market_state: program_keys.global_market_state,
        market_admins: program_keys.market_admins,
        program_state: program_keys.program_state,
        signing_authority: program_keys.signing_authority,
        lp_token_mint: program_keys.lp_token_mint,
        base_token_mint: program_keys.base_token_mint,
        liquidity_pool_token_account: program_keys.liquidity_pool_token_account,
        treasury: program_keys.treasury,
        treasury_pool_token_account: program_keys.treasury_pool_token_account,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };

    let payload = credix_client::instruction::InitializeMarket {
        _global_market_seed: program_keys.market_seeds.clone(),
        _multisig: Some(program_keys.authority.pubkey()),
        _managers: None,
        _pass_issuers: None,
        _grace_period: 10,
        _performance_fee: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _withdrawal_fee: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _fixed_late_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _variable_late_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _service_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _credix_performance_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _credix_service_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(
        program_test_context,
        instruction,
        &program_keys.authority,
    )
    .await
}
