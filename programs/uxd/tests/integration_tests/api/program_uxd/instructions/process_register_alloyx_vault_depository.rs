use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_register_alloyx_vault_depository(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    alloyx_vault_mint: &Pubkey,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u64,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;

    let alloyx_vault_id = program_alloyx::accounts::find_vault_id();
    let alloyx_vault_info = program_alloyx::accounts::find_vault_info(&alloyx_vault_id).0;
    let alloyx_vault_collateral =
        program_alloyx::accounts::find_vault_usdc_token(&alloyx_vault_id).0;
    let alloyx_vault_shares = program_alloyx::accounts::find_vault_alloyx_token(&alloyx_vault_id).0;

    let alloyx_vault_depository = program_uxd::accounts::find_alloyx_vault_depository_pda(
        &alloyx_vault_info,
        collateral_mint,
    )
    .0;
    let alloyx_vault_depository_collateral =
        program_uxd::accounts::find_alloyx_vault_depository_collateral(
            &alloyx_vault_depository,
            collateral_mint,
        );
    let alloyx_vault_depository_shares = program_uxd::accounts::find_alloyx_vault_depository_shares(
        &alloyx_vault_depository,
        alloyx_vault_mint,
    );

    // Execute IX
    let accounts = uxd::accounts::RegisterAlloyxVaultDepository {
        authority: authority.pubkey(),
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        depository: alloyx_vault_depository,
        depository_collateral: alloyx_vault_depository_collateral,
        depository_shares: alloyx_vault_depository_shares,
        alloyx_vault_info,
        alloyx_vault_collateral,
        alloyx_vault_shares,
        alloyx_vault_mint: *alloyx_vault_mint,
        system_program: solana_sdk::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        rent: solana_sdk::sysvar::rent::ID,
    };
    let payload = uxd::instruction::RegisterAlloyxVaultDepository {
        minting_fee_in_bps,
        redeeming_fee_in_bps,
        redeemable_amount_under_management_cap,
    };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction_with_signer(program_context, instruction, payer, authority)
        .await
}
