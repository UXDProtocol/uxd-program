use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;

pub async fn process_deposit(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    token_mint: &Pubkey,
    lp_mint: &Pubkey,
    user: &Keypair,
    user_token: &Pubkey,
    user_lp: &Pubkey,
    token_amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let base = program_mercurial::accounts::find_base();
    let vault = program_mercurial::accounts::find_vault_pda(token_mint, &base.pubkey()).0;
    let token_vault = program_mercurial::accounts::find_token_vault_pda(&vault).0;

    // Execute IX
    let accounts = mercurial_vault::accounts::Deposit {
        vault,
        token_vault,
        lp_mint: *lp_mint,
        user: user.pubkey(),
        user_token: *user_token,
        user_lp: *user_lp,
        token_program: anchor_spl::token::ID,
    };
    let payload = mercurial_vault::instruction::Deposit {
        _token_amount: token_amount,
        _minimum_lp_token_amount: token_amount,
    };
    let instruction = Instruction {
        program_id: mercurial_vault::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_runner, instruction, user).await
}
