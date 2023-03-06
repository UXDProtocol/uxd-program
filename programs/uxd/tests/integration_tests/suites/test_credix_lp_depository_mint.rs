use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[tokio::test]
async fn test_credix_lp_depository_mint() -> Result<(), program_test_context::ProgramTestError> {
    // TODO - to remove
    fn collateral_amount_ui_to_native(ui_amount: u64) -> u64 {
        ui_amount * 10u64.pow(6)
    }
    fn redeemable_amount_ui_to_native(ui_amount: u64) -> u64 {
        ui_amount * 10u64.pow(6)
    }

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

    // Important account keys
    let collateral_mint = Keypair::new();
    let authority = Keypair::new();
    let credix_authority = Keypair::new();
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint,
        &credix_authority,
    )
    .await?;

    // Main actor
    let user = Keypair::new();

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
        &redeemable_mint,
        &user.pubkey(),
    )
    .await?;

    // Useful amounts used during testing scenario
    let amount_we_use_as_supply_cap = redeemable_amount_ui_to_native(50);
    let amount_bigger_than_the_supply_cap = redeemable_amount_ui_to_native(300);

    let amount_of_collateral_airdropped_to_user = collateral_amount_ui_to_native(1000);
    let amount_the_user_should_be_able_to_mint = collateral_amount_ui_to_native(50);

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
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
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
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
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
        Some(amount_we_use_as_supply_cap.into()),
    )
    .await?;

    // Minting should fail because the depository cap is too low
    assert!(
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
            &user,
            &user_collateral,
            &user_redeemable,
            amount_the_user_should_be_able_to_mint,
        )
        .await
        .is_err()
    );

    // Set the depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        Some(amount_we_use_as_supply_cap.into()),
        Some(100),
        Some(100),
        Some(false),
        None,
    )
    .await?;

    // Minting too much should fail (above cap, but enough collateral)
    assert!(
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
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
        program_uxd::instructions::process_mint_with_credix_lp_depository(
            &mut program_test_context,
            &payer,
            &collateral_mint.pubkey(),
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
    // ---------------------------------------------------------------------

    // Minting should work now that everything is set
    program_uxd::instructions::process_mint_with_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_mint,
    )
    .await?;

    // Done
    Ok(())
}
