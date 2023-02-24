use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_mint_with_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    collateral_amount: u64,
) -> Result<(), String> {
    let credix_lp_depository_keys = &program_keys.credix_lp_depository_keys;

    let accounts = uxd::accounts::MintWithCredixLpDepository {
        payer: payer.pubkey(),
        user: user.pubkey(),
        controller: program_keys.controller,
        collateral_mint: program_keys.collateral_mint.pubkey(),
        redeemable_mint: program_keys.redeemable_mint,
        user_collateral: *user_collateral,
        user_redeemable: *user_redeemable,
        depository: credix_lp_depository_keys.depository,
        depository_collateral: credix_lp_depository_keys.depository_collateral,
        depository_shares: credix_lp_depository_keys.depository_shares,
        credix_global_market_state: credix_lp_depository_keys.credix_global_market_state,
        credix_signing_authority: credix_lp_depository_keys.credix_signing_authority,
        credix_liquidity_collateral: credix_lp_depository_keys.credix_liquidity_collateral,
        credix_shares_mint: credix_lp_depository_keys.credix_shares_mint,
        credix_pass: credix_lp_depository_keys.credix_pass,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        credix_program: credix_client::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = uxd::instruction::MintWithCredixLpDepository { collateral_amount };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        &user,
    )
    .await
}
