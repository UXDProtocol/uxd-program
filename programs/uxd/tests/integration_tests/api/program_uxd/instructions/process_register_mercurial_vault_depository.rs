use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_register_mercurial_vault_depository(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    minting_fee_in_bps: u8,
    redeeming_fee_in_bps: u8,
    redeemable_amount_under_management_cap: u128,
) -> Result<(), String> {
    let mercurial_vault_depository_keys = &program_keys.mercurial_vault_depository_keys;

    let accounts = uxd::accounts::RegisterMercurialVaultDepository {
        authority: program_keys.authority.pubkey(),
        payer: payer.pubkey(),
        controller: program_keys.controller,
        collateral_mint: program_keys.collateral_mint.pubkey(),
        depository: mercurial_vault_depository_keys.depository,
        depository_lp_token_vault: mercurial_vault_depository_keys.depository_lp_token_vault,
        mercurial_vault: mercurial_vault_depository_keys.mercurial_vault,
        mercurial_vault_lp_mint: mercurial_vault_depository_keys.mercurial_vault_lp_mint,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
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
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        &program_keys.authority,
    )
    .await
}
