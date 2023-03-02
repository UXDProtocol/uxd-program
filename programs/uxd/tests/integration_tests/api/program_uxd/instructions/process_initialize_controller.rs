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
    program_info: &program_uxd::accounts::ProgramInfo,
    payer: &Keypair,
    redeemable_mint_decimals: u8,
) -> Result<(), program_test_context::ProgramTestError> {
    let accounts = uxd::accounts::InitializeController {
        authority: program_info.authority.pubkey(),
        payer: payer.pubkey(),
        controller: program_info.controller,
        redeemable_mint: program_info.redeemable_mint,
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
        &program_info.authority,
    )
    .await
}
