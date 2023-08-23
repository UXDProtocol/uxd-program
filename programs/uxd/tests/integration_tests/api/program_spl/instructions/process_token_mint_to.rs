use solana_program::pubkey::Pubkey;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;

pub async fn process_token_mint_to(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    payer: &Keypair,
    mint: &Pubkey,
    authority: &Keypair,
    token_account: &Pubkey,
    amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    let instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        mint,
        token_account,
        &authority.pubkey(),
        &[],
        amount,
    )
    .map_err(program_test_context::ProgramTestError::Program)?;

    program_test_context::process_instruction_with_signer(
        program_runner,
        instruction,
        payer,
        authority,
    )
    .await
}
