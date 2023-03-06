use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_register_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller();
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint();
    let credix_program_state = program_credix::accounts::find_program_state();
    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state(&credix_market_seeds);
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository(
        collateral_mint,
        &credix_global_market_state,
    );
    let credix_shares_mint = program_credix::accounts::find_lp_token_mint(&credix_market_seeds);
    let credix_signing_authority =
        program_credix::accounts::find_signing_authority(&credix_market_seeds);
    let credix_liquidity_collateral = program_credix::accounts::find_liquidity_pool_token_account(
        &credix_signing_authority,
        collateral_mint,
    );
    let credix_pass = program_credix::accounts::find_credix_pass(
        &credix_global_market_state,
        &credix_lp_depository,
    );
    let credix_lp_depository_collateral =
        program_uxd::accounts::find_credix_lp_depository_collateral(
            &credix_lp_depository,
            collateral_mint,
        );
    let credix_lp_depository_shares = program_uxd::accounts::find_credix_lp_depository_shares(
        &credix_lp_depository,
        &credix_shares_mint,
    );

    // Execute IX
    let accounts = uxd::accounts::RegisterCredixLpDepository {
        authority: authority.pubkey(),
        payer: payer.pubkey(),
        controller: controller,
        collateral_mint: *collateral_mint,
        depository: credix_lp_depository,
        depository_collateral: credix_lp_depository_collateral,
        depository_shares: credix_lp_depository_shares,
        credix_program_state: credix_program_state,
        credix_global_market_state: credix_global_market_state,
        credix_signing_authority: credix_signing_authority,
        credix_liquidity_collateral: credix_liquidity_collateral,
        credix_shares_mint: credix_shares_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = uxd::instruction::RegisterCredixLpDepository {
        minting_fee_in_bps,
        redeeming_fee_in_bps,
        redeemable_amount_under_management_cap,
    };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        &authority,
    )
    .await
}
