use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_register_mercurial_vault_depository(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    mercurial_vault_lp_mint: &Pubkey,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let mercurial_base = program_mercurial::accounts::find_base();
    let mercurial_vault =
        program_mercurial::accounts::find_vault_pda(collateral_mint, &mercurial_base.pubkey()).0;
    let mercurial_vault_depository = program_uxd::accounts::find_mercurial_vault_depository_pda(
        collateral_mint,
        &mercurial_vault,
    )
    .0;
    let mercurial_vault_depository_lp_token_vault =
        program_uxd::accounts::find_mercurial_vault_depository_lp_token_vault_pda(
            collateral_mint,
            &mercurial_vault,
        )
        .0;

    // Execute IX
    let accounts = uxd::accounts::RegisterMercurialVaultDepository {
        authority: authority.pubkey(),
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        depository: mercurial_vault_depository,
        depository_lp_token_vault: mercurial_vault_depository_lp_token_vault,
        mercurial_vault,
        mercurial_vault_lp_mint: *mercurial_vault_lp_mint,
        system_program: solana_sdk::system_program::ID,
        token_program: anchor_spl::token::ID,
        rent: solana_sdk::sysvar::rent::ID,
    };
    let payload = uxd::instruction::RegisterMercurialVaultDepository {
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
