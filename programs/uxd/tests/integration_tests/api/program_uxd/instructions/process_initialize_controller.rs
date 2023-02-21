use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_initialize_controller(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    program_setup: &program_uxd::accounts::ProgramSetup,
    redeemable_mint_decimals: u8,
) -> Result<(), String> {
    let accounts = uxd::accounts::InitializeController {
        authority: program_setup.authority.pubkey(),
        payer: payer.pubkey(),
        controller: program_setup.controller,
        redeemable_mint: program_setup.redeemable_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = uxd::instruction::InitializeController {
        redeemable_mint_decimals,
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
