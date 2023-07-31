use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditCredixLpDepositoryFields;
use uxd::instructions::EditIdentityDepositoryFields;
use uxd::instructions::EditMercurialVaultDepositoryFields;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

#[allow(clippy::too_many_arguments)]
pub async fn process_deploy_credix(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    authority: &Keypair,
    collateral_mint: &Keypair,
    mercurial_vault_lp_mint: &Keypair,
    credix_multisig: &Keypair,
    collateral_mint_decimals: u8,
    redeemable_mint_decimals: u8,
) -> Result<(), program_test_context::ProgramTestError> {
    // Use restictive default values for all tests
    // Can be modified in individual test cases through edits
    // This forces all tests be explicit about their requirements
    let redeemable_global_supply_cap = 0;
    let identity_depository_redeemable_amount_under_management_cap = 0;
    let identity_depository_minting_disabled = true;
    let mercurial_vault_depository_redeemable_amount_under_management_cap = 0;
    let mercurial_vault_depository_minting_fee_in_bps = 255;
    let mercurial_vault_depository_redeeming_fee_in_bps = 255;
    let mercurial_vault_depository_minting_disabled = true;
    let mercurial_vault_depository_profits_beneficiary_collateral = Pubkey::default();
    let credix_lp_depository_marketplace_redeemable_amount_under_management_cap = 0;
    let credix_lp_depository_marketplace_minting_fee_in_bps = 255;
    let credix_lp_depository_marketplace_redeeming_fee_in_bps = 255;
    let credix_lp_depository_marketplace_minting_disabled = true;
    let credix_lp_depository_marketplace_profits_beneficiary_collateral = Pubkey::default();
    let credix_lp_depository_receivables_redeemable_amount_under_management_cap = 0;
    let credix_lp_depository_receivables_minting_fee_in_bps = 255;
    let credix_lp_depository_receivables_redeeming_fee_in_bps = 255;
    let credix_lp_depository_receivables_minting_disabled = true;
    let credix_lp_depository_receivables_profits_beneficiary_collateral = Pubkey::default();

    // Credix onchain dependency program deployment
    program_credix::procedures::process_deploy_program(
        program_test_context,
        credix_multisig,
        &collateral_mint.pubkey(),
    )
    .await?;
    program_credix::procedures::process_dummy_actors_behaviors(
        program_test_context,
        credix_multisig,
        &collateral_mint.pubkey(),
        collateral_mint,
        collateral_mint_decimals,
    )
    .await?;

    // Credix pass creation for our credix_lp depository (done by credix team on mainnet)
    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds).0;
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository_pda(
        &collateral_mint.pubkey(),
        &credix_global_market_state,
    )
    .0;
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
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

    // Credix lp depository setup
    program_uxd::instructions::process_register_credix_lp_depository(
        program_test_context,
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
