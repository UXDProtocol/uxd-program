use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditCredixLpDepositoryFields;
use uxd::instructions::EditDepositoriesRoutingWeightBps;
use uxd::instructions::EditIdentityDepositoryFields;
use uxd::instructions::EditMercurialVaultDepositoryFields;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::utils::ui_amount_to_native_amount;

#[tokio::test]
async fn test_credix_lp_depository_exchange_liquidity() -> Result<(), program_context::ProgramError>
{
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Setup basic context and accounts needed for this test suite
    // ---------------------------------------------------------------------

    let mut program_context: Box<dyn program_context::ProgramContext> =
        Box::new(program_context::create_program_test_context().await);

    // Fund payer
    let payer = Keypair::new();
    program_context
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
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
        &mut program_context,
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
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;
    // Create a redeemable account for our user
    let user_redeemable = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &program_uxd::accounts::find_redeemable_mint_pda().0,
        &user.pubkey(),
    )
    .await?;

    // Create a collateral account for our profits_beneficiary
    let profits_beneficiary_collateral =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_context,
            &payer,
            &collateral_mint.pubkey(),
            &profits_beneficiary.pubkey(),
        )
        .await?;

    // Useful amounts used during testing scenario
    let amount_we_use_as_supply_cap =
        ui_amount_to_native_amount(50_000_000, redeemable_mint_decimals);

    let amount_of_collateral_airdropped_to_user =
        ui_amount_to_native_amount(1_000_000_000, collateral_mint_decimals);
    let amount_the_user_should_be_able_to_mint =
        ui_amount_to_native_amount(50_000_000, collateral_mint_decimals);

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Prepare the program state to be ready,
    // -- Set all depository caps for proper target computation
    // -- Mint a bunch of redeemable to fill up all depositories
    // ---------------------------------------------------------------------

    // Airdrop collateral to our user
    program_spl::instructions::process_token_mint_to(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &collateral_mint,
        &user_collateral,
        amount_of_collateral_airdropped_to_user,
    )
    .await?;

    // Set the controller cap and the weights
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(amount_we_use_as_supply_cap.into()),
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 50 * 100,
                mercurial_vault_depository_weight_bps: 25 * 100,
                credix_lp_depository_weight_bps: 25 * 100,
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Now we set the router depositories to the correct PDAs
    program_uxd::procedures::process_set_router_depositories(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
    )
    .await?;

    // Set the identity_depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_context,
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
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditMercurialVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(0),
            redeeming_fee_in_bps: Some(0),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: Some(profits_beneficiary_collateral),
        },
    )
    .await?;

    // Set the credix_lp_depository cap and make sure minting is not disabled
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(0),
            redeeming_fee_in_bps: Some(0),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: Some(profits_beneficiary_collateral),
        },
    )
    .await?;

    // Minted amounts
    let identity_depository_collateral_amount_minted = amount_the_user_should_be_able_to_mint / 2; // identity_depository 50% weight
    let mercurial_vault_depository_collateral_amount_minted =
        amount_the_user_should_be_able_to_mint / 4; // mercurial_vault_depository 25% weight
    let credix_lp_depository_collateral_amount_minted = amount_the_user_should_be_able_to_mint / 4; // credix_lp_depository 25% weight

    // Minting on should work now that everything is set
    program_uxd::instructions::process_mint(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_mint,
        identity_depository_collateral_amount_minted,
        mercurial_vault_depository_collateral_amount_minted,
        credix_lp_depository_collateral_amount_minted,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- User will now trade USDC for all the liquidity in the credix_lp_depository
    // -- User will send the credix shares to an external wallet's account
    // ---------------------------------------------------------------------

    // Resolve credix's important PDAs
    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds).0;
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository_pda(
        &collateral_mint.pubkey(),
        &credix_global_market_state,
    )
    .0;
    let credix_shares_mint =
        program_credix::accounts::find_lp_token_mint_pda(&credix_market_seeds).0;
    let credix_lp_depository_shares = program_uxd::accounts::find_credix_lp_depository_shares(
        &credix_lp_depository,
        &credix_shares_mint,
    );

    // Create receiver's shares ATA
    let receiver = Keypair::new();
    let receiver_credix_shares =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_context,
            &payer,
            &credix_shares_mint,
            &receiver.pubkey(),
        )
        .await?;

    // Thaw the accounts involved in the exchange
    program_credix::instructions::process_thaw_freeze_token_account(
        &mut program_context,
        &credix_multisig,
        &credix_lp_depository_shares,
        false,
    )
    .await?;
    program_credix::instructions::process_thaw_freeze_token_account(
        &mut program_context,
        &credix_multisig,
        &receiver_credix_shares,
        false,
    )
    .await?;

    // Exchanging 0 should fail
    assert!(
        program_uxd::instructions::process_exchange_liquidity_with_credix_lp_depository(
            &mut program_context,
            &payer,
            &collateral_mint.pubkey(),
            &user,
            &user_collateral,
            &receiver_credix_shares,
            0,
        )
        .await
        .is_err()
    );

    // Proceed to the exchange in 2 separate partial transaction
    let credix_lp_depository_redeemable_amount_under_management =
        credix_lp_depository_collateral_amount_minted - 1; // precision-loss
    program_uxd::instructions::process_exchange_liquidity_with_credix_lp_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &user,
        &user_collateral,
        &receiver_credix_shares,
        credix_lp_depository_redeemable_amount_under_management / 2,
    )
    .await?;
    program_uxd::instructions::process_exchange_liquidity_with_credix_lp_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &user,
        &user_collateral,
        &receiver_credix_shares,
        credix_lp_depository_redeemable_amount_under_management / 2,
    )
    .await?;

    // Done
    Ok(())
}
