use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn process_mint_with_identity_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    user: &Keypair,
    collateral_mint: &Pubkey,
    collateral_amount: u64,
) -> Result<(), String> {
    let controller = crate::integration_tests::program_uxd::accounts::find_controller_address();

    let identity_depository =
        crate::integration_tests::program_uxd::accounts::find_identity_depository_address();
    let identity_collateral_vault =
        crate::integration_tests::program_uxd::accounts::find_identity_collateral_vault_address();

    let redeemable_mint =
        crate::integration_tests::program_uxd::accounts::find_redeemable_mint_address();

    let user_collateral =
        crate::integration_tests::program_spl::accounts::find_associated_token_account_address(
            collateral_mint,
            &user.pubkey(),
        );
    let user_redeemable =
        crate::integration_tests::program_spl::accounts::find_associated_token_account_address(
            &redeemable_mint,
            &user.pubkey(),
        );

    let accounts = uxd::accounts::MintWithIdentityDepository {
        user: user.pubkey(),
        payer: payer.pubkey(),
        controller,
        depository: identity_depository,
        collateral_vault: identity_collateral_vault,
        redeemable_mint,
        user_collateral,
        user_redeemable,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
    };
    let payload = uxd::instruction::MintWithIdentityDepository { collateral_amount };
    let instruction = solana_sdk::instruction::Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        user,
    )
    .await
}
