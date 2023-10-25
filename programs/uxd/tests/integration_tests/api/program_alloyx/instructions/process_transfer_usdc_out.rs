use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Mint;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_spl;

pub async fn process_transfer_usdc_out(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    collateral_amount: u64,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let vault_id = program_alloyx::accounts::find_vault_id();
    let vault_info = program_alloyx::accounts::find_vault_info(&vault_id).0;
    let vault_usdc_token = program_alloyx::accounts::find_vault_usdc_token(&vault_id).0;

    let authority_collateral =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_context,
            authority,
            collateral_mint,
            &authority.pubkey(),
        )
        .await?;

    let collateral_mint_before =
        program_context::read_account_packed::<Mint>(program_context, collateral_mint).await?;

    // Execute IX
    let accounts = alloyx_cpi::accounts::TransferUsdcOut {
        signer: authority.pubkey(),
        vault_info_account: vault_info,
        usdc_vault_account: vault_usdc_token,
        usdc_mint: *collateral_mint,
        user_usdc_account: authority_collateral,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        system_program: solana_sdk::system_program::ID,
    };
    let payload = alloyx_cpi::instruction::TransferUsdcOut {
        _vault_id: vault_id,
        _amount: collateral_amount / 10u64.pow(u32::from(collateral_mint_before.decimals)), // this IX takes UI units as parameter for some reason
    };
    let instruction = Instruction {
        program_id: alloyx_cpi::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, authority).await
}
