use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;

pub async fn process_associated_token_account_init(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    mint: &Pubkey,
    wallet: &Pubkey,
) -> Result<Pubkey, String> {
    let instruction =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &payer.pubkey(),
            wallet,
            mint,
            &spl_token::id(),
        );
    program_test_context::process_instruction(program_test_context, instruction, payer).await?;
    Ok(spl_associated_token_account::get_associated_token_address(
        wallet, mint,
    ))
}
