use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;

pub async fn process_thaw_freeze_token_account(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
    token_account: &Pubkey,
    freeze: bool,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let market_seeds = program_credix::accounts::find_market_seeds();
    let program_state = program_credix::accounts::find_program_state_pda().0;
    let global_market_state =
        program_credix::accounts::find_global_market_state_pda(&market_seeds).0;
    let signing_authority = program_credix::accounts::find_signing_authority_pda(&market_seeds).0;
    let lp_token_mint = program_credix::accounts::find_lp_token_mint_pda(&market_seeds).0;

    // Execute IX
    let accounts = credix_client::accounts::ThawFreezeTokenAccount {
        owner: multisig.pubkey(),
        program_state,
        global_market_state,
        signing_authority,
        token_account: *token_account,
        mint: lp_token_mint,
        token_program: anchor_spl::token::ID,
    };
    let payload = credix_client::instruction::ThawFreezeTokenAccount { _freeze: freeze };
    let instruction = Instruction {
        program_id: credix_client::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, multisig).await
}
