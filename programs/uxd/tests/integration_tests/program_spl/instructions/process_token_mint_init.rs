use anchor_lang::prelude::Pubkey;
use solana_program::program_pack::Pack;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;
use spl_token::state::Mint;

pub async fn process_token_mint_init(
    program_test_context: &mut ProgramTestContext,
    mint: &Keypair,
    decimals: u8,
    authority: &Keypair,
    payer: &Keypair,
) -> Result<(), String> {
    let rent =
        crate::integration_tests::program_test_context::get_rent(program_test_context).await?;

    let instruction_create = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent.minimum_balance(Mint::LEN),
        Mint::LEN as u64,
        &authority.pubkey(),
    );
    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction_create,
        mint,
        payer,
    )
    .await?;

    let instruction_init = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &authority.pubkey(),
        Some(&authority.pubkey()),
        decimals,
    )
    .map_err(|e| e.to_string())?;

    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction_init,
        authority,
        payer,
    )
    .await?;

    Ok(())
}
