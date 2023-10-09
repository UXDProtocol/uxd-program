use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;

pub async fn process_initialize(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    alloyx_vault_mint: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let vault_id = program_alloyx::accounts::find_vault_id();
    let vault_info = program_alloyx::accounts::find_vault_info(&vault_id).0;
    let vault_usdc_token = program_alloyx::accounts::find_vault_usdc_token(&vault_id).0;
    let vault_alloyx_token = program_alloyx::accounts::find_vault_alloyx_token(&vault_id).0;

    // Execute IX
    let accounts = alloyx_cpi::accounts::Initialize {
        signer: authority.pubkey(),
        usdc_vault_account: vault_usdc_token,
        usdc_mint: *collateral_mint,
        alloyx_vault_account: vault_alloyx_token,
        alloyx_mint: *alloyx_vault_mint,
        vault_info_account: vault_info,
        token_program: anchor_spl::token::ID,
        system_program: solana_sdk::system_program::ID,
    };
    let payload = alloyx_cpi::instruction::Initialize {
        _vault_id: vault_id,
    };
    let instruction = Instruction {
        program_id: alloyx_cpi::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, authority).await
}
