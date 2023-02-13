use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::program_test_context;
use crate::integration_tests::program_uxd;

pub async fn process_initialize_identity_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
) -> Result<(), String> {
    let controller = program_uxd::accounts::find_controller_address();

    let identity_depository = program_uxd::accounts::find_identity_depository_address();
    let identity_collateral_vault = program_uxd::accounts::find_identity_collateral_vault_address();

    let accounts = uxd::accounts::InitializeIdentityDepository {
        authority: authority.pubkey(),
        payer: payer.pubkey(),
        controller,
        depository: identity_depository,
        collateral_vault: identity_collateral_vault,
        collateral_mint: *collateral_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = uxd::instruction::InitializeIdentityDepository {};
    let instruction = solana_sdk::instruction::Instruction {
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
