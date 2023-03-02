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
use uxd::state::IdentityDepository;

use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_redeem_from_identity_depository(
    program_test_context: &mut ProgramTestContext,
    program_info: &program_uxd::accounts::ProgramInfo,
    payer: &Keypair,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    redeemable_amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Read state before
    let redeemable_mint_before =
        program_test_context::read_account_packed::<spl_token::state::Mint>(
            program_test_context,
            &program_info.redeemable_mint,
        )
        .await?;
    let controller_before = program_test_context::read_account_anchor::<Controller>(
        program_test_context,
        &program_info.controller,
    )
    .await?;
    let identity_depository_before =
        program_test_context::read_account_anchor::<IdentityDepository>(
            program_test_context,
            &program_info.identity_depository_info.depository,
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
        program_test_context::read_account_packed::<Account>(program_test_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_before =
        program_test_context::read_account_packed::<Account>(program_test_context, user_redeemable)
            .await?
            .amount;

    // Execute IX
    let accounts = uxd::accounts::RedeemFromIdentityDepository {
        user: user.pubkey(),
        payer: payer.pubkey(),
        controller: program_info.controller,
        depository: program_info.identity_depository_info.depository,
        collateral_vault: program_info.identity_depository_info.collateral_vault,
        redeemable_mint: program_info.redeemable_mint,
        user_collateral: *user_collateral,
        user_redeemable: *user_redeemable,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
    };
    let payload = uxd::instruction::RedeemFromIdentityDepository { redeemable_amount };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction,
        payer,
        user,
    )
    .await?;

    // Read state after
    let redeemable_mint_after = program_test_context::read_account_packed::<Mint>(
        program_test_context,
        &program_info.redeemable_mint,
    )
    .await?;
    let controller_after = program_test_context::read_account_anchor::<Controller>(
        program_test_context,
        &program_info.controller,
    )
    .await?;
    let identity_depository_after =
        program_test_context::read_account_anchor::<IdentityDepository>(
            program_test_context,
            &program_info.identity_depository_info.depository,
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
        program_test_context::read_account_packed::<Account>(program_test_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_after =
        program_test_context::read_account_packed::<Account>(program_test_context, user_redeemable)
            .await?
            .amount;

    // For the identity depository, we always get 1:1 collateral/redeemable
    let collateral_amount = redeemable_amount;

    // Check result
    assert_eq!(
        redeemable_mint_supply_before - redeemable_amount,
        redeemable_mint_supply_after,
    );
    assert_eq!(
        redeemable_circulating_supply_before - redeemable_amount,
        redeemable_circulating_supply_after,
    );
    assert_eq!(
        redeemable_amount_under_management_before - redeemable_amount,
        redeemable_amount_under_management_after,
    );
    assert_eq!(
        collateral_amount_deposited_before - collateral_amount,
        collateral_amount_deposited_after,
    );

    assert_eq!(
        user_collateral_amount_before + collateral_amount,
        user_collateral_amount_after,
    );
    assert_eq!(
        user_redeemable_amount_before - redeemable_amount,
        user_redeemable_amount_after,
    );

    // Done
    Ok(())
}
