use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn process_edit_identity_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    redeemable_amount_under_management_cap: Option<u128>,
    minting_disabled: Option<bool>,
) -> Result<(), String> {
    let controller = crate::integration_tests::program_uxd::accounts::find_controller_address();

    let identity_depository =
        crate::integration_tests::program_uxd::accounts::find_identity_depository_address();

    let accounts = uxd::accounts::EditIdentityDepository {
        authority: authority.pubkey(),
        controller,
        depository: identity_depository,
    };
    let payload = uxd::instruction::EditIdentityDepository {
        fields: uxd::instructions::EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap,
            minting_disabled,
        },
    };
    let instruction = solana_sdk::instruction::Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        authority,
    )
    .await
}
