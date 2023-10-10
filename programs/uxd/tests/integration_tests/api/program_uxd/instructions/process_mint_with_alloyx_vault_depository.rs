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
use uxd::utils::calculate_amount_less_fees;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_mint_with_alloyx_vault_depository(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Pubkey,
    alloyx_vault_mint: &Pubkey,
    user: &Keypair,
    user_collateral: &Pubkey,
    user_redeemable: &Pubkey,
    collateral_amount: u64,
) -> Result<(), program_context::ProgramError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint_pda().0;

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
    let alloyx_vault_depository_before = program_context::read_account_anchor::<
        AlloyxVaultDepository,
    >(program_context, &alloyx_vault_depository)
    .await?;

    let user_collateral_amount_before =
        program_context::read_account_packed::<Account>(program_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_before =
        program_context::read_account_packed::<Account>(program_context, user_redeemable)
            .await?
            .amount;

    // Execute IX
    let accounts = uxd::accounts::MintWithAlloyxVaultDepository {
        authority: authority.pubkey(),
        user: user.pubkey(),
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        redeemable_mint,
        user_collateral: *user_collateral,
        user_redeemable: *user_redeemable,
        depository: alloyx_vault_depository,
        depository_collateral: alloyx_vault_depository_collateral,
        depository_shares: alloyx_vault_depository_shares,
        alloyx_vault_info,
        alloyx_vault_collateral,
        alloyx_vault_shares,
        alloyx_vault_mint: *alloyx_vault_mint,
        alloyx_vault_pass,
        system_program: solana_sdk::system_program::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        token_program: anchor_spl::token::ID,
        alloyx_program: alloyx_cpi::ID,
        rent: solana_sdk::sysvar::rent::ID,
    };
    let payload = uxd::instruction::MintWithAlloyxVaultDepository { collateral_amount };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_context::process_instruction_with_signers(
        program_context,
        instruction,
        payer,
        &[authority, user],
    )
    .await?;

    // Read state after
    let redeemable_mint_after =
        program_context::read_account_packed::<Mint>(program_context, &redeemable_mint).await?;
    let controller_after =
        program_context::read_account_anchor::<Controller>(program_context, &controller).await?;
    let alloyx_vault_depository_after =
        program_context::read_account_anchor::<AlloyxVaultDepository>(
            program_context,
            &alloyx_vault_depository,
        )
        .await?;

    let user_collateral_amount_after =
        program_context::read_account_packed::<Account>(program_context, user_collateral)
            .await?
            .amount;
    let user_redeemable_amount_after =
        program_context::read_account_packed::<Account>(program_context, user_redeemable)
            .await?
            .amount;

    // Compute expected redeemable amount after minting fees
    let redeemable_amount = calculate_amount_less_fees(
        collateral_amount,
        alloyx_vault_depository_before.minting_fee_in_bps,
    )
    .map_err(program_context::ProgramError::Anchor)?;
    let fees_amount = collateral_amount - redeemable_amount;

    // redeemable_mint.supply must have increased by the minted amount (equivalent to redeemable_amount)
    let redeemable_mint_supply_before = redeemable_mint_before.supply;
    let redeemable_mint_supply_after = redeemable_mint_after.supply;
    assert_eq!(
        redeemable_mint_supply_before + redeemable_amount,
        redeemable_mint_supply_after,
    );

    // controller.redeemable_circulating_supply must have increased by the minted amount (equivalent to redeemable_amount)
    let redeemable_circulating_supply_before = controller_before.redeemable_circulating_supply;
    let redeemable_circulating_supply_after = controller_after.redeemable_circulating_supply;
    assert_eq!(
        redeemable_circulating_supply_before + u128::from(redeemable_amount),
        redeemable_circulating_supply_after,
    );

    // depository.redeemable_amount_under_management must have increased by the minted amount (equivalent to redeemable_amount)
    let redeemable_amount_under_management_before =
        alloyx_vault_depository_before.redeemable_amount_under_management;
    let redeemable_amount_under_management_after =
        alloyx_vault_depository_after.redeemable_amount_under_management;
    assert_eq!(
        redeemable_amount_under_management_before + redeemable_amount,
        redeemable_amount_under_management_after,
    );

    // depository.minting_fee_total_accrued must have increased by the fees amount
    let minting_fee_total_accrued_before = alloyx_vault_depository_before.minting_fee_total_accrued;
    let minting_fee_total_accrued_after = alloyx_vault_depository_after.minting_fee_total_accrued;
    assert_eq!(
        minting_fee_total_accrued_before + fees_amount,
        minting_fee_total_accrued_after,
    );

    // depository.collateral_amount_deposited must have increased by the deposited amount (equivalent to collateral_amount)
    let collateral_amount_deposited_before =
        alloyx_vault_depository_before.collateral_amount_deposited;
    let collateral_amount_deposited_after =
        alloyx_vault_depository_after.collateral_amount_deposited;
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

    // TODO - handle precision loss checks

    // Done
    Ok(())
}
