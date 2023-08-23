use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

pub async fn process_rebalance_create_withdraw_request_from_credix_lp_depository(
    program_runner: &mut dyn program_test_context::ProgramRunner,
    payer: &Keypair,
    collateral_mint: &Pubkey,
) -> Result<(), program_test_context::ProgramTestError> {
    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;

    let identity_depository = program_uxd::accounts::find_identity_depository_pda().0;
    let mercurial_base = program_mercurial::accounts::find_base();
    let mercurial_vault_depository_vault =
        program_mercurial::accounts::find_vault_pda(collateral_mint, &mercurial_base.pubkey()).0;
    let mercurial_vault_depository = program_uxd::accounts::find_mercurial_vault_depository_pda(
        collateral_mint,
        &mercurial_vault_depository_vault,
    )
    .0;

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
    let credix_lp_depository_shares = program_uxd::accounts::find_credix_lp_depository_shares(
        &credix_lp_depository,
        &credix_shares_mint,
    );

    // Find the credix withdraw accounts
    let credix_latest_withdraw_epoch_idx = program_test_context::read_account_anchor::<
        credix_client::GlobalMarketState,
    >(program_runner, &credix_global_market_state)
    .await?
    .latest_withdraw_epoch_idx;
    let credix_withdraw_epoch = program_credix::accounts::find_withdraw_epoch_pda(
        &credix_global_market_state,
        credix_latest_withdraw_epoch_idx,
    )
    .0;
    let credix_withdraw_request = program_credix::accounts::find_withdraw_request_pda(
        &credix_global_market_state,
        &credix_lp_depository,
        credix_latest_withdraw_epoch_idx,
    )
    .0;

    // Execute IX
    let accounts = uxd::accounts::RebalanceCreateWithdrawRequestFromCredixLpDepository {
        payer: payer.pubkey(),
        controller,
        collateral_mint: *collateral_mint,
        identity_depository,
        mercurial_vault_depository,
        depository: credix_lp_depository,
        depository_shares: credix_lp_depository_shares,
        credix_global_market_state,
        credix_signing_authority,
        credix_liquidity_collateral,
        credix_shares_mint,
        credix_pass,
        credix_withdraw_epoch,
        credix_withdraw_request,
        system_program: anchor_lang::system_program::ID,
        credix_program: credix_client::ID,
    };
    let payload = uxd::instruction::RebalanceCreateWithdrawRequestFromCredixLpDepository {};
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    program_test_context::process_instruction(program_runner, instruction, payer).await?;

    // Done
    Ok(())
}
