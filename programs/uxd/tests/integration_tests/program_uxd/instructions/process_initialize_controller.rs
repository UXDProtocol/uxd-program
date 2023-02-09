use anchor_lang::prelude::Pubkey;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn process_initialize_controller(
    program_test_context: &mut ProgramTestContext,
    authority: &Keypair,
    payer: &Keypair,
    redeemable_mint_decimals: u8,
) -> Result<(), String> {
    let controller = crate::integration_tests::program_uxd::accounts::find_controller_address();
    let redeemable_mint =
        crate::integration_tests::program_uxd::accounts::find_redeemable_mint_address();

    let accounts = uxd::accounts::InitializeController {
        authority: authority.pubkey(),
        payer: payer.pubkey(),
        /*
        controller,
        redeemable_mint,
         */
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = uxd::instruction::InitializeController {
        redeemable_mint_decimals,
    };
    let instruction = solana_sdk::instruction::Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        authority,
        payer,
    )
    .await
}
