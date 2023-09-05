use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::clock::SECONDS_PER_DAY;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;

pub async fn process_initialize_market(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let program_state = program_credix::accounts::find_program_state_pda().0;
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let market_admins = program_credix::accounts::find_market_admins_pda(&global_market_state).0;
    let lp_token_mint = program_credix::accounts::find_lp_token_mint_pda(&market_seeds).0;
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let liquidity_pool_token_account = program_credix::accounts::find_liquidity_pool_token_account(
        &signing_authority,
        base_token_mint,
    );
    let treasury = program_credix::accounts::find_treasury(&multisig.pubkey());
    let treasury_pool_token_account =
        program_credix::accounts::find_treasury_pool_token_account(&treasury, base_token_mint);

    // Each withdraw epoch will last 3 days each
    let withdraw_epochs_seconds = 3 * u32::try_from(SECONDS_PER_DAY).unwrap();

    // Execute IX
    let accounts = credix_client::accounts::InitializeMarket {
        owner: multisig.pubkey(),
        global_market_state,
        market_admins,
        program_state,
        lp_token_mint,
        base_token_mint: *base_token_mint,
        signing_authority,
        liquidity_pool_token_account,
        treasury,
        treasury_pool_token_account,
        system_program: solana_sdk::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        rent: solana_sdk::sysvar::rent::ID,
    };
    let payload = credix_client::instruction::InitializeMarket {
        _global_market_seed: market_seeds.clone(),
        _multisig: Some(multisig.pubkey()),
        _managers: None,
        _pass_issuers: None,
        _credix_fee_percentage: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _withdrawal_fee: credix_client::Fraction {
            numerator: 1,
            denominator: 100,
        },
        _withdraw_epoch_redeem_seconds: withdraw_epochs_seconds,
        _withdraw_epoch_available_liquidity_seconds: withdraw_epochs_seconds,
        _withdraw_epoch_request_seconds: withdraw_epochs_seconds,
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, multisig).await
}
