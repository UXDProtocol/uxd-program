use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;

use uxd::state::Controller;
use uxd::state::CredixLpDepository;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_rebalance_request_from_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    collateral_mint: &Pubkey,
    credix_multisig_key: &Pubkey,
    profits_beneficiary_collateral: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
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
    let credix_treasury = program_credix::accounts::find_treasury();
    let credix_treasury_collateral = program_credix::accounts::find_treasury_pool_token_account(
        &credix_treasury,
        collateral_mint,
    );
    let credix_pass = program_credix::accounts::find_credix_pass_pda(
        &credix_global_market_state,
        &credix_lp_depository,
    )
    .0;
    let credix_multisig_collateral = spl_associated_token_account::get_associated_token_address(
        credix_multisig_key,
        collateral_mint,
    );
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
    let credix_latest_withdraw_epoch_idx = program_test_context::read_account_anchor::<
        credix_client::GlobalMarketState,
    >(program_test_context, &credix_global_market_state)
    .await?
    .latest_withdraw_epoch_idx;
    let credix_withdraw_epoch = program_credix::accounts::find_withdraw_epoch_pda(
        &credix_global_market_state,
        credix_latest_withdraw_epoch_idx,
    )
    .0;
    let credix_withdraw_request = program_credix::accounts::find_withdraw_request_pda(
        &credix_global_market_state,
        credix_latest_withdraw_epoch_idx,
        &credix_lp_depository,
    )
    .0;

    // Read state before
    let controller_before =
        program_test_context::read_account_anchor::<Controller>(program_test_context, &controller)
            .await?;
    let credix_lp_depository_before =
        program_test_context::read_account_anchor::<CredixLpDepository>(
            program_test_context,
            &credix_lp_depository,
        )
        .await?;

    let profits_beneficiary_collateral_amount_before =
        program_test_context::read_account_packed::<Account>(
            program_test_context,
            profits_beneficiary_collateral,
        )
        .await?
        .amount;

    // Execute IX
    let accounts = uxd::accounts::RebalanceRequestFromCredixLpDepository {
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        depository: credix_lp_depository,
        depository_collateral: credix_lp_depository_collateral,
        depository_shares: credix_lp_depository_shares,
        credix_program_state,
        credix_global_market_state,
        credix_signing_authority,
        credix_liquidity_collateral,
        credix_shares_mint,
        credix_pass,
        credix_withdraw_epoch,
        credix_withdraw_request,
        credix_treasury_collateral,
        credix_multisig_key: *credix_multisig_key,
        credix_multisig_collateral,
        profits_beneficiary_collateral: *profits_beneficiary_collateral,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        credix_program: credix_client::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = uxd::instruction::RebalanceRequestFromCredixLpDepository {};
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
    let credix_lp_depository_after =
        program_test_context::read_account_anchor::<CredixLpDepository>(
            program_test_context,
            &credix_lp_depository,
        )
        .await?;

    let profits_beneficiary_collateral_amount_after =
        program_test_context::read_account_packed::<Account>(
            program_test_context,
            profits_beneficiary_collateral,
        )
        .await?
        .amount;

    // TODO - check after state

    // Done
    Ok(())
}
