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

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_mint_with_credix_lp_depository(
    program_test_context: &mut ProgramTestContext,
    program_keys: &program_uxd::accounts::ProgramKeys,
    payer: &Keypair,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    collateral_amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Read state before
    let redeemable_mint_before = program_test_context::read_account_packed::<Mint>(
        program_test_context,
        &program_keys.redeemable_mint,
    )
    .await?;
    let controller_before = program_test_context::read_account_anchor::<Controller>(
        program_test_context,
        &program_keys.controller,
    )
    .await?;
    let credix_lp_depository_before =
        program_test_context::read_account_anchor::<CredixLpDepository>(
            program_test_context,
            &program_keys.credix_lp_depository_keys.depository,
        )
        .await?;

    let redeemable_mint_supply_before = redeemable_mint_before.supply;
    let redeemable_circulating_supply_before =
        u64::try_from(controller_before.redeemable_circulating_supply).unwrap();
    let redeemable_amount_under_management_before =
        u64::try_from(credix_lp_depository_before.redeemable_amount_under_management).unwrap();
    let collateral_amount_deposited_before =
        u64::try_from(credix_lp_depository_before.collateral_amount_deposited).unwrap();

    let user_collateral_amount_before =
        program_test_context::read_account_packed::<Account>(program_test_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_before =
        program_test_context::read_account_packed::<Account>(program_test_context, user_redeemable)
            .await?
            .amount;

    // Execute IX
    let credix_lp_depository_keys = &program_keys.credix_lp_depository_keys;
    let accounts = uxd::accounts::MintWithCredixLpDepository {
        payer: payer.pubkey(),
        user: user.pubkey(),
        controller: program_keys.controller,
        collateral_mint: program_keys.collateral_mint.pubkey(),
        redeemable_mint: program_keys.redeemable_mint,
        user_collateral: *user_collateral,
        user_redeemable: *user_redeemable,
        depository: credix_lp_depository_keys.depository,
        depository_collateral: credix_lp_depository_keys.depository_collateral,
        depository_shares: credix_lp_depository_keys.depository_shares,
        credix_global_market_state: credix_lp_depository_keys.credix_global_market_state,
        credix_signing_authority: credix_lp_depository_keys.credix_signing_authority,
        credix_liquidity_collateral: credix_lp_depository_keys.credix_liquidity_collateral,
        credix_shares_mint: credix_lp_depository_keys.credix_shares_mint,
        credix_pass: credix_lp_depository_keys.credix_pass,
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
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        &user,
    )
    .await?;

    // Read state after
    let redeemable_mint_after = program_test_context::read_account_packed::<Mint>(
        program_test_context,
        &program_keys.redeemable_mint,
    )
    .await?;
    let controller_after = program_test_context::read_account_anchor::<Controller>(
        program_test_context,
        &program_keys.controller,
    )
    .await?;
    let credix_lp_depository_after =
        program_test_context::read_account_anchor::<CredixLpDepository>(
            program_test_context,
            &program_keys.credix_lp_depository_keys.depository,
        )
        .await?;

    let redeemable_mint_supply_after = redeemable_mint_after.supply;
    let redeemable_circulating_supply_after =
        u64::try_from(controller_after.redeemable_circulating_supply).unwrap();
    let redeemable_amount_under_management_after =
        u64::try_from(credix_lp_depository_after.redeemable_amount_under_management).unwrap();
    let collateral_amount_deposited_after =
        u64::try_from(credix_lp_depository_after.collateral_amount_deposited).unwrap();

    let user_collateral_amount_after =
        program_test_context::read_account_packed::<Account>(program_test_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_after =
        program_test_context::read_account_packed::<Account>(program_test_context, user_redeemable)
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
