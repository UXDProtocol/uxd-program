use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditCredixLpDepositoryFields;
use uxd::instructions::EditDepositoriesRoutingWeightBps;
use uxd::instructions::EditIdentityDepositoryFields;
use uxd::instructions::EditMercurialVaultDepositoryFields;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::utils::ui_amount_to_native_amount;

#[tokio::test]
async fn test_credix_lp_depository_rebalance_under_requested(
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
    let amount_we_use_as_supply_cap =
        ui_amount_to_native_amount(100_000_000, redeemable_mint_decimals);

    let amount_of_collateral_airdropped_to_user =
        ui_amount_to_native_amount(1_000_000_000, collateral_mint_decimals);
    let amount_the_user_should_be_able_to_mint =
        ui_amount_to_native_amount(50_000_000, collateral_mint_decimals);

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Prepare the program state to be ready,
    // -- Set all depository caps for proper target computation
    // ---------------------------------------------------------------------

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

    // Set the controller cap and the weights
    program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(amount_we_use_as_supply_cap.into()),
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 25 * 100,
                mercurial_vault_depository_weight_bps: 25 * 100,
                credix_lp_depository_weight_bps: 50 * 100,
            }),
            router_depositories: None,
        },
    )
    .await?;

    // Now we set the router depositories to the correct PDAs
    program_uxd::procedures::process_set_router_depositories(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
    )
    .await?;

    // Set the identity_depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_disabled: Some(false),
        },
    )
    .await?;

    // Set the mercurial_vault_depository cap and make sure minting is not disabled
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
            profits_beneficiary_collateral: Some(profits_beneficiary_collateral),
        },
    )
    .await?;

    // Set the credix_lp_depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(100),
            redeeming_fee_in_bps: Some(100),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: Some(profits_beneficiary_collateral),
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Mint some and then start the rebalancing process
    // -- In between request and redeem, also do a new mint
    // -- In this case, there should be enough liquidity, but not enough requested
    // ---------------------------------------------------------------------

    // Minting on credix should work, happens BEFORE the request
    program_uxd::instructions::process_mint_with_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_mint,
    )
    .await?;

    // Create an epoch (done by credix team usually)
    program_credix::instructions::process_create_withdraw_epoch(
        &mut program_test_context,
        &credix_multisig,
        1,
    )
    .await?;

    // Since the epoch was just created it should be available to create a WithdrawRequest
    program_uxd::instructions::process_rebalance_create_withdraw_request_from_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
    )
    .await?;

    // Pretend 3 days have passed (the time for the request period)
    program_test_context::move_clock_forward(&mut program_test_context, 3 * 24 * 60 * 60).await?;

    // Minting on credix should work, but happens AFTER the request
    program_uxd::instructions::process_mint_with_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_mint,
    )
    .await?;

    // Set the epoch's locked liquidity (done by credix team usually)
    program_credix::instructions::process_set_locked_liquidity(
        &mut program_test_context,
        &credix_multisig,
        &collateral_mint.pubkey(),
    )
    .await?;

    // Compute the expected rebalancing amounts
    let expected_credix_profits_during_first_mint = amount_the_user_should_be_able_to_mint / 100; // 1% profit
    let expected_credix_profits_during_second_mint = amount_the_user_should_be_able_to_mint / 100; // 1% profit

    let expected_credix_first_minted_amount = amount_the_user_should_be_able_to_mint - 1; // precision loss included
    let expected_credix_supply_after_first_mint =
        expected_credix_first_minted_amount - expected_credix_profits_during_first_mint;
    let expected_credix_overflow_after_first_mint =
        expected_credix_supply_after_first_mint * 50 / 100; // 50% overflow (since credix is 50% weight)

    // Executing the rebalance request should now work as intended because we are in the execute period
    program_uxd::instructions::process_rebalance_redeem_withdraw_request_from_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &credix_multisig.pubkey(),
        &profits_beneficiary_collateral,
        expected_credix_overflow_after_first_mint - expected_credix_profits_during_second_mint - 1, // only a little rebalance
        expected_credix_profits_during_first_mint + expected_credix_profits_during_second_mint, // all profits withdrawn
    )
    .await?;

    // Done
    Ok(())
}
