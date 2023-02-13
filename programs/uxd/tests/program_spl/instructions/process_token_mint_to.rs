use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test_context;

pub async fn process_token_mint_to(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    mint: &Pubkey,
    authority: &Keypair,
    token_account: &Pubkey,
    amount: u64,
) -> Result<(), String> {
    let instruction = spl_token::instruction::mint_to(
        &spl_token::ID,
        mint,
        token_account,
        &authority.pubkey(),
        &[],
        amount,
    )
    .map_err(|e| e.to_string())?;

    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        authority,
    )
    .await?;

    Ok(())
}
