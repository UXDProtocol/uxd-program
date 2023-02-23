use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_register_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    program_setup: &program_uxd::accounts::ProgramSetup,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<(), String> {
    let credix_lp_depository_setup = &program_setup.credix_lp_depository_setup;

    let accounts = uxd::accounts::RegisterCredixLpDepository {
        authority: program_setup.authority.pubkey(),
        payer: payer.pubkey(),
        controller: program_setup.controller,
        collateral_mint: program_setup.collateral_mint.pubkey(),
        depository: credix_lp_depository_setup.depository,
        depository_collateral: credix_lp_depository_setup.depository_collateral,
        depository_shares: credix_lp_depository_setup.depository_shares,
        credix_program_state: credix_lp_depository_setup.credix_program_state,
        credix_global_market_state: credix_lp_depository_setup.credix_global_market_state,
        credix_signing_authority: credix_lp_depository_setup.credix_signing_authority,
        credix_liquidity_collateral: credix_lp_depository_setup.credix_liquidity_collateral,
        credix_shares_mint: credix_lp_depository_setup.credix_shares_mint,
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
        &program_setup.authority,
    )
    .await
}
