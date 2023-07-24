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
use uxd::utils::calculate_amount_less_fees;
use uxd::utils::compute_shares_amount_for_value_floor;
use uxd::utils::compute_value_for_shares_amount_floor;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_mint_with_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    collateral_amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint_pda().0;
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

    // Read state before
    let redeemable_mint_before =
        program_test_context::read_account_packed::<Mint>(program_test_context, &redeemable_mint)
            .await?;
    let controller_before =
        program_test_context::read_account_anchor::<Controller>(program_test_context, &controller)
            .await?;
    let credix_lp_depository_before =
        program_test_context::read_account_anchor::<CredixLpDepository>(
            program_test_context,
            &credix_lp_depository,
        )
        .await?;

    let user_collateral_amount_before =
        program_test_context::read_account_packed::<Account>(program_test_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_before =
        program_test_context::read_account_packed::<Account>(program_test_context, user_redeemable)
            .await?
            .amount;

    // Execute IX
    let accounts = uxd::accounts::MintWithCredixLpDepository {
        authority: authority.pubkey(),
        user: user.pubkey(),
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        redeemable_mint,
        user_collateral: *user_collateral,
        user_redeemable: *user_redeemable,
        depository: credix_lp_depository,
        depository_collateral: credix_lp_depository_collateral,
        depository_shares: credix_lp_depository_shares,
        credix_global_market_state,
        credix_signing_authority,
        credix_liquidity_collateral,
        credix_shares_mint,
        credix_pass,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        credix_program: credix_client::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    };
    let payload = uxd::instruction::MintWithCredixLpDepository { collateral_amount };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signers(
        program_test_context,
        instruction,
        payer,
        &vec![authority, user],
    )
    .await?;

    // Read state after
    let redeemable_mint_after =
        program_test_context::read_account_packed::<Mint>(program_test_context, &redeemable_mint)
            .await?;
    let controller_after =
        program_test_context::read_account_anchor::<Controller>(program_test_context, &controller)
            .await?;
    let credix_lp_depository_after =
        program_test_context::read_account_anchor::<CredixLpDepository>(
            program_test_context,
            &credix_lp_depository,
        )
        .await?;

    let user_collateral_amount_after =
        program_test_context::read_account_packed::<Account>(program_test_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_after =
        program_test_context::read_account_packed::<Account>(program_test_context, user_redeemable)
            .await?
            .amount;

    // Compute collateral amount deposited after precision loss
    let collateral_amount_after_precision_loss =
        process_mint_with_credix_lp_depository_collateral_amount_after_precision_loss(
            program_test_context,
            collateral_mint,
            collateral_amount,
        )
        .await?;

    // Compute expected redeemable amount after minting fees and precision loss
    let redeemable_amount = calculate_amount_less_fees(
        collateral_amount_after_precision_loss,
        credix_lp_depository_before.minting_fee_in_bps,
    )
    .map_err(program_test_context::ProgramTestError::Anchor)?;

    // Compute fees amount
    let fees_amount = collateral_amount_after_precision_loss - redeemable_amount;

    // redeemable_mint.supply must have increased by the minted amount (equivalent to redeemable_amount)
    let redeemable_mint_supply_before = redeemable_mint_before.supply;
    let redeemable_mint_supply_after = redeemable_mint_after.supply;
    assert_eq!(
        redeemable_mint_supply_before + redeemable_amount,
        redeemable_mint_supply_after,
    );

    // controller.redeemable_circulating_supply must have increased by the minted amount (equivalent to redeemable_amount)
    let redeemable_circulating_supply_before =
        u64::try_from(controller_before.redeemable_circulating_supply).unwrap();
    let redeemable_circulating_supply_after =
        u64::try_from(controller_after.redeemable_circulating_supply).unwrap();
    assert_eq!(
        redeemable_circulating_supply_before + redeemable_amount,
        redeemable_circulating_supply_after,
    );

    // depository.redeemable_amount_under_management must have increased by the minted amount (equivalent to redeemable_amount)
    let redeemable_amount_under_management_before =
        u64::try_from(credix_lp_depository_before.redeemable_amount_under_management).unwrap();
    let redeemable_amount_under_management_after =
        u64::try_from(credix_lp_depository_after.redeemable_amount_under_management).unwrap();
    assert_eq!(
        redeemable_amount_under_management_before + redeemable_amount,
        redeemable_amount_under_management_after,
    );

    // depository.minting_fee_total_accrued must have increased by the fees amount
    let minting_fee_total_accrued_before = credix_lp_depository_before.minting_fee_total_accrued;
    let minting_fee_total_accrued_after = credix_lp_depository_after.minting_fee_total_accrued;
    assert_eq!(
        minting_fee_total_accrued_before + u128::from(fees_amount),
        minting_fee_total_accrued_after,
    );

    // depository.collateral_amount_deposited must have increased by the deposited amount (equivalent to collateral_amount)
    let collateral_amount_deposited_before =
        u64::try_from(credix_lp_depository_before.collateral_amount_deposited).unwrap();
    let collateral_amount_deposited_after =
        u64::try_from(credix_lp_depository_after.collateral_amount_deposited).unwrap();
    assert_eq!(
        collateral_amount_deposited_before + collateral_amount,
        collateral_amount_deposited_after,
    );

    // user_collateral.amount must have decreased by the deposited amount (equivalent to collateral_amount)
    assert_eq!(
        user_collateral_amount_before - collateral_amount,
        user_collateral_amount_after,
    );
    // user_redeemable.amount must have increased by the minted amount (equivalent to redeemable_amount)
    assert_eq!(
        user_redeemable_amount_before + redeemable_amount,
        user_redeemable_amount_after,
    );

    // Done
    Ok(())
}

pub async fn process_mint_with_credix_lp_depository_collateral_amount_after_precision_loss(
    program_test_context: &mut ProgramTestContext,
    collateral_mint: &Pubkey,
    collateral_amount: u64,
) -> Result<u64, program_test_context::ProgramTestError> {
    // Read on chain accounts that contains the credix useful states
    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds).0;
    let credix_shares_mint =
        program_credix::accounts::find_lp_token_mint_pda(&credix_market_seeds).0;
    let credix_signing_authority =
        program_credix::accounts::find_signing_authority_pda(&credix_market_seeds).0;
    let credix_liquidity_collateral = program_credix::accounts::find_liquidity_pool_token_account(
        &credix_signing_authority,
        collateral_mint,
    );

    // Fetch information from onchain credix lp pool, to be able to predict precision loss
    let credix_shares_mint_supply = program_test_context::read_account_packed::<Mint>(
        program_test_context,
        &credix_shares_mint,
    )
    .await?
    .supply;
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
    let total_shares_supply = credix_shares_mint_supply;
    let total_shares_value = credix_liquidity_collateral_amount + credix_pool_outstanding_credit;

    // Compute expected redeemable amount after minting fees and precision loss
    let shares_amount = compute_shares_amount_for_value_floor(
        collateral_amount,
        total_shares_supply,
        total_shares_value,
    )
    .map_err(program_test_context::ProgramTestError::Anchor)?;
    let collateral_amount_after_precision_loss = compute_value_for_shares_amount_floor(
        shares_amount,
        total_shares_supply,
        total_shares_value,
    )
    .map_err(program_test_context::ProgramTestError::Anchor)?;

    // Done
    Ok(collateral_amount_after_precision_loss)
}
