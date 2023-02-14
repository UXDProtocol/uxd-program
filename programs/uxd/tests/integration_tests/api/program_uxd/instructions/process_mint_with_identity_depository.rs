use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;

pub async fn process_mint_with_identity_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    collateral_amount: u64,
) -> Result<(), String> {
    let controller =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id()).0;

    let redeemable_mint =
        Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE.as_ref()], &uxd::id()).0;

    let identity_depository =
        Pubkey::find_program_address(&[uxd::IDENTITY_DEPOSITORY_NAMESPACE.as_ref()], &uxd::id()).0;
    let identity_collateral_vault = Pubkey::find_program_address(
        &[uxd::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE.as_ref()],
        &uxd::id(),
    )
    .0;

    let accounts = uxd::accounts::MintWithIdentityDepository {
        user: user.pubkey(),
        payer: payer.pubkey(),
        controller,
        depository: identity_depository,
        collateral_vault: identity_collateral_vault,
        redeemable_mint,
        user_collateral: *user_collateral,
        user_redeemable: *user_redeemable,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
    };
    let payload = uxd::instruction::MintWithIdentityDepository { collateral_amount };
    let instruction = solana_sdk::instruction::Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        user,
    )
    .await
}
