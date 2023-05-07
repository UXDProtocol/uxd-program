use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use uxd::instructions::EditControllerFields;
use uxd::instructions::EditMercurialVaultDepositoryFields;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::utils::ui_amount_to_native_amount;

#[tokio::test]
async fn test_mercurial_vault_depository_mint_and_redeem_and_collect_profits(
) -> Result<(), program_test_context::ProgramTestError> {
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
    let credix_multisig = Keypair::new();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint,
        &credix_multisig,
        collateral_mint_decimals,
        redeemable_mint_decimals,
    )
    .await?;

    // Main actor
    let user = Keypair::new();
    let profits_beneficiary = Keypair::new();

    // Create a collateral account for our user
    let user_collateral = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;
    // Create a redeemable account for our user
    let user_redeemable = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_test_context,
        &payer,
        &program_uxd::accounts::find_redeemable_mint_pda().0,
        &user.pubkey(),
    )
    .await?;

    // Create a collateral account for our profits_beneficiary
    let profits_beneficiary_collateral =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &profits_beneficiary.pubkey(),
        )
        .await?;

    // Useful amounts used during testing scenario
    let amount_we_use_as_supply_cap = ui_amount_to_native_amount(50, redeemable_mint_decimals);
    let amount_bigger_than_the_supply_cap =
        ui_amount_to_native_amount(300, redeemable_mint_decimals);

    let amount_of_collateral_airdropped_to_user =
        ui_amount_to_native_amount(1000, collateral_mint_decimals);
    let amount_the_user_should_be_able_to_mint =
        ui_amount_to_native_amount(50, collateral_mint_decimals);

    let amount_the_user_should_be_able_to_redeem =
        ui_amount_to_native_amount(40, redeemable_mint_decimals);

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- We try to mint (and it should fail)
    // -- and progressively set all things needed to mint one by one:
    // --  - airdrop collateral to our user
    // --  - set the supply cap in the controller
    // --  - set the redeemable cap and enable minting in the identity depository
    // -- when everything is ready, try to mint incorrect amounts (it should fail)
    // ---------------------------------------------------------------------

    // Minting should fail because the user doesnt have collateral yet
    assert!(
        program_uxd::instructions::process_mint_with_mercurial_vault_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &mercurial_vault_lp_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_the_user_should_be_able_to_mint,
        )
        .await
        .is_err()
    );

    // Airdrop collateral to our user
    program_spl::instructions::process_token_mint_to(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &collateral_mint,
        &user_collateral,
        amount_of_collateral_airdropped_to_user,
    )
    .await?;

    // Minting should fail because the controller cap is too low
    assert!(
        program_uxd::instructions::process_mint_with_mercurial_vault_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &mercurial_vault_lp_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_the_user_should_be_able_to_mint,
        )
        .await
        .is_err()
    );

    // Set the controller cap
    program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(amount_we_use_as_supply_cap.into()),
            router_depositories_weight_bps: None,
            router_depositories: None,
        },
    )
    .await?;

    // Minting should fail because the depository cap is too low
    assert!(
        program_uxd::instructions::process_mint_with_mercurial_vault_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &mercurial_vault_lp_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_the_user_should_be_able_to_mint,
        )
        .await
        .is_err()
    );

    // Set the depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_mercurial_vault_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditMercurialVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(100),
            redeeming_fee_in_bps: Some(100),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: None,
        },
    )
    .await?;

    // Minting too much should fail (above cap, but enough collateral)
    assert!(
        program_uxd::instructions::process_mint_with_mercurial_vault_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &mercurial_vault_lp_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_bigger_than_the_supply_cap,
        )
        .await
        .is_err()
    );

    // Minting zero should fail
    assert!(
        program_uxd::instructions::process_mint_with_mercurial_vault_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &mercurial_vault_lp_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            0,
        )
        .await
        .is_err()
    );

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Everything is ready for minting
    // -- We should now successfully be able to mint
    // -- After minting, we redeem (and it should succeed)
    // -- We also test invalid redeem amounts (and it should fail)
    // ---------------------------------------------------------------------

    // Minting should work now that everything is set
    program_uxd::instructions::process_mint_with_mercurial_vault_depository(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_mint,
    )
    .await?;

    // Redeeming the correct amount should succeed
    program_uxd::instructions::process_redeem_from_mercurial_vault_depository(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_redeem,
    )
    .await?;

    // Redeeming too much should fail
    assert!(
        program_uxd::instructions::process_redeem_from_mercurial_vault_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &mercurial_vault_lp_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_bigger_than_the_supply_cap,
        )
        .await
        .is_err()
    );

    // Redeeming zero should fail
    assert!(
        program_uxd::instructions::process_redeem_from_mercurial_vault_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &mercurial_vault_lp_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            0,
        )
        .await
        .is_err()
    );

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- After mint/redeem cycle, we should have a small amount of profits
    // -- We should now successfully be able to collect profits
    // -- Before that we make sure to test and set the depository flag profits beneficiary
    // ---------------------------------------------------------------------

    // Collecting profits first should fail because we havent set a profits beneficiary
    assert!(
        program_uxd::instructions::process_collect_profits_of_mercurial_vault_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &mercurial_vault_lp_mint.pubkey(),
            &profits_beneficiary_collateral,
        )
        .await
        .is_err()
    );

    // Setting the profits beneficiary
    program_uxd::instructions::process_edit_mercurial_vault_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditMercurialVaultDepositoryFields {
            redeemable_amount_under_management_cap: None,
            minting_fee_in_bps: None,
            redeeming_fee_in_bps: None,
            minting_disabled: None,
            profits_beneficiary_collateral: Some(profits_beneficiary_collateral),
        },
    )
    .await?;

    // Now that profits beneficiary is set, collecting profits should succeed
    program_uxd::instructions::process_collect_profits_of_mercurial_vault_depository(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &profits_beneficiary_collateral,
    )
    .await?;

    // Done
    Ok(())
}
