use anchor_lang::prelude::Pubkey;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::program_test_context::program_test_context_execute_instruction_with_signer;

pub async fn execute_instruction_initialize_controller(
    program_test_context: &mut ProgramTestContext,
    authority: &Keypair,
    payer: &Keypair,
    redeemable_mint_decimals: u8,
) -> Result<(), String> {
    let (controller, _) =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id());

    let (redeemable_mint, _) =
        Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE.as_ref()], &uxd::id());

    assert_eq!(
        "3tbJcXAWQkFVN26rZPtwkFNvC24sPT35fDxG4M7irLQW",
        controller.to_string()
    );
    assert_eq!(
        "7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT",
        redeemable_mint.to_string()
    );

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
    program_test_context_execute_instruction_with_signer(
        program_test_context,
        instruction,
        authority,
        payer,
    )
    .await
}
