use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_initialize_controller(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    payer: &Keypair,
    authority: &Keypair,
    redeemable_mint_decimals: u8,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint_pda().0;

    // Execute IX
    let accounts = uxd::accounts::InitializeController {
        authority: authority.pubkey(),
        payer: payer.pubkey(),
        controller,
        redeemable_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        rent: solana_sdk::sysvar::rent::ID,
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
        program_runner,
        instruction,
        payer,
        authority,
    )
    .await
}
