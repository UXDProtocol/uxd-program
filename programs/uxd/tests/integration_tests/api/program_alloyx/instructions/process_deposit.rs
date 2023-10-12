use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;

pub async fn process_deposit(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    collateral_mint: &Pubkey,
    alloyx_vault_mint: &Pubkey,
    investor: &Keypair,
    investor_collateral: &Pubkey,
    investor_alloyx: &Pubkey,
    collateral_amount: u64,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let vault_id = program_alloyx::accounts::find_vault_id();
    let vault_info = program_alloyx::accounts::find_vault_info(&vault_id).0;
    let vault_usdc_token = program_alloyx::accounts::find_vault_usdc_token(&vault_id).0;
    let vault_alloyx_token = program_alloyx::accounts::find_vault_alloyx_token(&vault_id).0;
    let investor_pass =
        program_alloyx::accounts::find_investor_pass(&vault_id, &investor.pubkey()).0;

    // Execute IX
    let accounts = alloyx_cpi::accounts::Deposit {
        signer: investor.pubkey(),
        investor_pass,
        vault_info_account: vault_info,
        usdc_vault_account: vault_usdc_token,
        usdc_mint: *collateral_mint,
        alloyx_vault_account: vault_alloyx_token,
        alloyx_mint: *alloyx_vault_mint,
        user_usdc_account: *investor_collateral,
        user_alloyx_account: *investor_alloyx,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        system_program: solana_sdk::system_program::ID,
    };
    let payload = alloyx_cpi::instruction::Deposit {
        _vault_id: vault_id,
        _usdc_amount: collateral_amount,
    };
    let instruction = Instruction {
        program_id: alloyx_cpi::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, investor).await
}
