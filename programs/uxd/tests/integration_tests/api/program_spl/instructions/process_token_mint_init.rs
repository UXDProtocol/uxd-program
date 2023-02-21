use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Mint;

use crate::integration_tests::api::program_test_context;

pub async fn process_token_mint_init(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    mint: &Keypair,
    decimals: u8,
    authority: &Pubkey,
) -> Result<(), String> {
    let rent = program_test_context
        .banks_client
        .get_rent()
        .await
        .map_err(|e| e.to_string())?;

    let instruction_create = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent.minimum_balance(Mint::LEN),
        Mint::LEN as u64,
        &spl_token::id(),
    );
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction_create,
        payer,
        mint,
    )
    .await?;

    let instruction_init = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        authority,
        Some(authority),
        decimals,
    )
    .map_err(|e| e.to_string())?;

    program_test_context::process_instruction(program_test_context, instruction_init, payer)
        .await?;

    Ok(())
}
