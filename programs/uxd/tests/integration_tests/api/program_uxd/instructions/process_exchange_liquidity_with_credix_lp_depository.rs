use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;

use uxd::state::CredixLpDepository;
use uxd::state::IdentityDepository;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_exchange_liquidity_with_credix_lp_depository(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    collateral_mint: &Pubkey,
    user: &Keypair,
    user_collateral: &Pubkey,
    receiver_credix_shares: &Pubkey,
    collateral_amount: u64,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;

    let identity_depository = program_uxd::accounts::find_identity_depository_pda().0;
    let identity_depository_collateral =
        program_uxd::accounts::find_identity_depository_collateral_vault_pda().0;

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
    let credix_lp_depository_shares = program_uxd::accounts::find_credix_lp_depository_shares(
        &credix_lp_depository,
        &credix_shares_mint,
    );

    // Read state before
    let identity_depository_before = program_context::read_account_anchor::<IdentityDepository>(
        program_context,
        &identity_depository,
    )
    .await?;
    let credix_lp_depository_before = program_context::read_account_anchor::<CredixLpDepository>(
        program_context,
        &credix_lp_depository,
    )
    .await?;

    let identity_depository_collateral_amount_before =
        program_context::read_account_packed::<Account>(
            program_context,
            &identity_depository_collateral,
        )
        .await?
        .amount;
    let credix_lp_depository_shares_amount_before =
        program_context::read_account_packed::<Account>(
            program_context,
            &credix_lp_depository_shares,
        )
        .await?
        .amount;
    let user_collateral_amount_before =
        program_context::read_account_packed::<Account>(program_context, user_collateral)
            .await?
            .amount;
    let receiver_credix_shares_amount_before =
        program_context::read_account_packed::<Account>(program_context, receiver_credix_shares)
            .await?
            .amount;

    // Execute IX
    let accounts = uxd::accounts::ExchangeLiquidityWithCredixLpDepository {
        payer: payer.pubkey(),
        controller,
        identity_depository,
        identity_depository_collateral,
        credix_lp_depository,
        credix_lp_depository_shares,
        collateral_mint: *collateral_mint,
        credix_shares_mint,
        user: user.pubkey(),
        user_collateral: *user_collateral,
        receiver_credix_shares: *receiver_credix_shares,
        system_program: solana_sdk::system_program::ID,
        token_program: anchor_spl::token::ID,
    };
    let payload = uxd::instruction::ExchangeLiquidityWithCredixLpDepository { collateral_amount };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction_with_signers(program_context, instruction, payer, &[user])
        .await?;

    // Read state after
    let identity_depository_after = program_context::read_account_anchor::<IdentityDepository>(
        program_context,
        &identity_depository,
    )
    .await?;
    let credix_lp_depository_after = program_context::read_account_anchor::<CredixLpDepository>(
        program_context,
        &credix_lp_depository,
    )
    .await?;

    let identity_depository_collateral_amount_after =
        program_context::read_account_packed::<Account>(
            program_context,
            &identity_depository_collateral,
        )
        .await?
        .amount;
    let credix_lp_depository_shares_amount_after = program_context::read_account_packed::<Account>(
        program_context,
        &credix_lp_depository_shares,
    )
    .await?
    .amount;
    let user_collateral_amount_after =
        program_context::read_account_packed::<Account>(program_context, user_collateral)
            .await?
            .amount;
    let receiver_credix_shares_amount_after =
        program_context::read_account_packed::<Account>(program_context, receiver_credix_shares)
            .await?
            .amount;

    // credix_lp_depository.redeemable_amount_under_management must have decreased by the exchanged amount
    let credix_lp_depository_redeemable_amount_under_management_before =
        u64::try_from(credix_lp_depository_before.redeemable_amount_under_management).unwrap();
    let credix_lp_depository_redeemable_amount_under_management_after =
        u64::try_from(credix_lp_depository_after.redeemable_amount_under_management).unwrap();
    assert_eq!(
        credix_lp_depository_redeemable_amount_under_management_before - collateral_amount,
        credix_lp_depository_redeemable_amount_under_management_after,
    );
    // credix_lp_depository.collateral_amount_deposited must have decreased by the exchanged amount
    let credix_lp_depository_collateral_amount_deposited_before =
        u64::try_from(credix_lp_depository_before.collateral_amount_deposited).unwrap();
    let credix_lp_depository_collateral_amount_deposited_after =
        u64::try_from(credix_lp_depository_after.collateral_amount_deposited).unwrap();
    assert_eq!(
        credix_lp_depository_collateral_amount_deposited_before - collateral_amount,
        credix_lp_depository_collateral_amount_deposited_after,
    );

    // identity_depository.redeemable_amount_under_management must have increased by the exchanged amount
    let identity_depository_redeemable_amount_under_management_before =
        u64::try_from(identity_depository_before.redeemable_amount_under_management).unwrap();
    let identity_depository_redeemable_amount_under_management_after =
        u64::try_from(identity_depository_after.redeemable_amount_under_management).unwrap();
    assert_eq!(
        identity_depository_redeemable_amount_under_management_before + collateral_amount,
        identity_depository_redeemable_amount_under_management_after,
    );
    // identity_depository.collateral_amount_deposited must have increased by the exchanged amount
    let identity_depository_collateral_amount_deposited_before =
        u64::try_from(identity_depository_before.collateral_amount_deposited).unwrap();
    let identity_depository_collateral_amount_deposited_after =
        u64::try_from(identity_depository_after.collateral_amount_deposited).unwrap();
    assert_eq!(
        identity_depository_collateral_amount_deposited_before + collateral_amount,
        identity_depository_collateral_amount_deposited_after,
    );

    // identity_depository_collateral.amount must have increased by the exchanged amount
    assert_eq!(
        identity_depository_collateral_amount_before + collateral_amount,
        identity_depository_collateral_amount_after,
    );
    // user_collateral.amount must have decreased by the exchanged amount
    assert_eq!(
        user_collateral_amount_before - collateral_amount,
        user_collateral_amount_after,
    );

    // Expected transacted shares amount
    let exchanged_shares_amount = u64::try_from(
        u128::from(credix_lp_depository_shares_amount_before) * u128::from(collateral_amount)
            / credix_lp_depository_before.redeemable_amount_under_management,
    )
    .unwrap();

    // credix_lp_depository_shares.amount must have decreased by the computed shares amount
    assert_eq!(
        credix_lp_depository_shares_amount_before - exchanged_shares_amount,
        credix_lp_depository_shares_amount_after,
    );
    // receiver_credix_shares.amount must have increased by the computed shares amount
    assert_eq!(
        receiver_credix_shares_amount_before + exchanged_shares_amount,
        receiver_credix_shares_amount_after,
    );

    // Done
    Ok(())
}
