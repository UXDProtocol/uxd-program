use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use spl_token::state::Account;
use uxd::instructions::EditControllerFields;
use uxd::instructions::EditDepositoriesRoutingWeightBps;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::utils::ui_amount_to_native_amount;

#[tokio::test]
async fn test_alloyx_vault_depository_rebalance_liquid() -> Result<(), program_context::ProgramError>
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
    let alloyx_vault_mint = Keypair::new();

    // Initialize basic UXD program state
    program_uxd::procedures::process_deploy_program(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint,
        &credix_multisig,
        &alloyx_vault_mint,
        collateral_mint_decimals,
        redeemable_mint_decimals,
    )
    .await?;

    // Main actors
    let user = Keypair::new();
    let profits_beneficiary = Keypair::new();

    // Create a collateral account for our payer
    let payer_collateral = program_spl::instructions::process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &payer.pubkey(),
    )
    .await?;

    // Create a collateral account for our authority
    let authority_collateral =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_context,
            &payer,
            &collateral_mint.pubkey(),
            &authority.pubkey(),
        )
        .await?;

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

    let amount_of_generated_profits =
        ui_amount_to_native_amount(1_000_000, collateral_mint_decimals);

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Prepare the program state to be ready,
    // -- Set all depository caps for proper target computation
    // -- Mint a bunch using identity_depository to fill it up above its target
    // ---------------------------------------------------------------------

    // Airdrop collateral to our authority, this collateral will be used for depositing as profits to alloyx vault
    program_spl::instructions::process_token_mint_to(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &collateral_mint,
        &authority_collateral,
        amount_of_generated_profits,
    )
    .await?;

    // Airdrop a tiny amount of collateral to our payer (to pay rebalance precision loss)
    program_spl::instructions::process_token_mint_to(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &collateral_mint,
        &payer_collateral,
        1_000,
    )
    .await?;

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
                identity_depository_weight_bps: 40 * 100,        // 40%
                mercurial_vault_depository_weight_bps: 25 * 100, // 25%
                credix_lp_depository_weight_bps: 25 * 100,       // 25%
                alloyx_vault_depository_weight_bps: 10 * 100,    // 10%
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Now we set the router depositories to the correct PDAs
    program_uxd::procedures::process_set_controller_router_depositories(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
    )
    .await?;

    // Setup the fees, caps and profits beneficiary for router depositories
    program_uxd::procedures::process_setup_router_depositories_fields(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        amount_we_use_as_supply_cap,
        Some(100),
        Some(100),
        Some(false),
        Some(profits_beneficiary_collateral),
    )
    .await?;

    // Minting on identity_depository should work now that everything is set
    program_uxd::instructions::process_mint_with_identity_depository(
        &mut program_context,
        &payer,
        &authority,
        &user,
        &user_collateral,
        &user_redeemable,
        amount_the_user_should_be_able_to_mint,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Now that identity_depository has some over-weight and alloyx is underweight
    // -- Trigger a rebalance, then change the weights and rebalance again.
    // -- Then change the weights again to make alloyx overweight and rebalance again
    // ---------------------------------------------------------------------

    // Alloyx should be 10% underweight, so rebalancing should deposit 10% of the supply into alloyx
    // note: we hardcoded the precision-loss corrections to keep things simple, but the precision loss is handled at the protocol level
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(i128::from(
            amount_the_user_should_be_able_to_mint * 10 / 100 - 1,
        )), // 10% deposit (+ precision-loss)
        Some(0),
    )
    .await?;

    // Set the controller's router weights (increase alloyx weight)
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 40 * 100,        // 40%
                mercurial_vault_depository_weight_bps: 25 * 100, // 25%
                credix_lp_depository_weight_bps: 20 * 100,       // 20%
                alloyx_vault_depository_weight_bps: 15 * 100,    // 15%
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Alloyx should be 5% underweight, so rebalancing should deposit 5% of the supply into alloyx
    // note: we hardcoded the precision-loss corrections to keep things simple, but the precision loss is handled at the protocol level
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(i128::from(
            amount_the_user_should_be_able_to_mint * 5 / 100 - 1,
        )), // 5% deposit (+ precision-loss)
        Some(0),
    )
    .await?;

    // Set the controller's router weights (decrease alloyx weight)
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 40 * 100,        // 40%
                mercurial_vault_depository_weight_bps: 25 * 100, // 25%
                credix_lp_depository_weight_bps: 30 * 100,       // 30%
                alloyx_vault_depository_weight_bps: 5 * 100,     // 5%
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Alloyx should be 10% overweight, so rebalancing should withdraw 10% of the supply into alloyx
    // note: we hardcoded the precision-loss corrections to keep things simple, but the precision loss is handled at the protocol level
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(-i128::from(
            amount_the_user_should_be_able_to_mint * 10 / 100 - 2,
        )), // 10% withdrawal (+ precision-loss)
        Some(2), // +precision-loss
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Simulate profits generation inside of the alloyx vault by depositing collaeral
    // -- Rebalancing should withdraw the expected profits while while also rebalancing to the desired weight at the same time
    // ---------------------------------------------------------------------

    // At profit generation time, the collateral inside of the vault is 5% of supply
    let amount_deposited_in_alloyx_vault_at_profit_time =
        amount_the_user_should_be_able_to_mint * 5 / 100;

    // Compute how much profits we are expected to be able to collect
    let alloyx_vault_id = program_alloyx::accounts::find_vault_id();
    let alloyx_vault_info = program_alloyx::accounts::find_vault_info(&alloyx_vault_id).0;
    let alloyx_vault_collateral =
        program_alloyx::accounts::find_vault_usdc_token(&alloyx_vault_id).0;

    let alloyx_vault_info_before = program_context::read_account_anchor::<alloyx_cpi::VaultInfo>(
        &mut program_context,
        &alloyx_vault_info,
    )
    .await?;
    let alloyx_vault_collateral_before = program_context::read_account_packed::<Account>(
        &mut program_context,
        &alloyx_vault_collateral,
    )
    .await?;
    let alloyx_vault_total_collateral_before =
        alloyx_vault_info_before.wallet_desk_usdc_value + alloyx_vault_collateral_before.amount;

    let expected_profits_collateral_amount = u64::try_from(
        u128::from(amount_deposited_in_alloyx_vault_at_profit_time)
            * u128::from(amount_of_generated_profits)
            / u128::from(alloyx_vault_total_collateral_before),
    )
    .unwrap();

    // Notify that the vault has generated profits
    program_alloyx::instructions::process_set_vault_info(
        &mut program_context,
        &authority,
        &collateral_mint.pubkey(),
        amount_of_generated_profits,
    )
    .await?;

    // Deposit collateral into the alloyx vault, this collateral will be considered as profit
    program_alloyx::instructions::process_transfer_usdc_in(
        &mut program_context,
        &authority,
        &collateral_mint.pubkey(),
        amount_of_generated_profits,
    )
    .await?;

    // Set the controller's router weights (increase alloyx weight)
    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 40 * 100,        // 40%
                mercurial_vault_depository_weight_bps: 20 * 100, // 20%
                credix_lp_depository_weight_bps: 30 * 100,       // 30%
                alloyx_vault_depository_weight_bps: 10 * 100,    // 10%
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Alloyx should be 5% underweight, so rebalancing should deposit 5% of the supply into alloyx
    // It should also withdraw the expected amount of profits at the same time
    // note: we hardcoded the precision-loss corrections to keep things simple, but the precision loss is handled at the protocol level
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(i128::from(
            amount_the_user_should_be_able_to_mint * 5 / 100 - 1,
        )), // 5% deposit + precision-loss
        Some(expected_profits_collateral_amount + 4), // +precision-loss
    )
    .await?;

    // Done
    Ok(())
}
