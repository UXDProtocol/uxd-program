use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;
use spl_token::state::Mint;

use uxd::state::Controller;
use uxd::state::CredixLpDepository;
use uxd::utils::compute_shares_amount_for_value_floor;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_collect_profits_of_credix_lp_depository(
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
    let credix_treasury = program_credix::accounts::find_treasury(credix_multisig_key);
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
    let credix_shares_mint_before = program_test_context::read_account_packed::<Mint>(
        program_test_context,
        &credix_shares_mint,
    )
    .await?;

    let credix_lp_depository_shares_value_before = compute_credix_lp_depository_shares_value(
        program_test_context,
        &credix_global_market_state,
        &credix_shares_mint,
        &credix_liquidity_collateral,
    )
    .await?;
    let credix_lp_depository_shares_amount_before =
        program_test_context::read_account_packed::<Account>(
            program_test_context,
            &credix_lp_depository_shares,
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

    // Execute IX
    let accounts = uxd::accounts::CollectProfitsOfCredixLpDepository {
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
    let payload = uxd::instruction::CollectProfitsOfCredixLpDepository {};
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
    let credix_shares_mint_after = program_test_context::read_account_packed::<Mint>(
        program_test_context,
        &credix_shares_mint,
    )
    .await?;

    let credix_lp_depository_shares_value_after = compute_credix_lp_depository_shares_value(
        program_test_context,
        &credix_global_market_state,
        &credix_shares_mint,
        &credix_liquidity_collateral,
    )
    .await?;
    let credix_lp_depository_shares_amount_after =
        program_test_context::read_account_packed::<Account>(
            program_test_context,
            &credix_lp_depository_shares,
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

    // Compute the amount of expected profits to be withdrawn
    let liabilities_value_before =
        u64::try_from(credix_lp_depository_before.redeemable_amount_under_management).unwrap();
    let liabilities_value_after =
        u64::try_from(credix_lp_depository_after.redeemable_amount_under_management).unwrap();

    let profits_collateral_amount =
        credix_lp_depository_shares_value_before - liabilities_value_before;
    let profits_shares_amount = compute_shares_amount_for_value_floor(
        profits_collateral_amount,
        credix_shares_mint_before.supply,
        credix_lp_depository_shares_value_before,
    )
    .map_err(program_test_context::ProgramTestError::Anchor)?;

    // Check result
    let credix_shares_mint_supply_before = credix_shares_mint_before.supply;
    let credix_shares_mint_supply_after = credix_shares_mint_after.supply;
    assert_eq!(
        credix_shares_mint_supply_before - profits_shares_amount,
        credix_shares_mint_supply_after,
    );
    let controller_profits_total_collected_before = controller_before.profits_total_collected;
    let controller_profits_total_collected_after = controller_after.profits_total_collected;
    assert_eq!(
        controller_profits_total_collected_before + u128::from(profits_collateral_amount),
        controller_profits_total_collected_after,
    );
    let credix_lp_depository_profits_total_collected_before =
        credix_lp_depository_before.profits_total_collected;
    let credix_lp_depository_profits_total_collected_after =
        credix_lp_depository_after.profits_total_collected;
    assert_eq!(
        credix_lp_depository_profits_total_collected_before + u128::from(profits_collateral_amount),
        credix_lp_depository_profits_total_collected_after,
    );

    assert_eq!(
        credix_lp_depository_shares_amount_before - profits_shares_amount,
        credix_lp_depository_shares_amount_after,
    );
    assert_eq!(
        profits_beneficiary_collateral_amount_before + profits_collateral_amount,
        profits_beneficiary_collateral_amount_after,
    );

    assert_eq!(
        credix_lp_depository_shares_value_before - profits_collateral_amount,
        credix_lp_depository_shares_value_after,
    );

    assert_eq!(liabilities_value_before, liabilities_value_after);

    // Done
    Ok(())
}

async fn compute_credix_lp_depository_shares_value(
    program_test_context: &mut ProgramTestContext,
    credix_global_market_state: &Pubkey,
    credix_shares_mint: &Pubkey,
    credix_liquidity_collateral: &Pubkey,
) -> Result<u64, program_test_context::ProgramTestError> {
    let credix_pool_outstanding_credit = program_test_context::read_account_anchor::<
        credix_client::GlobalMarketState,
    >(program_test_context, &credix_global_market_state)
    .await?
    .pool_outstanding_credit;
    let credix_liquidity_collateral_amount = program_test_context::read_account_packed::<Account>(
        program_test_context,
        &credix_liquidity_collateral,
    )
    .await?
    .amount;
    Ok(credix_liquidity_collateral_amount + credix_pool_outstanding_credit)
}
