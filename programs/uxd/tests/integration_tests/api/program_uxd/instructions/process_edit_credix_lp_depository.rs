use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_edit_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    redeemable_amount_under_management_cap: Option<u128>,
    minting_fee_in_bps: Option<u8>,
    redeeming_fee_in_bps: Option<u8>,
    minting_disabled: Option<bool>,
    profits_beneficiary_collateral: Option<Pubkey>,
) -> Result<(), String> {
    let accounts = uxd::accounts::EditCredixLpDepository {
        authority: program_keys.authority.pubkey(),
        controller: program_keys.controller,
        depository: program_keys.credix_lp_depository_keys.depository,
    };
    let payload = uxd::instruction::EditCredixLpDepository {
        fields: uxd::instructions::EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap,
            minting_fee_in_bps,
            redeeming_fee_in_bps,
            minting_disabled,
            profits_beneficiary_collateral,
        },
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
