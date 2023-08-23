use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;

pub async fn process_initialize(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    admin: &Keypair,
    token_mint: &Pubkey,
    lp_mint: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let base = program_mercurial::accounts::find_base();
    let vault = program_mercurial::accounts::find_vault_pda(token_mint, &base.pubkey()).0;
    let token_vault = program_mercurial::accounts::find_token_vault_pda(&vault).0;
    let treasury = program_mercurial::accounts::find_treasury();
    let fee_vault = program_mercurial::accounts::find_fee_vault(&treasury, lp_mint);

    // Execute IX
    let accounts = mercurial_vault::accounts::Initialize {
        base: base.pubkey(),
        vault,
        admin: admin.pubkey(),
        token_vault,
        token_mint: *token_mint,
        fee_vault,
        lp_mint: *lp_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = mercurial_vault::instruction::Initialize {};
    let instruction = Instruction {
        program_id: mercurial_vault::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(program_runner, instruction, admin, &base)
        .await
}
