use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_credix;
use crate::program_spl;
use crate::program_test_context;

pub async fn process_initialize_market(
    program_test_context: &mut ProgramTestContext,
    admin: &Keypair,
    collateral_mint: &Pubkey,
) -> Result<(), String> {
    let market_seeds = program_credix::accounts::get_market_seeds();
    let global_market_state = credix_client::GlobalMarketState::generate_pda(&market_seeds).0;
    let market_admins = credix_client::MarketAdmins::generate_pda(global_market_state).0;
    let program_state = credix_client::ProgramState::generate_pda().0;
    let signing_authority =
        credix_client::GlobalMarketState::generate_signing_authority_pda(&market_seeds).0;
    let lp_token_mint =
        credix_client::GlobalMarketState::generate_lp_token_mint_pda(&market_seeds).0;

    let liquidity_pool_token_account =
        program_spl::instructions::process_associated_token_account_init(
            program_test_context,
            admin,
            collateral_mint,
            &signing_authority,
        )
        .await?;

    let treasury = admin.pubkey();
    let treasury_pool_token_account =
        program_spl::instructions::process_associated_token_account_init(
            program_test_context,
            admin,
            collateral_mint,
            &treasury,
        )
        .await?;

    let accounts = credix_client::accounts::InitializeMarket {
        owner: admin.pubkey(),
        global_market_state,
        market_admins,
        program_state,
        signing_authority,
        lp_token_mint,
        liquidity_pool_token_account,
        treasury,
        treasury_pool_token_account,
        base_token_mint: *collateral_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };

    let payload = credix_client::instruction::InitializeMarket {
        _global_market_seed: program_credix::accounts::get_market_seeds(),
        _multisig: Some(admin.pubkey()),
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
    let instruction = solana_sdk::instruction::Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    //program_test_context::process_instruction(program_test_context, instruction, admin).await?;

    Ok(())
}
