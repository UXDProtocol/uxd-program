use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use uxd::instructions::EditAlloyxVaultDepositoryFields;
use uxd::instructions::EditCredixLpDepositoryFields;
use uxd::instructions::EditIdentityDepositoryFields;
use uxd::instructions::EditMercurialVaultDepositoryFields;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_setup_router_depositories_fields(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    supply_cap: u64,
    minting_fee_in_bps: Option<u8>,
    redeeming_fee_in_bps: Option<u8>,
    minting_disabled: Option<bool>,
    profits_beneficiary_collateral: Option<Pubkey>,
) -> Result<(), program_context::ProgramError> {
    // Set the identity_depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_identity_depository(
        program_context,
        payer,
        authority,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: Some(supply_cap.into()),
            minting_disabled,
        },
    )
    .await?;

    // Set the mercurial_vault_depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_mercurial_vault_depository(
        program_context,
        payer,
        authority,
        collateral_mint,
        &EditMercurialVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(supply_cap.into()),
            minting_fee_in_bps,
            redeeming_fee_in_bps,
            minting_disabled,
            profits_beneficiary_collateral,
        },
    )
    .await?;

    // Set the credix_lp_depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_credix_lp_depository(
        program_context,
        payer,
        authority,
        collateral_mint,
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(supply_cap.into()),
            minting_fee_in_bps,
            redeeming_fee_in_bps,
            minting_disabled,
            profits_beneficiary_collateral,
        },
    )
    .await?;

    // Set the alloyx_vault_depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_alloyx_vault_depository(
        program_context,
        payer,
        authority,
        collateral_mint,
        &EditAlloyxVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(supply_cap),
            minting_fee_in_bps,
            redeeming_fee_in_bps,
            minting_disabled,
            profits_beneficiary_collateral,
        },
    )
    .await?;

    // Done
    Ok(())
}
