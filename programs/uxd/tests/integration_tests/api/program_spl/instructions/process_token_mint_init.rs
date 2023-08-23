use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Mint;

use crate::integration_tests::api::program_context;

pub async fn process_token_mint_init(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    mint: &Keypair,
    decimals: u8,
    authority: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    let minimum_balance = program_context.get_minimum_balance(Mint::LEN).await?;

    let instruction_create = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        minimum_balance,
        Mint::LEN as u64,
        &spl_token::id(),
    );
    program_context::process_instruction_with_signer(
        program_context,
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
    .map_err(program_context::ProgramError::Program)?;

    program_context::process_instruction(program_context, instruction_init, payer).await?;

    Ok(())
}
