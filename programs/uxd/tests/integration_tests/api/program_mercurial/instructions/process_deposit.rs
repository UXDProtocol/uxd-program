use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;

pub async fn process_deposit(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_mercurial::accounts::ProgramKeys,
    user: &Keypair,
    user_token: &Pubkey,
    user_lp: &Pubkey,
    token_amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    let accounts = mercurial_vault::accounts::Deposit {
        vault: program_keys.vault,
        token_vault: program_keys.token_vault,
        lp_mint: program_keys.lp_mint.pubkey(),
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
    program_test_context::process_instruction(program_test_context, instruction, &user).await
}
