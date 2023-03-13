use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditCredixLpDepositoryFields;
use uxd::instructions::EditMercurialVaultDepositoryFields;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::utils::ui_amount_to_native_amount;

fn percent_to_bps(percent: u16) -> u16 {
    percent * 100
}

#[tokio::test]
async fn test_compute_depositories_targets() -> Result<(), program_test_context::ProgramTestError> {
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
        &program_uxd::accounts::find_redeemable_mint_pda().0,
        &user.pubkey(),
    )
    .await?;

    // Useful amounts used during testing scenario
    let amount_we_use_as_supply_cap = ui_amount_to_native_amount(300, redeemable_mint_decimals);

    let amount_of_collateral_airdropped_to_user =
        ui_amount_to_native_amount(1000, collateral_mint_decimals);

    let amount_of_collateral_supply_after_the_first_mint =
        ui_amount_to_native_amount(50, collateral_mint_decimals);
    let amount_of_collateral_supply_after_the_second_mint =
        ui_amount_to_native_amount(150, collateral_mint_decimals);

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Initialize the supply of mercurial and mint into mercurial
    // -- Also set the weights of mercurial and credix
    // -- Then compute the weights while credix is empty and capped to zero
    // -- Since the supply cap of credix should be zero, no matter the weights, mercurial will get all
    // ---------------------------------------------------------------------

    // Set the controller cap to allow for some minting
    program_uxd::instructions::process_edit_controller(
        &mut program_test_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(amount_we_use_as_supply_cap.into()),
        },
    )
    .await?;

    // Edit the mercurial vault depository:
    // - set the depository supply cap
    // - make sure minting is not disabled (and no fees)
    // - set the depository weight to 50%
    program_uxd::instructions::process_edit_mercurial_vault_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditMercurialVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(0),
            redeeming_fee_in_bps: Some(0),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight_bps: Some(percent_to_bps(50)), // 50%
        },
    )
    .await?;

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

    // Mint some in the mercurial first
    program_uxd::instructions::process_mint_with_mercurial_vault_depository(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &mercurial_vault_lp_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_of_collateral_supply_after_the_first_mint,
    )
    .await?;

    // Compute the targets, mercurial should get 50% (equal to its target)
    // Credix is unitinialized, so we give it 0% because its target is 0%
    program_uxd::instructions::process_compute_depositories_targets(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        amount_of_collateral_supply_after_the_first_mint * 50 / 100, // 50%
        0,                                                           // 0%
    )
    .await?;

    // Set the credix weight but not the supply cap
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
            redeemable_amount_under_management_weight_bps: Some(percent_to_bps(50)), // 50%
        },
    )
    .await?;

    // Recompute the targets, mercurial should now get everything
    // since credix is capped to zero, and if its weight is set to 50%
    // all of the credix 50% weight should be re-allocated to mercurial
    program_uxd::instructions::process_compute_depositories_targets(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        amount_of_collateral_supply_after_the_first_mint, // 100%
        0,                                                // 0%
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Now that all liquidity is in mercurial, opening the supply cap of credix
    // -- Should allow us to recompute the targets taking credix into account,
    // -- and half of the supply should then be allocated to credix (since it now has space)
    // ---------------------------------------------------------------------

    // Set just the credix cap and make sure minting is not disabled (no fees for simple math)
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(amount_we_use_as_supply_cap.into()),
            minting_fee_in_bps: Some(0),
            redeeming_fee_in_bps: Some(0),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight_bps: None,
        },
    )
    .await?;

    // Recompute the targets, mercurial should now only get HALF of everything
    // Since credix weight is 50%, and mercurial weight is 50%
    // And since both depository has available space within their caps
    program_uxd::instructions::process_compute_depositories_targets(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        amount_of_collateral_supply_after_the_first_mint * 50 / 100, // 50%
        amount_of_collateral_supply_after_the_first_mint * 50 / 100, // 50%
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- We then increase the total supply by minting more into credix
    // -- Recomputing the targets should give us different value based on the new supply
    // -- Should still be balanced 50%/50%
    // ---------------------------------------------------------------------

    // Mint some more in credix
    program_uxd::instructions::process_mint_with_credix_lp_depository(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        &user,
        &user_collateral,
        &user_redeemable,
        amount_of_collateral_supply_after_the_second_mint
            - amount_of_collateral_supply_after_the_first_mint,
    )
    .await?;

    // Recompute the targets, new supply should be reflected in the targets
    // Since credix weight is 50%, and mercurial weight is 50%
    // And since both depository has available space within their caps
    program_uxd::instructions::process_compute_depositories_targets(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        amount_of_collateral_supply_after_the_second_mint * 50 / 100, // 50%
        amount_of_collateral_supply_after_the_second_mint * 50 / 100, // 50%
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 5
    // -- We change the cap of mercurial to 25% of total supply (even if its weight is 50%)
    // -- Recomputing the targets should now reallocate all extra to credix
    // -- Should now be balanced 25%/75% even if the weights are 50%/50%
    // ---------------------------------------------------------------------

    // Edit the mercurial vault depository and make its cap smaller than its content
    program_uxd::instructions::process_edit_mercurial_vault_depository(
        &mut program_test_context,
        &payer,
        &authority,
        &collateral_mint.pubkey(),
        &EditMercurialVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(
                (amount_of_collateral_supply_after_the_second_mint * 25 / 100).into(), // 25%
            ),
            minting_fee_in_bps: None,
            redeeming_fee_in_bps: None,
            minting_disabled: None,
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight_bps: None,
        },
    )
    .await?;

    // Recompute the targets, mercurial is now capped
    // Even if weights are 50%/50%, mercurial target should be equal to its cap
    // All remaining amount should be re-allocated to credix (25%/75%)
    program_uxd::instructions::process_compute_depositories_targets(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        amount_of_collateral_supply_after_the_second_mint * 25 / 100, // 25%
        amount_of_collateral_supply_after_the_second_mint * 75 / 100, // 75%
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 6
    // -- We set the weights to 10%/90%, then we recompute the targets
    // -- After recomputing the targets, we should now see the targets at 10%/90%
    // -- Since both depositories can fit those amounts in their caps it should work
    // ---------------------------------------------------------------------

    // Set the weight of mercurial to 10
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
            profits_beneficiary_collateral: None,
            redeemable_amount_under_management_weight_bps: Some(percent_to_bps(10)), // 10%
        },
    )
    .await?;

    // Set the weight of credix to 90
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
            redeemable_amount_under_management_weight_bps: Some(percent_to_bps(90)), // 90%
        },
    )
    .await?;

    // Recompute the targets, we should now get 10%/90% split for targets too
    // Since both targets will comfortably fit in each depository, weights should be respected
    program_uxd::instructions::process_compute_depositories_targets(
        &mut program_test_context,
        &payer,
        &collateral_mint.pubkey(),
        amount_of_collateral_supply_after_the_second_mint * 10 / 100, // 10%
        amount_of_collateral_supply_after_the_second_mint * 90 / 100, // 90%
    )
    .await?;

    // Done
    Ok(())
}
