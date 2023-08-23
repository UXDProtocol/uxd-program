use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;
use spl_token::state::Mint;

use uxd::state::Controller;
use uxd::state::MercurialVaultDepository;
use uxd::utils::calculate_amount_less_fees;

use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_mint_with_mercurial_vault_depository(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    mercurial_vault_lp_mint: &Pubkey,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    collateral_amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint_pda().0;
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
    let redeemable_mint_before =
        program_test_context::read_account_packed::<Mint>(program_runner, &redeemable_mint).await?;
    let controller_before =
        program_test_context::read_account_anchor::<Controller>(program_runner, &controller)
            .await?;
    let mercurial_vault_depository_before = program_test_context::read_account_anchor::<
        MercurialVaultDepository,
    >(program_runner, &mercurial_vault_depository)
    .await?;

    let user_collateral_amount_before =
        program_test_context::read_account_packed::<Account>(program_runner, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_before =
        program_test_context::read_account_packed::<Account>(program_runner, user_redeemable)
            .await?
            .amount;

    // Execute IX
    let accounts = uxd::accounts::MintWithMercurialVaultDepository {
        authority: authority.pubkey(),
        user: user.pubkey(),
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        redeemable_mint,
        user_collateral: *user_collateral,
        user_redeemable: *user_redeemable,
        depository: mercurial_vault_depository,
        depository_lp_token_vault: mercurial_vault_depository_lp_token_vault,
        mercurial_vault,
        mercurial_vault_lp_mint: *mercurial_vault_lp_mint,
        mercurial_vault_collateral_token_safe,
        system_program: anchor_lang::system_program::ID,
        token_program: anchor_spl::token::ID,
        mercurial_vault_program: mercurial_vault::ID,
    };
    let payload = uxd::instruction::MintWithMercurialVaultDepository { collateral_amount };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction_with_signers(
        program_runner,
        instruction,
        payer,
        &[authority, user],
    )
    .await?;

    // Read state after
    let redeemable_mint_after =
        program_test_context::read_account_packed::<Mint>(program_runner, &redeemable_mint).await?;
    let controller_after =
        program_test_context::read_account_anchor::<Controller>(program_runner, &controller)
            .await?;
    let mercurial_vault_depository_after = program_test_context::read_account_anchor::<
        MercurialVaultDepository,
    >(program_runner, &mercurial_vault_depository)
    .await?;

    let user_collateral_amount_after =
        program_test_context::read_account_packed::<Account>(program_runner, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_after =
        program_test_context::read_account_packed::<Account>(program_runner, user_redeemable)
            .await?
            .amount;

    // Compute expected redeemable amount after minting fees
    let redeemable_amount = calculate_amount_less_fees(
        collateral_amount,
        mercurial_vault_depository_before.minting_fee_in_bps,
    )
    .map_err(program_test_context::ProgramTestError::Anchor)?;
    let fees_amount = collateral_amount - redeemable_amount;

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
        u64::try_from(mercurial_vault_depository_before.redeemable_amount_under_management)
            .unwrap();
    let redeemable_amount_under_management_after =
        u64::try_from(mercurial_vault_depository_after.redeemable_amount_under_management).unwrap();
    assert_eq!(
        redeemable_amount_under_management_before + redeemable_amount,
        redeemable_amount_under_management_after,
    );

    // depository.minting_fee_total_accrued must have increased by the fees amount
    let minting_fee_total_accrued_before =
        mercurial_vault_depository_before.minting_fee_total_accrued;
    let minting_fee_total_accrued_after =
        mercurial_vault_depository_after.minting_fee_total_accrued;
    assert_eq!(
        minting_fee_total_accrued_before + u128::from(fees_amount),
        minting_fee_total_accrued_after,
    );

    // depository.collateral_amount_deposited must have increased by the deposited amount (equivalent to collateral_amount)
    let collateral_amount_deposited_before =
        u64::try_from(mercurial_vault_depository_before.collateral_amount_deposited).unwrap();
    let collateral_amount_deposited_after =
        u64::try_from(mercurial_vault_depository_after.collateral_amount_deposited).unwrap();
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
