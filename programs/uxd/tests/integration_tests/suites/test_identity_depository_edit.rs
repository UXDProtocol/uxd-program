use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditIdentityDepositoryFields;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_identity_depository_edit() -> Result<(), program_test_context::ProgramTestError> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Setup basic context and accounts needed for this test suite
    // ---------------------------------------------------------------------

    let mut program_runner = program_test_context::create_program_test_context().await;

    // Fund payer
    let payer = Keypair::new();
    program_test_context::ProgramRunner::process_airdrop(
        &mut program_runner,
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
    let credix_multisig = Keypair::new();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_runner,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint,
        &credix_multisig,
        collateral_mint_decimals,
        redeemable_mint_decimals,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Change the depository fields one by one
    // ---------------------------------------------------------------------

    // Change redeemable_amount_under_management_cap
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_runner,
        &payer,
        &authority,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: Some(100),
            minting_disabled: None,
        },
    )
    .await?;

    // Change minting_disabled
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_runner,
        &payer,
        &authority,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_disabled: Some(false),
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Change the depository fields all at once
    // ---------------------------------------------------------------------

    // Change everything, using the wrong authority (should fail)
    assert!(program_uxd::instructions::process_edit_identity_depository(
        &mut program_runner,
        &payer,
        &payer,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: Some(9999),
            minting_disabled: Some(true),
        },
    )
    .await
    .is_err());

    // Change everything, using the correct authority (should succeed)
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_runner,
        &payer,
        &authority,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: Some(9999),
            minting_disabled: Some(true),
        },
    )
    .await?;

    // Change nothing, using the correct authority (should succeed)
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_runner,
        &payer,
        &authority,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_disabled: None,
        },
    )
    .await?;

    // Done
    Ok(())
}
