use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;

pub async fn process_dummy_investor(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
    base_token_authority: &Keypair,
    investor_deposit_amount: u64,
    investor_withdraw_request_amount: u64,
) -> Result<(), program_context::ProgramError> {
    let market_seeds = program_credix::accounts::find_market_seeds();
    let lp_token_mint = program_credix::accounts::find_lp_token_mint_pda(&market_seeds).0;

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Prepare our dummy investor
    // -- Creating all their accounts
    // -- And airdroping the stuff they will need to act
    // ---------------------------------------------------------------------

    let dummy_investor = Keypair::new();

    // Airdrop lamports to the dummy investor wallet
    program_context
        .process_airdrop(&dummy_investor.pubkey(), 1_000_000_000_000)
        .await?;

    // Create the investor ATAs
    let dummy_investor_token_account =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_context,
            &dummy_investor,
            base_token_mint,
            &dummy_investor.pubkey(),
        )
        .await?;
    let dummy_investor_lp_token_account =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_context,
            &dummy_investor,
            &lp_token_mint,
            &dummy_investor.pubkey(),
        )
        .await?;

    // Give collateral (base token) to our dummy investor
    program_spl::instructions::process_token_mint_to(
        program_context,
        &dummy_investor,
        base_token_mint,
        base_token_authority,
        &dummy_investor_token_account,
        investor_deposit_amount,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Have our dummy investor invest in the credix lp pool
    // -- Optionally if specified, have our investor create a withdraw request
    // ---------------------------------------------------------------------

    // Create the credix-pass for the dummy investor
    program_credix::instructions::process_create_credix_pass(
        program_context,
        multisig,
        &dummy_investor.pubkey(),
        &credix_client::instruction::CreateCredixPass {
            _is_investor: true,
            _is_borrower: false,
            _release_timestamp: 0,
            _amount_cap: None,
            _disable_withdrawal_fee: false,
            _bypass_withdraw_epochs: false,
        },
    )
    .await?;

    // The dummy investor will do a dummy deposit
    program_credix::instructions::process_deposit_funds(
        program_context,
        base_token_mint,
        &dummy_investor,
        &dummy_investor_token_account,
        &dummy_investor_lp_token_account,
        investor_deposit_amount,
    )
    .await?;

    // The dummy investor will optionally create a dummy withdraw request
    if investor_withdraw_request_amount > 0 {
        program_credix::instructions::process_create_withdraw_request(
            program_context,
            base_token_mint,
            &dummy_investor,
            &dummy_investor_lp_token_account,
            investor_withdraw_request_amount,
        )
        .await?;
    }

    // Done
    Ok(())
}
