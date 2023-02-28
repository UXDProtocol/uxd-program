use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;

pub async fn test_mint_with_identity_depository(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    collateral_amount: u64,
) -> Result<(), String> {
    // Read state before
    let redeemable_mint_before =
        program_spl::accounts::read_token_mint(program_test_context, &program_keys.redeemable_mint)
            .await?;
    let controller_before =
        program_uxd::accounts::read_controller(program_test_context, &program_keys.controller)
            .await?;
    let identity_depository_before = program_uxd::accounts::read_identity_depository(
        program_test_context,
        &program_keys.identity_depository_keys.depository,
    )
    .await?;

    let redeemable_mint_supply_before = redeemable_mint_before.supply;
    let redeemable_circulating_supply_before =
        u64::try_from(controller_before.redeemable_circulating_supply).unwrap();
    let redeemable_amount_under_management_before =
        u64::try_from(identity_depository_before.redeemable_amount_under_management).unwrap();
    let collateral_amount_deposited_before =
        u64::try_from(identity_depository_before.collateral_amount_deposited).unwrap();

    let user_collateral_amount_before =
        program_spl::accounts::read_token_account(program_test_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_before =
        program_spl::accounts::read_token_account(program_test_context, user_redeemable)
            .await?
            .amount;

    // Execute
    program_uxd::instructions::process_mint_with_identity_depository(
        program_test_context,
        program_keys,
        payer,
        user,
        user_collateral,
        user_redeemable,
        collateral_amount,
    )
    .await?;

    // Read state after
    let redeemable_mint_after =
        program_spl::accounts::read_token_mint(program_test_context, &program_keys.redeemable_mint)
            .await?;
    let controller_after =
        program_uxd::accounts::read_controller(program_test_context, &program_keys.controller)
            .await?;
    let identity_depository_after = program_uxd::accounts::read_identity_depository(
        program_test_context,
        &program_keys.identity_depository_keys.depository,
    )
    .await?;

    let redeemable_mint_supply_after = redeemable_mint_after.supply;
    let redeemable_circulating_supply_after =
        u64::try_from(controller_after.redeemable_circulating_supply).unwrap();
    let redeemable_amount_under_management_after =
        u64::try_from(identity_depository_after.redeemable_amount_under_management).unwrap();
    let collateral_amount_deposited_after =
        u64::try_from(identity_depository_after.collateral_amount_deposited).unwrap();

    let user_collateral_amount_after =
        program_spl::accounts::read_token_account(program_test_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_after =
        program_spl::accounts::read_token_account(program_test_context, user_redeemable)
            .await?
            .amount;

    // Check result
    assert_eq!(
        redeemable_mint_supply_before + collateral_amount,
        redeemable_mint_supply_after,
    );
    assert_eq!(
        redeemable_circulating_supply_before + collateral_amount,
        redeemable_circulating_supply_after,
    );
    assert_eq!(
        redeemable_amount_under_management_before + collateral_amount,
        redeemable_amount_under_management_after,
    );
    assert_eq!(
        collateral_amount_deposited_before + collateral_amount,
        collateral_amount_deposited_after,
    );

    assert_eq!(
        user_collateral_amount_before - collateral_amount,
        user_collateral_amount_after,
    );
    assert_eq!(
        user_redeemable_amount_before + collateral_amount,
        user_redeemable_amount_after,
    );

    // Done
    Ok(())
}
