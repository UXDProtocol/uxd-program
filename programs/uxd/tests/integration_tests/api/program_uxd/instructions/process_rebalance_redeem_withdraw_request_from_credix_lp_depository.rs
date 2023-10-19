use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;
use spl_token::state::Mint;

use uxd::state::Controller;
use uxd::state::CredixLpDepository;
use uxd::state::IdentityDepository;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_uxd;

pub async fn process_rebalance_redeem_withdraw_request_from_credix_lp_depository(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    collateral_mint: &Pubkey,
    credix_multisig: &Pubkey,
    profits_beneficiary_collateral: &Pubkey,
    expected_overflow_value: u64,
    expected_profits_amount: u64,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
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

    // Find all needed credix_lp_depository accounts
    let credix_program_state = program_credix::accounts::find_program_state_pda().0;
    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds).0;
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository_pda(
        collateral_mint,
        &credix_global_market_state,
    )
    .0;
    let credix_shares_mint =
        program_credix::accounts::find_lp_token_mint_pda(&credix_market_seeds).0;
    let credix_signing_authority =
        program_credix::accounts::find_signing_authority_pda(&credix_market_seeds).0;
    let credix_liquidity_collateral = program_credix::accounts::find_liquidity_pool_token_account(
        &credix_signing_authority,
        collateral_mint,
    );
    let credix_treasury_pool = program_credix::accounts::find_treasury_pool(credix_multisig);
    let credix_treasury_pool_collateral =
        program_credix::accounts::find_treasury_pool_token_account(
            &credix_treasury_pool,
            collateral_mint,
        );
    let credix_treasury = program_credix::accounts::find_credix_treasury(credix_multisig);
    let credix_treasury_collateral = program_credix::accounts::find_credix_treasury_token_account(
        &credix_treasury,
        collateral_mint,
    );
    let credix_pass = program_credix::accounts::find_credix_pass_pda(
        &credix_global_market_state,
        &credix_lp_depository,
    )
    .0;
    let credix_lp_depository_collateral =
        program_uxd::accounts::find_credix_lp_depository_collateral(
            &credix_lp_depository,
            collateral_mint,
        );
    let credix_lp_depository_shares = program_uxd::accounts::find_credix_lp_depository_shares(
        &credix_lp_depository,
        &credix_shares_mint,
    );

    // Find the credix withdraw accounts
    let credix_latest_withdraw_epoch_idx = program_context::read_account_anchor::<
        credix_client::GlobalMarketState,
    >(program_context, &credix_global_market_state)
    .await?
    .latest_withdraw_epoch_idx;
    let credix_withdraw_epoch = program_credix::accounts::find_withdraw_epoch_pda(
        &credix_global_market_state,
        credix_latest_withdraw_epoch_idx,
    )
    .0;

    // alloyx related accounts
    let alloyx_vault_id = program_alloyx::accounts::find_vault_id();
    let alloyx_vault_info = program_alloyx::accounts::find_vault_info(&alloyx_vault_id).0;
    let alloyx_vault_depository = program_uxd::accounts::find_alloyx_vault_depository_pda(
        &alloyx_vault_info,
        collateral_mint,
    )
    .0;

    // Read state before
    let redeemable_mint_before =
        program_context::read_account_packed::<Mint>(program_context, &redeemable_mint).await?;
    let controller_before =
        program_context::read_account_anchor::<Controller>(program_context, &controller).await?;
    let credix_lp_depository_before = program_context::read_account_anchor::<CredixLpDepository>(
        program_context,
        &credix_lp_depository,
    )
    .await?;
    let identity_depository_before = program_context::read_account_anchor::<IdentityDepository>(
        program_context,
        &identity_depository,
    )
    .await?;
    let identity_depository_collateral_amount_before =
        program_context::read_account_packed::<Account>(
            program_context,
            &identity_depository_collateral,
        )
        .await?
        .amount;
    let profits_beneficiary_collateral_amount_before =
        program_context::read_account_packed::<Account>(
            program_context,
            profits_beneficiary_collateral,
        )
        .await?
        .amount;

    // Execute IX
    let accounts = uxd::accounts::RebalanceRedeemWithdrawRequestFromCredixLpDepository {
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        identity_depository,
        identity_depository_collateral,
        mercurial_vault_depository,
        credix_lp_depository,
        credix_lp_depository_collateral,
        credix_lp_depository_shares,
        credix_program_state,
        credix_global_market_state,
        credix_signing_authority,
        credix_liquidity_collateral,
        credix_shares_mint,
        credix_pass,
        credix_withdraw_epoch,
        credix_treasury_pool_collateral,
        credix_treasury,
        credix_treasury_collateral,
        alloyx_vault_depository,
        profits_beneficiary_collateral: *profits_beneficiary_collateral,
        system_program: solana_sdk::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        credix_program: credix_client::ID,
        rent: solana_sdk::sysvar::rent::ID,
    };
    let payload = uxd::instruction::RebalanceRedeemWithdrawRequestFromCredixLpDepository {};
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
    let credix_lp_depository_after = program_context::read_account_anchor::<CredixLpDepository>(
        program_context,
        &credix_lp_depository,
    )
    .await?;
    let identity_depository_after = program_context::read_account_anchor::<IdentityDepository>(
        program_context,
        &identity_depository,
    )
    .await?;
    let identity_depository_collateral_amount_after =
        program_context::read_account_packed::<Account>(
            program_context,
            &identity_depository_collateral,
        )
        .await?
        .amount;
    let profits_beneficiary_collateral_amount_after =
        program_context::read_account_packed::<Account>(
            program_context,
            profits_beneficiary_collateral,
        )
        .await?
        .amount;

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

    // controller.profits_amount_collected must have increased by the profits amount
    let controller_profits_total_collected_before: u64 =
        u64::try_from(controller_before.profits_total_collected).unwrap();
    let controller_profits_total_collected_after =
        u64::try_from(controller_after.profits_total_collected).unwrap();
    assert_eq!(
        controller_profits_total_collected_before + expected_profits_amount,
        controller_profits_total_collected_after,
    );

    // credix_lp_depository.redeemable_amount_under_management must have decreased by the withdraw overflow
    let credix_lp_depository_redeemable_amount_under_management_before =
        u64::try_from(credix_lp_depository_before.redeemable_amount_under_management).unwrap();
    let credix_lp_depository_redeemable_amount_under_management_after =
        u64::try_from(credix_lp_depository_after.redeemable_amount_under_management).unwrap();
    assert_eq!(
        credix_lp_depository_redeemable_amount_under_management_before - expected_overflow_value,
        credix_lp_depository_redeemable_amount_under_management_after,
    );

    // identity_depository.redeemable_amount_under_management must have increased by the withdraw overflow
    let identity_depository_redeemable_amount_under_management_before =
        u64::try_from(identity_depository_before.redeemable_amount_under_management).unwrap();
    let identity_depository_redeemable_amount_under_management_after =
        u64::try_from(identity_depository_after.redeemable_amount_under_management).unwrap();
    assert_eq!(
        identity_depository_redeemable_amount_under_management_before + expected_overflow_value,
        identity_depository_redeemable_amount_under_management_after,
    );

    // credix_lp_depository.collateral_amount_deposited must have decreased by the withdraw overflow
    let credix_lp_depository_collateral_amount_deposited_before =
        u64::try_from(credix_lp_depository_before.collateral_amount_deposited).unwrap();
    let credix_lp_depository_collateral_amount_deposited_after =
        u64::try_from(credix_lp_depository_after.collateral_amount_deposited).unwrap();
    assert_eq!(
        credix_lp_depository_collateral_amount_deposited_before - expected_overflow_value,
        credix_lp_depository_collateral_amount_deposited_after,
    );

    // identity_depository.collateral_amount_deposited must have increased by the withdraw overflow
    let identity_depository_collateral_amount_deposited_before =
        u64::try_from(identity_depository_before.collateral_amount_deposited).unwrap();
    let identity_depository_collateral_amount_deposited_after =
        u64::try_from(identity_depository_after.collateral_amount_deposited).unwrap();
    assert_eq!(
        identity_depository_collateral_amount_deposited_before + expected_overflow_value,
        identity_depository_collateral_amount_deposited_after,
    );

    // credix_lp_depository.profits_amount_collected must have increased by the profits amount
    let credix_lp_depository_profits_total_collected_before: u64 =
        u64::try_from(credix_lp_depository_before.profits_total_collected).unwrap();
    let credix_lp_depository_profits_total_collected_after =
        u64::try_from(credix_lp_depository_after.profits_total_collected).unwrap();
    assert_eq!(
        credix_lp_depository_profits_total_collected_before + expected_profits_amount,
        credix_lp_depository_profits_total_collected_after,
    );

    // identity_depository_collateral.amount must have increased by the overflow amount
    assert_eq!(
        identity_depository_collateral_amount_before + expected_overflow_value,
        identity_depository_collateral_amount_after,
    );

    // profits_beneficiary_collateral.amount must have increased by the profits amount
    assert_eq!(
        profits_beneficiary_collateral_amount_before + expected_profits_amount,
        profits_beneficiary_collateral_amount_after,
    );

    // Done
    Ok(())
}
