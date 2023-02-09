use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn process_initialize_market(
    program_test_context: &mut ProgramTestContext,
    admin: &Keypair,
    collateral_mint: &Pubkey,
) -> Result<(), String> {
    let global_market_state =
        crate::integration_tests::program_credix::accounts::find_global_market_state_address();
    let program_state =
        crate::integration_tests::program_credix::accounts::find_program_state_address();
    let lp_token_mint =
        crate::integration_tests::program_credix::accounts::find_lp_token_mint_address();
    let signing_authority =
        crate::integration_tests::program_credix::accounts::find_signing_authority_address();
    let market_admins =
        crate::integration_tests::program_credix::accounts::find_market_admins_address(
            global_market_state,
        );

    let liquidity_pool_token_account = anchor_spl::associated_token::get_associated_token_address(
        &signing_authority,
        collateral_mint,
    );

    let treasury = admin.pubkey();
    let treasury_pool_token_account =
        anchor_spl::associated_token::get_associated_token_address(&treasury, collateral_mint);

    let accounts = credix_client::accounts::InitializeMarket {
        owner: admin.pubkey(),
        global_market_state,
        program_state,
        lp_token_mint,
        signing_authority,
        market_admins,
        liquidity_pool_token_account,
        treasury,
        treasury_pool_token_account,
        base_token_mint: *collateral_mint,
        system_program: anchor_lang::system_program::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        token_program: anchor_spl::token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };

    let market_seeds = crate::integration_tests::program_credix::accounts::get_market_seeds();

    let payload = credix_client::instruction::InitializeMarket {
        _multisig: Some(admin.pubkey()),
        _managers: Some(vec![admin.pubkey()]),
        _pass_issuers: Some(vec![admin.pubkey()]),
        _grace_period: 10,
        _global_market_seed: crate::integration_tests::program_credix::accounts::get_market_seeds(),
        _credix_performance_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _credix_service_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _performance_fee: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _withdrawal_fee: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _variable_late_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _fixed_late_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _service_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
    };
    let instruction = solana_sdk::instruction::Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        admin,
        admin,
    )
    .await
}
