use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_redeem_from_identity_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    program_setup: &program_uxd::accounts::ProgramSetup,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    redeemable_amount: u64,
) -> Result<(), String> {
    let accounts = uxd::accounts::RedeemFromIdentityDepository {
        user: user.pubkey(),
        payer: payer.pubkey(),
        controller: program_setup.controller,
        depository: program_setup.identity_depository_setup.depository,
        collateral_vault: program_setup.identity_depository_setup.collateral_vault,
        redeemable_mint: program_setup.redeemable_mint,
        user_collateral: *user_collateral,
        user_redeemable: *user_redeemable,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
    };
    let payload = uxd::instruction::RedeemFromIdentityDepository { redeemable_amount };
    let instruction = Instruction {
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
