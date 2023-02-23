use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;

pub async fn process_create_credix_pass(
    program_test_context: &mut ProgramTestContext,
    program_setup: &program_credix::accounts::ProgramSetup,
    pass_holder: &Pubkey,
    credix_pass: &Pubkey,
    is_investor: bool,
    is_borrower: bool,
    release_timestamp: i64,
    disable_withdrawal_fee: bool,
) -> Result<(), String> {
    let accounts = credix_client::accounts::CreateCredixPass {
        owner: program_setup.authority.pubkey(),
        pass_holder: *pass_holder,
        program_state: program_setup.program_state,
        global_market_state: program_setup.global_market_state,
        credix_pass: *credix_pass,
        market_admins: program_setup.market_admins,
        system_program: anchor_lang::system_program::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = credix_client::instruction::CreateCredixPass {
        _is_investor: is_investor,
        _is_borrower: is_borrower,
        _release_timestamp: release_timestamp,
        _disable_withdrawal_fee: disable_withdrawal_fee,
    };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(
        program_test_context,
        instruction,
        &program_setup.authority,
    )
    .await
}
