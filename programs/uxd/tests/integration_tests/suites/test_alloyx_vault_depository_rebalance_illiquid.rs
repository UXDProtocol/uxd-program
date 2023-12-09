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
async fn test_alloyx_vault_depository_rebalance_illiquid(
) -> Result<(), program_context::ProgramError> {
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
                credix_lp_depository_weight_bps: 15 * 100,       // 15%
                alloyx_vault_depository_weight_bps: 20 * 100,    // 20%
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
    // -- We trigger a rebalance to make a deposit into the alloyx vault.
    // -- After that we siphon off the liquidity from the alloyx vault, we simulate profits.
    // -- Rebalancing should be blocked until liquidity comes back
    // ---------------------------------------------------------------------

    // The first rebalance will deposit 20% of supply to alloyx
    let amount_first_deposited_into_alloyx = amount_the_user_should_be_able_to_mint * 20 / 100;

    // Alloyx should be 20% underweight, so rebalancing should deposit 20% of the supply into alloyx
    // note: we hardcoded the precision-loss corrections to keep things simple, but the precision loss is handled at the protocol level
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(i128::from(amount_first_deposited_into_alloyx - 1)), // 20% deposit (+ precision-loss)
        Some(0),
    )
    .await?;

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
        u128::from(amount_first_deposited_into_alloyx) * u128::from(amount_of_generated_profits)
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

    // Siphon all of the liquidity out of the alloyx vault
    program_alloyx::instructions::process_transfer_usdc_out(
        &mut program_context,
        &authority,
        &collateral_mint.pubkey(),
        alloyx_vault_collateral_before.amount,
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
                identity_depository_weight_bps: 45 * 100,        // 45%
                mercurial_vault_depository_weight_bps: 25 * 100, // 25%
                credix_lp_depository_weight_bps: 25 * 100,       // 25%
                alloyx_vault_depository_weight_bps: 5 * 100,     // 5%
            }),
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // Alloyx should be 15% overweight, so rebalancing should try to withdraw 15% of the supply into alloyx
    // No available liquidity is there tho, so nothing can be done yet
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(0), // nothing can happen yet
        Some(0), // nothing can happen yet
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Now that there is zero liquidity in the vault, drip back some liquidity
    // -- Then we watch the rebalancing do its best effort to collect profit and do rebalancing when liquidity becomes available
    // ---------------------------------------------------------------------

    let amount_of_liquidity_collateral_first_unlock =
        ui_amount_to_native_amount(10, collateral_mint_decimals);

    let amount_of_liquidity_collateral_second_unlock =
        ui_amount_to_native_amount(5_000_000, collateral_mint_decimals);

    let amount_of_liquidity_collateral_third_unlock = amount_of_generated_profits
        + alloyx_vault_collateral_before.amount
        - amount_of_liquidity_collateral_first_unlock
        - amount_of_liquidity_collateral_second_unlock;

    // Return a little bit of liquidity back to the vault
    program_alloyx::instructions::process_transfer_usdc_in(
        &mut program_context,
        &authority,
        &collateral_mint.pubkey(),
        amount_of_liquidity_collateral_first_unlock,
    )
    .await?;

    // The rebalancing should prioritize profits on a best-effort
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(0), // no liquidity available for rebalancing
        Some(amount_of_liquidity_collateral_first_unlock), // all liquidity goes toward profits
    )
    .await?;

    // Return a little bit of liquidity back to the vault
    program_alloyx::instructions::process_transfer_usdc_in(
        &mut program_context,
        &authority,
        &collateral_mint.pubkey(),
        amount_of_liquidity_collateral_second_unlock,
    )
    .await?;

    // The rebalancing can start rebalancing only once all profits was collected
    // note: we hardcoded the precision-loss corrections to keep things simple, but the precision loss is handled at the protocol level
    let amount_of_profits_for_second_unlock =
        expected_profits_collateral_amount - amount_of_liquidity_collateral_first_unlock;
    let amount_of_rebalancing_for_second_unlock =
        amount_of_liquidity_collateral_second_unlock - amount_of_profits_for_second_unlock;
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(-i128::from(amount_of_rebalancing_for_second_unlock)), // partial rebalancing if possible
        Some(amount_of_profits_for_second_unlock + 2), // profits prioritized (+ precision-loss)
    )
    .await?;

    // Return a little bit of liquidity back to the vault
    program_alloyx::instructions::process_transfer_usdc_in(
        &mut program_context,
        &authority,
        &collateral_mint.pubkey(),
        amount_of_liquidity_collateral_third_unlock,
    )
    .await?;

    // The rebalancing can start rebalancing once all profits was collected
    // note: we hardcoded the precision-loss corrections to keep things simple, but the precision loss is handled at the protocol level
    let amount_final_expected_in_alloyx_depository =
        amount_the_user_should_be_able_to_mint * 5 / 100; // 5% weight
    let amount_of_rebalancing_for_third_unlock = amount_first_deposited_into_alloyx
        - amount_final_expected_in_alloyx_depository
        - amount_of_rebalancing_for_second_unlock;
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(-i128::from(amount_of_rebalancing_for_third_unlock - 1)), // partial rebalancing (+ precision-loss)
        Some(2), // no more profits to collect (+ precision-loss)
    )
    .await?;

    // The rebalancing should now be finished
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(0), // finished rebalancing
        Some(2), // no more profits to collect (+ precision-loss)
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 5
    // -- After all this, syphoon liquidity out of identity_depository
    // -- Then we make sure that the alloyx_vault_depository is trying its best to maintain the identity_depository full
    // ---------------------------------------------------------------------

    let amount_of_removed_liquidity = amount_the_user_should_be_able_to_mint * 95 / 100;
    let amount_of_backup_liquidity =
        (amount_the_user_should_be_able_to_mint - amount_of_removed_liquidity) * 95 / 100;

    // Removing the liquidity from the identity_depository should make the alloyx_vault_depository want to withdraw to fill it up
    program_uxd::instructions::process_redeem_from_identity_depository(
        &mut program_context,
        &payer,
        &authority,
        &user,
        &user_collateral,
        &user_redeemable,
        amount_of_removed_liquidity,
    )
    .await?;

    // The alloyx_vault_depository should sacrifice itself to fill up the identity_depository
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(-i128::from(amount_of_backup_liquidity)), // try to fill the missing liquidity by emptying itself
        Some(0),                                       // no more profits to collect
    )
    .await?;

    // Putting back the liquidity in the identity_depository should make the alloyx_vault_depository want to replenish itself up
    program_uxd::instructions::process_mint_with_identity_depository(
        &mut program_context,
        &payer,
        &authority,
        &user,
        &user_collateral,
        &user_redeemable,
        amount_of_removed_liquidity,
    )
    .await?;

    // The alloyx_vault_depository should want to replenish itself
    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint.pubkey(),
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        Some(i128::from(amount_of_backup_liquidity - 1)), // recover the liquidity to fill itself up (-precision-loss)
        Some(2), // no more profits to collect (+precision-loss)
    )
    .await?;

    // Done
    Ok(())
}
