use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_register_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    credix_lp_depository: &program_uxd::accounts::CredixLpDepository,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<(), String> {
    let controller =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id()).0;

    let accounts = uxd::accounts::RegisterCredixLpDepository {
        authority: authority.pubkey(),
        payer: payer.pubkey(),
        controller,
        depository: credix_lp_depository.depository,
        depository_collateral: credix_lp_depository.depository_collateral,
        depository_shares: credix_lp_depository.depository_shares,
        collateral_mint: credix_lp_depository.collateral_mint,
        credix_program_state: credix_lp_depository.credix_program_state,
        credix_global_market_state: credix_lp_depository.credix_global_market_state,
        credix_signing_authority: credix_lp_depository.credix_signing_authority,
        credix_liquidity_collateral: credix_lp_depository.credix_liquidity_collateral,
        credix_shares_mint: credix_lp_depository.credix_shares_mint,
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
        authority,
    )
    .await
}
