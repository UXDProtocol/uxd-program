use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;

pub async fn process_token_transfer(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    from_authority: &Keypair,
    from_token_account: &Pubkey,
    to_token_account: &Pubkey,
    amount: u64,
) -> Result<(), String> {
    let instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        from_token_account,
        to_token_account,
        &from_authority.pubkey(),
        &[],
        amount,
    )
    .map_err(|e| e.to_string())?;

    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        from_authority,
    )
    .await
}
