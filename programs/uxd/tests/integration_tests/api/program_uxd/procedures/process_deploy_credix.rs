use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditCredixLpDepositoryFields;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_deploy_credix(
    program_test_context: &mut ProgramTestContext,
    market_seeds: &String,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Keypair,
    credix_multisig: &Keypair,
    collateral_mint_decimals: u8,
    redeemable_mint_decimals: u8,
    credix_lp_depository_redeemable_amount_under_management_cap: u128,
    credix_lp_depository_minting_fee_in_bps: u8,
    credix_lp_depository_redeeming_fee_in_bps: u8,
    credix_lp_depository_minting_disabled: bool,
    credix_lp_depository_profits_beneficiary_collateral: Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    // Credix onchain dependency program deployment
    program_credix::procedures::process_dummy_actors_behaviors(
        program_test_context,
        market_seeds,
        credix_multisig,
        &collateral_mint.pubkey(),
        collateral_mint,
        collateral_mint_decimals,
    )
    .await?;

    // Credix pass creation for our credix_lp depository (done by credix team on mainnet)
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state_pda(market_seeds).0;
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository_pda(
        &collateral_mint.pubkey(),
        &credix_global_market_state,
    )
    .0;
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
        market_seeds,
        credix_multisig,
        &credix_lp_depository,
        &credix_client::instruction::CreateCredixPass {
            _is_investor: true,
            _is_borrower: false,
            _release_timestamp: 0,
            _amount_cap: None,
            _disable_withdrawal_fee: true,
            _bypass_withdraw_epochs: false,
        },
    )
    .await?;

    // depository setup
    program_uxd::instructions::process_register_credix_lp_depository(
        program_test_context,
        market_seeds,
        payer,
        authority,
        &collateral_mint.pubkey(),
        0,
        0,
        0,
    )
    .await?;
    program_uxd::instructions::process_edit_credix_lp_depository(
        program_test_context,
        market_seeds,
        payer,
        authority,
        &collateral_mint.pubkey(),
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(
                credix_lp_depository_redeemable_amount_under_management_cap,
            ),
            minting_fee_in_bps: Some(credix_lp_depository_minting_fee_in_bps),
            redeeming_fee_in_bps: Some(credix_lp_depository_redeeming_fee_in_bps),
            minting_disabled: Some(credix_lp_depository_minting_disabled),
            profits_beneficiary_collateral: Some(
                credix_lp_depository_profits_beneficiary_collateral,
            ),
        },
    )
    .await?;

    // Done
    Ok(())
}
