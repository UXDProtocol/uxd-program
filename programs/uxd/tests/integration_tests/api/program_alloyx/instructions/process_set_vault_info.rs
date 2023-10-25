use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Mint;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;

pub async fn process_set_vault_info(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    desk_collateral_amount: u64,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let vault_id = program_alloyx::accounts::find_vault_id();
    let vault_info = program_alloyx::accounts::find_vault_info(&vault_id).0;

    let collateral_mint_before =
        program_context::read_account_packed::<Mint>(program_context, collateral_mint).await?;

    // Execute IX
    let accounts = alloyx_cpi::accounts::SetVaultInfo {
        signer: authority.pubkey(),
        vault_info_account: vault_info,
        usdc_mint: *collateral_mint,
        system_program: solana_sdk::system_program::ID,
    };
    let payload = alloyx_cpi::instruction::SetVaultInfo {
        _vault_id: vault_id,
        _wallet_desk_amount: desk_collateral_amount
            / 10u64.pow(u32::from(collateral_mint_before.decimals)), // this IX takes UI units as parameter for some reason
    };
    let instruction = Instruction {
        program_id: alloyx_cpi::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, authority).await
}
