use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditCredixLpDepositoryFields;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_edit_credix_lp_depository() -> Result<(), program_test_context::ProgramTestError> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Setup basic context and accounts needed for this test suite
    // ---------------------------------------------------------------------

    let mut program_test_context = program_test_context::create_program_test_context().await;

    // Fund payer
    let payer = Keypair::new();
    program_spl::instructions::process_lamports_airdrop(
        &mut program_test_context,
        &payer.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Hardcode mints decimals
    let collateral_mint_decimals = 6;
    let redeemable_mint_decimals = 6;

    // Important account keys
    let authority = Keypair::new();
    let collateral_mint = Keypair::new();
    let mercurial_vault_lp_mint = Keypair::new();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint,
        collateral_mint_decimals,
        redeemable_mint_decimals,
    )
    .await?;

    // Initialize some ATAs
    let payer_collateral = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &payer.pubkey(),
    )
    .await?;
    let authority_collateral =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &authority.pubkey(),
        )
        .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Change the depository fields one by one
    // ---------------------------------------------------------------------

    // Change redeemable_amount_under_management_cap
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(100),
            minting_fee_in_bps: None,
            redeeming_fee_in_bps: None,
            minting_disabled: None,
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight: None,
        },
    )
    .await?;

    // Change minting_fee_in_bps
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_fee_in_bps: Some(100),
            redeeming_fee_in_bps: None,
            minting_disabled: None,
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight: None,
        },
    )
    .await?;

    // Change redeeming_fee_in_bps
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_fee_in_bps: None,
            redeeming_fee_in_bps: Some(100),
            minting_disabled: None,
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight: None,
        },
    )
    .await?;

    // Change minting_disabled
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_fee_in_bps: None,
            redeeming_fee_in_bps: None,
            minting_disabled: Some(false),
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight: None,
        },
    )
    .await?;

    // Change profits_beneficiary_collateral
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_fee_in_bps: None,
            redeeming_fee_in_bps: None,
            minting_disabled: None,
            profits_beneficiary_collateral: Some(payer_collateral),
            redeemable_amount_under_management_weight: None,
        },
    )
    .await?;

    // Change redeemable_amount_under_management_weight
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_fee_in_bps: None,
            redeeming_fee_in_bps: None,
            minting_disabled: None,
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight: Some(77),
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Change the depository fields all at once
    // ---------------------------------------------------------------------

    // Change everything, using the wrong authority (should fail)
    assert!(
        program_uxd::instructions::process_edit_credix_lp_depository(
            &mut program_test_context,
            &payer,
            &payer,
            &collateral_mint.pubkey(),
            &EditCredixLpDepositoryFields {
                redeemable_amount_under_management_cap: Some(9999),
                minting_fee_in_bps: Some(41),
                redeeming_fee_in_bps: Some(42),
                minting_disabled: Some(true),
                profits_beneficiary_collateral: Some(authority_collateral),
                redeemable_amount_under_management_weight: Some(66),
            },
        )
        .await
        .is_err()
    );

    // Change everything, using the correct authority (should succeed)
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(9999),
            minting_fee_in_bps: Some(41),
            redeeming_fee_in_bps: Some(42),
            minting_disabled: Some(true),
            profits_beneficiary_collateral: Some(authority_collateral),
            redeemable_amount_under_management_weight: Some(66),
        },
    )
    .await?;

    // Change nothing, using the correct authority (should succeed)
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_fee_in_bps: None,
            redeeming_fee_in_bps: None,
            minting_disabled: None,
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight: None,
        },
    )
    .await?;

    // Done
    Ok(())
}
