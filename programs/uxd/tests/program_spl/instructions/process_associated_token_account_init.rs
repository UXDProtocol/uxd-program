use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test_context;

pub async fn process_associated_token_account_init(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    mint: &Pubkey,
    wallet: &Pubkey,
) -> Result<Pubkey, String> {
    // note: after upgrading to anchor 0.26.0, we can use the non-deprecated ::instruction
    let instruction = spl_associated_token_account::create_associated_token_account(
        &payer.pubkey(),
        &wallet,
        mint,
    );
    program_test_context::process_instruction(program_test_context, instruction, payer).await?;
    Ok(spl_associated_token_account::get_associated_token_address(
        wallet, mint,
    ))
}
