use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::clock::Clock;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;
use spl_token::state::Mint;

use uxd::state::Controller;
use uxd::state::MercurialVaultDepository;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_collect_profits_of_mercurial_vault_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    collateral_mint: &Pubkey,
    mercurial_vault_lp_mint: &Pubkey,
    profits_beneficiary_collateral: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let mercurial_base = program_mercurial::accounts::find_base();
    let mercurial_vault =
        program_mercurial::accounts::find_vault_pda(collateral_mint, &mercurial_base.pubkey()).0;
    let mercurial_vault_depository = program_uxd::accounts::find_mercurial_vault_depository_pda(
        collateral_mint,
        &mercurial_vault,
    )
    .0;
    let mercurial_vault_depository_lp_token_vault =
        program_uxd::accounts::find_mercurial_vault_depository_lp_token_vault_pda(
            collateral_mint,
            &mercurial_vault,
        )
        .0;
    let mercurial_vault_collateral_token_safe =
        program_mercurial::accounts::find_token_vault_pda(&mercurial_vault).0;

    // Read state before
    let controller_before =
        program_test_context::read_account_anchor::<Controller>(program_test_context, &controller)
            .await?;
    let mercurial_vault_depository_before = program_test_context::read_account_anchor::<
        MercurialVaultDepository,
    >(program_test_context, &mercurial_vault_depository)
    .await?;
    let mercurial_vault_before =
        program_test_context::read_account_anchor::<mercurial_vault::Vault>(
            program_test_context,
            &mercurial_vault,
        )
        .await?;
    let mercurial_vault_lp_mint_before = program_test_context::read_account_packed::<Mint>(
        program_test_context,
        mercurial_vault_lp_mint,
    )
    .await?;

    let mercurial_vault_depository_lp_token_vault_amount_before =
        program_test_context::read_account_packed::<Account>(
            program_test_context,
            &mercurial_vault_depository_lp_token_vault,
        )
        .await?
        .amount;

    let profits_beneficiary_collateral_amount_before =
        program_test_context::read_account_packed::<Account>(
            program_test_context,
            profits_beneficiary_collateral,
        )
        .await?
        .amount;

    let unix_timestamp_before = u64::try_from(
        program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .map_err(program_test_context::ProgramTestError::BanksClient)?
            .unix_timestamp,
    )
    .unwrap();

    // Execute IX
    let accounts = uxd::accounts::CollectProfitsOfMercurialVaultDepository {
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        depository: mercurial_vault_depository,
        depository_lp_token_vault: mercurial_vault_depository_lp_token_vault,
        mercurial_vault,
        mercurial_vault_lp_mint: *mercurial_vault_lp_mint,
        mercurial_vault_collateral_token_safe,
        profits_beneficiary_collateral: *profits_beneficiary_collateral,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        mercurial_vault_program: mercurial_vault::ID,
    };
    let payload = uxd::instruction::CollectProfitsOfMercurialVaultDepository {};
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_test_context, instruction, payer).await?;

    // Read state after
    let controller_after =
        program_test_context::read_account_anchor::<Controller>(program_test_context, &controller)
            .await?;
    let mercurial_vault_depository_after = program_test_context::read_account_anchor::<
        MercurialVaultDepository,
    >(program_test_context, &mercurial_vault_depository)
    .await?;
    let mercurial_vault_after =
        program_test_context::read_account_anchor::<mercurial_vault::Vault>(
            program_test_context,
            &mercurial_vault,
        )
        .await?;
    let mercurial_vault_lp_mint_after = program_test_context::read_account_packed::<Mint>(
        program_test_context,
        mercurial_vault_lp_mint,
    )
    .await?;

    let mercurial_vault_depository_lp_token_vault_amount_after =
        program_test_context::read_account_packed::<Account>(
            program_test_context,
            &mercurial_vault_depository_lp_token_vault,
        )
        .await?
        .amount;

    let profits_beneficiary_collateral_amount_after =
        program_test_context::read_account_packed::<Account>(
            program_test_context,
            profits_beneficiary_collateral,
        )
        .await?
        .amount;

    let unix_timestamp_after = u64::try_from(
        program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .map_err(program_test_context::ProgramTestError::BanksClient)?
            .unix_timestamp,
    )
    .unwrap();

    // Compute Assets and Liabilities
    let assets_value_before = mercurial_vault_before
        .get_amount_by_share(
            unix_timestamp_before,
            mercurial_vault_depository_lp_token_vault_amount_before,
            mercurial_vault_lp_mint_before.supply,
        )
        .unwrap();
    let assets_value_after = mercurial_vault_after
        .get_amount_by_share(
            unix_timestamp_after,
            mercurial_vault_depository_lp_token_vault_amount_after,
            mercurial_vault_lp_mint_after.supply,
        )
        .unwrap();

    let liabilities_value_before =
        u64::try_from(mercurial_vault_depository_before.redeemable_amount_under_management)
            .unwrap();
    let liabilities_value_after =
        u64::try_from(mercurial_vault_depository_after.redeemable_amount_under_management).unwrap();

    // Compute the amount of expected profits to be withdrawn
    let profits_collateral_amount = assets_value_before - liabilities_value_before;
    let profits_lp_amount = mercurial_vault_before
        .get_unmint_amount(
            unix_timestamp_before,
            profits_collateral_amount,
            mercurial_vault_lp_mint_before.supply,
        )
        .unwrap();

    // Check result
    let mercurial_vault_lp_mint_supply_before = mercurial_vault_lp_mint_before.supply;
    let mercurial_vault_lp_mint_supply_after = mercurial_vault_lp_mint_after.supply;
    assert_eq!(
        mercurial_vault_lp_mint_supply_before - profits_lp_amount,
        mercurial_vault_lp_mint_supply_after,
    );
    let controller_profits_total_collected_before = controller_before.profits_total_collected;
    let controller_profits_total_collected_after = controller_after.profits_total_collected;
    assert_eq!(
        controller_profits_total_collected_before + u128::from(profits_collateral_amount),
        controller_profits_total_collected_after,
    );
    let mercurial_vault_depository_profits_total_collected_before =
        mercurial_vault_depository_before.profits_total_collected;
    let mercurial_vault_depository_profits_total_collected_after =
        mercurial_vault_depository_after.profits_total_collected;
    assert_eq!(
        mercurial_vault_depository_profits_total_collected_before
            + u128::from(profits_collateral_amount),
        mercurial_vault_depository_profits_total_collected_after,
    );

    assert_eq!(
        mercurial_vault_depository_lp_token_vault_amount_before - profits_lp_amount,
        mercurial_vault_depository_lp_token_vault_amount_after,
    );
    assert_eq!(
        profits_beneficiary_collateral_amount_before + profits_collateral_amount,
        profits_beneficiary_collateral_amount_after,
    );

    assert_eq!(
        assets_value_before - profits_collateral_amount,
        assets_value_after,
    );
    assert_eq!(liabilities_value_before, liabilities_value_after);

    // Done
    Ok(())
}
