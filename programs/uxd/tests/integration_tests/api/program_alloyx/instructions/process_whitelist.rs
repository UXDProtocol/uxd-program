use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;

pub async fn process_whitelist(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    authority: &Keypair,
    investor: &Pubkey,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let vault_id = program_alloyx::accounts::find_vault_id();
    let vault_info = program_alloyx::accounts::find_vault_info(&vault_id).0;
    let investor_pass = program_alloyx::accounts::find_investor_pass(&vault_id, &investor).0;

    // Execute IX
    let accounts = alloyx_cpi::accounts::Whitelist {
        signer: authority.pubkey(),
        investor_pass,
        vault_info_account: vault_info,
        system_program: solana_sdk::system_program::ID,
    };
    let payload = alloyx_cpi::instruction::Whitelist {
        _vault_id: vault_id,
        _investor: *investor,
    };
    let instruction = Instruction {
        program_id: alloyx_cpi::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, authority).await
}
