use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;

pub async fn process_token_mint_to(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    mint: &Pubkey,
    authority: &Keypair,
    token_account: &Pubkey,
    amount: u64,
) -> Result<(), program_context::ProgramError> {
    let instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        mint,
        token_account,
        &authority.pubkey(),
        &[],
        amount,
    )
    .map_err(program_context::ProgramError::Program)?;

    program_context::process_instruction_with_signer(program_context, instruction, payer, authority)
        .await
}
