use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn process_associated_token_account_init(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    wallet: &Pubkey,
    mint: &Pubkey,
) -> Result<(), String> {
    let instruction = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        wallet,
        mint,
    );
    crate::integration_tests::program_test_context::process_instruction(
        program_test_context,
        instruction,
        payer,
    )
    .await
}
