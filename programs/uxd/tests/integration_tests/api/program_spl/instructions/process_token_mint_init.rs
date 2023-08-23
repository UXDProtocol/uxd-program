use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Mint;

use crate::integration_tests::api::program_test_context;

pub async fn process_token_mint_init(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    payer: &Keypair,
    mint: &Keypair,
    decimals: u8,
    authority: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    let minimum_balance = program_runner.get_minimum_balance(Mint::LEN).await?;

    let instruction_create = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        minimum_balance,
        Mint::LEN as u64,
        &spl_token::id(),
    );
    program_test_context::process_instruction_with_signer(
        program_runner,
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
    .map_err(program_test_context::ProgramTestError::Program)?;

    program_test_context::process_instruction(program_runner, instruction_init, payer).await?;

    Ok(())
}
