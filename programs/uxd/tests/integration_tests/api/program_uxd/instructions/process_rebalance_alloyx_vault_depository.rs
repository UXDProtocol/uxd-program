use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;
use spl_token::state::Mint;

use uxd::state::AlloyxVaultDepository;
use uxd::state::Controller;
use uxd::state::IdentityDepository;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;

pub async fn process_rebalance_alloyx_vault_depository(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    collateral_mint: &Pubkey,
    alloyx_vault_mint: &Pubkey,
    profits_beneficiary_collateral: &Pubkey,
    expected_rebalance_delta_value: Option<i128>,
    expected_profits_amount: Option<u64>,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let payer_collateral = program_spl::instructions::process_associated_token_account_get_or_init(
        program_context,
        payer,
        collateral_mint,
        &payer.pubkey(),
    )
    .await?;

    let controller = program_uxd::accounts::find_controller_pda().0;
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint_pda().0;

    let identity_depository = program_uxd::accounts::find_identity_depository_pda().0;
    let identity_depository_collateral =
        program_uxd::accounts::find_identity_depository_collateral_vault_pda().0;

    let mercurial_base = program_mercurial::accounts::find_base();
    let mercurial_vault_depository_vault =
        program_mercurial::accounts::find_vault_pda(collateral_mint, &mercurial_base.pubkey()).0;
    let mercurial_vault_depository = program_uxd::accounts::find_mercurial_vault_depository_pda(
        collateral_mint,
        &mercurial_vault_depository_vault,
    )
    .0;

    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds).0;
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository_pda(
        collateral_mint,
        &credix_global_market_state,
    )
    .0;

    // alloyx related accounts
    let alloyx_vault_id = program_alloyx::accounts::find_vault_id();
    let alloyx_vault_info = program_alloyx::accounts::find_vault_info(&alloyx_vault_id).0;
    let alloyx_vault_collateral =
        program_alloyx::accounts::find_vault_usdc_token(&alloyx_vault_id).0;
    let alloyx_vault_shares = program_alloyx::accounts::find_vault_alloyx_token(&alloyx_vault_id).0;
    let alloyx_vault_depository = program_uxd::accounts::find_alloyx_vault_depository_pda(
        &alloyx_vault_info,
        collateral_mint,
    )
    .0;
    let alloyx_vault_depository_collateral =
        program_uxd::accounts::find_alloyx_vault_depository_collateral(
            &alloyx_vault_depository,
            collateral_mint,
        );
    let alloyx_vault_depository_shares = program_uxd::accounts::find_alloyx_vault_depository_shares(
        &alloyx_vault_depository,
        alloyx_vault_mint,
    );
    let alloyx_vault_pass =
        program_alloyx::accounts::find_investor_pass(&alloyx_vault_id, &alloyx_vault_depository).0;

    // Read state before
    let redeemable_mint_before =
        program_context::read_account_packed::<Mint>(program_context, &redeemable_mint).await?;
    let controller_before =
        program_context::read_account_anchor::<Controller>(program_context, &controller).await?;
    let identity_depository_before = program_context::read_account_anchor::<IdentityDepository>(
        program_context,
        &identity_depository,
    )
    .await?;
    let identity_depository_collateral_before = program_context::read_account_packed::<Account>(
        program_context,
        &identity_depository_collateral,
    )
    .await?;
    let alloyx_vault_mint_before =
        program_context::read_account_packed::<Mint>(program_context, alloyx_vault_mint).await?;
    let alloyx_vault_info_before = program_context::read_account_anchor::<alloyx_cpi::VaultInfo>(
        program_context,
        &alloyx_vault_info,
    )
    .await?;
    let alloyx_vault_collateral_before =
        program_context::read_account_packed::<Account>(program_context, &alloyx_vault_collateral)
            .await?;
    let alloyx_vault_depository_before = program_context::read_account_anchor::<
        AlloyxVaultDepository,
    >(program_context, &alloyx_vault_depository)
    .await?;
    let alloyx_vault_depository_shares_before = program_context::read_account_packed::<Account>(
        program_context,
        &alloyx_vault_depository_shares,
    )
    .await?;
    let profits_beneficiary_collateral_before = program_context::read_account_packed::<Account>(
        program_context,
        profits_beneficiary_collateral,
    )
    .await?;

    // Execute IX
    let accounts = uxd::accounts::RebalanceAlloyxVaultDepository {
        payer: payer.pubkey(),
        payer_collateral,
        controller,
        collateral_mint: *collateral_mint,
        identity_depository,
        identity_depository_collateral,
        mercurial_vault_depository,
        credix_lp_depository,
        alloyx_vault_depository,
        alloyx_vault_depository_collateral,
        alloyx_vault_depository_shares,
        alloyx_vault_info,
        alloyx_vault_collateral,
        alloyx_vault_shares,
        alloyx_vault_mint: *alloyx_vault_mint,
        alloyx_vault_pass,
        profits_beneficiary_collateral: *profits_beneficiary_collateral,
        system_program: solana_sdk::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        alloyx_program: alloyx_cpi::ID,
    };
    let payload = uxd::instruction::RebalanceAlloyxVaultDepository {
        vault_id: alloyx_vault_id,
    };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction(program_context, instruction, payer).await?;

    // Read state after
    let redeemable_mint_after =
        program_context::read_account_packed::<Mint>(program_context, &redeemable_mint).await?;
    let controller_after =
        program_context::read_account_anchor::<Controller>(program_context, &controller).await?;
    let identity_depository_after = program_context::read_account_anchor::<IdentityDepository>(
        program_context,
        &identity_depository,
    )
    .await?;
    let identity_depository_collateral_after = program_context::read_account_packed::<Account>(
        program_context,
        &identity_depository_collateral,
    )
    .await?;
    let alloyx_vault_mint_after =
        program_context::read_account_packed::<Mint>(program_context, alloyx_vault_mint).await?;
    let alloyx_vault_info_after = program_context::read_account_anchor::<alloyx_cpi::VaultInfo>(
        program_context,
        &alloyx_vault_info,
    )
    .await?;
    let alloyx_vault_collateral_after =
        program_context::read_account_packed::<Account>(program_context, &alloyx_vault_collateral)
            .await?;
    let alloyx_vault_depository_after =
        program_context::read_account_anchor::<AlloyxVaultDepository>(
            program_context,
            &alloyx_vault_depository,
        )
        .await?;
    let alloyx_vault_depository_shares_after = program_context::read_account_packed::<Account>(
        program_context,
        &alloyx_vault_depository_shares,
    )
    .await?;
    let profits_beneficiary_collateral_after = program_context::read_account_packed::<Account>(
        program_context,
        profits_beneficiary_collateral,
    )
    .await?;

    // MOST IMPORTANT: Ensure that we never lose value for the protocol!
    let protocol_alloyx_value_before = u128::from(alloyx_vault_depository_shares_before.amount)
        * (u128::from(alloyx_vault_info_before.wallet_desk_usdc_value)
            + u128::from(alloyx_vault_collateral_before.amount))
        / u128::from(alloyx_vault_mint_before.supply);
    let protocol_alloyx_value_after = u128::from(alloyx_vault_depository_shares_after.amount)
        * (u128::from(alloyx_vault_info_after.wallet_desk_usdc_value)
            + u128::from(alloyx_vault_collateral_after.amount))
        / u128::from(alloyx_vault_mint_after.supply);
    let protocol_total_value_before = protocol_alloyx_value_before
        + u128::from(identity_depository_collateral_before.amount)
        + u128::from(profits_beneficiary_collateral_before.amount);
    let protocol_total_value_after = protocol_alloyx_value_after
        + u128::from(identity_depository_collateral_after.amount)
        + u128::from(profits_beneficiary_collateral_after.amount);
    assert!(
        protocol_total_value_before <= protocol_total_value_after,
        "protocol value loss!"
    );

    // redeemable_mint.supply must stay unchanged
    let redeemable_mint_supply_before = redeemable_mint_before.supply;
    let redeemable_mint_supply_after = redeemable_mint_after.supply;
    assert_eq!(redeemable_mint_supply_before, redeemable_mint_supply_after,);

    // controller.redeemable_circulating_supply must stay unchanged
    let redeemable_circulating_supply_before =
        u64::try_from(controller_before.redeemable_circulating_supply).unwrap();
    let redeemable_circulating_supply_after =
        u64::try_from(controller_after.redeemable_circulating_supply).unwrap();
    assert_eq!(
        redeemable_circulating_supply_before,
        redeemable_circulating_supply_after,
    );

    // If we want to check that the rebalance delta is valid
    if let Some(expected_rebalance_delta_value) = expected_rebalance_delta_value {
        // identity_depository.redeemable_amount_under_management must have moved by the expected rebalance amount
        let identity_depository_redeemable_amount_under_management_before =
            identity_depository_before.redeemable_amount_under_management;
        let identity_depository_redeemable_amount_under_management_after =
            identity_depository_after.redeemable_amount_under_management;
        assert_eq!(
            i128::try_from(identity_depository_redeemable_amount_under_management_before).unwrap()
                - expected_rebalance_delta_value,
            i128::try_from(identity_depository_redeemable_amount_under_management_after).unwrap(),
            "invalid identity_depository.redeemable_amount_under_management",
        );
        // alloyx_vault_depository.redeemable_amount_under_management must have moved by the expected rebalance amount
        let alloyx_vault_depository_redeemable_amount_under_management_before =
            alloyx_vault_depository_before.redeemable_amount_under_management;
        let alloyx_vault_depository_redeemable_amount_under_management_after =
            alloyx_vault_depository_after.redeemable_amount_under_management;
        assert_eq!(
            i128::from(alloyx_vault_depository_redeemable_amount_under_management_before)
                + expected_rebalance_delta_value,
            i128::from(alloyx_vault_depository_redeemable_amount_under_management_after),
            "invalid alloyx_vault_depository.redeemable_amount_under_management"
        );

        // identity_depository.collateral_amount_deposited must have moved by the expected rebalance amount
        let identity_depository_collateral_amount_deposited_before =
            identity_depository_before.collateral_amount_deposited;
        let identity_depository_collateral_amount_deposited_after =
            identity_depository_after.collateral_amount_deposited;
        assert_eq!(
            i128::try_from(identity_depository_collateral_amount_deposited_before).unwrap()
                - expected_rebalance_delta_value,
            i128::try_from(identity_depository_collateral_amount_deposited_after).unwrap(),
            "invalid identity_depository.collateral_amount_deposited"
        );
        // alloyx_vault_depository.collateral_amount_deposited must have moved by the expected rebalance amount
        let alloyx_vault_depository_collateral_amount_deposited_before =
            alloyx_vault_depository_before.collateral_amount_deposited;
        let alloyx_vault_depository_collateral_amount_deposited_after =
            alloyx_vault_depository_after.collateral_amount_deposited;
        assert_eq!(
            i128::from(alloyx_vault_depository_collateral_amount_deposited_before)
                + expected_rebalance_delta_value,
            i128::from(alloyx_vault_depository_collateral_amount_deposited_after),
            "invalid alloyx_vault_depository.collateral_amount_deposited"
        );
    }

    if let Some(expected_profits_amount) = expected_profits_amount {
        // controller.profits_amount_collected must have increased by the profits amount
        let controller_profits_total_collected_before =
            u64::try_from(controller_before.profits_total_collected).unwrap();
        let controller_profits_total_collected_after =
            u64::try_from(controller_after.profits_total_collected).unwrap();
        assert_eq!(
            controller_profits_total_collected_before + expected_profits_amount,
            controller_profits_total_collected_after,
            "invalid controller.profits_total_collected",
        );

        // profits_beneficiary_collateral.amount must have increased by the expected profits amount
        let profits_beneficiary_collateral_amount_before =
            profits_beneficiary_collateral_before.amount;
        let profits_beneficiary_collateral_amount_after =
            profits_beneficiary_collateral_after.amount;
        assert_eq!(
            profits_beneficiary_collateral_amount_before + expected_profits_amount,
            profits_beneficiary_collateral_amount_after,
            "invalid profits_beneficiary_collateral.amount"
        );

        // alloyx_vault_depository.profits_amount_collected must have increased by the expected profits amount
        let alloyx_vault_depository_profits_total_collected_before =
            alloyx_vault_depository_before.profits_total_collected;
        let alloyx_vault_depository_profits_total_collected_after =
            alloyx_vault_depository_after.profits_total_collected;
        assert_eq!(
            alloyx_vault_depository_profits_total_collected_before + expected_profits_amount,
            alloyx_vault_depository_profits_total_collected_after,
            "invalid alloyx_vault_depository.profits_amount_collected",
        );
    }

    // Done
    Ok(())
}
