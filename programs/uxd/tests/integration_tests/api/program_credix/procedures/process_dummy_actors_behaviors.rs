use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_dummy_actors_behaviors(
    program_test_context: &mut ProgramTestContext,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
    base_token_authority: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let market_seeds = program_credix::accounts::find_market_seeds();
    let lp_token_mint = program_credix::accounts::find_lp_token_mint_pda(&market_seeds).0;
    let dummy_investor = Keypair::new();
    let dummy_borrower = Keypair::new();

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Prepare our important actors by:
    // -- Creating their token accounts
    // -- And airdroping the stuff they will need to act
    // ---------------------------------------------------------------------

    // Airdrop lamports to the dummy investor wallet
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &dummy_investor.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Airdrop lamports to the dummy borrower wallet
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &dummy_borrower.pubkey(),
        1_000_000_000_000,
    )
    .await?;

    // Create the investor ATAs
    let dummy_investor_token_account =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_test_context,
            &dummy_investor,
            base_token_mint,
            &dummy_investor.pubkey(),
        )
        .await?;
    let dummy_investor_lp_token_account =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_test_context,
            &dummy_investor,
            &lp_token_mint,
            &dummy_investor.pubkey(),
        )
        .await?;

    // Create the borrower ATAs
    let dummy_borrower_token_account =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_test_context,
            &dummy_borrower,
            base_token_mint,
            &dummy_borrower.pubkey(),
        )
        .await?;
    let dummy_borrower_lp_token_account =
        program_spl::instructions::process_associated_token_account_get_or_init(
            program_test_context,
            &dummy_borrower,
            &lp_token_mint,
            &dummy_borrower.pubkey(),
        )
        .await?;

    // Give some collateral (base token) to our dummy investor and create its token account
    program_spl::instructions::process_token_mint_to(
        program_test_context,
        &dummy_investor,
        base_token_mint,
        base_token_authority,
        &dummy_investor_token_account,
        1_000_000_000,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Have our dummy investor invest in the credix lp pool
    // -- This will initialize the pool with another competing investor
    // ---------------------------------------------------------------------

    // Create the credix-pass for the dummy investor
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
        multisig,
        &dummy_investor.pubkey(),
        &credix_client::instruction::CreateCredixPass {
            _is_investor: true,
            _is_borrower: false,
            _release_timestamp: 0,
            _disable_withdrawal_fee: false,
            _bypass_withdraw_epochs: false,
        },
    )
    .await?;

    // The dummy investor will do a dummy deposit to initialize the lp-pool
    program_credix::instructions::process_deposit_funds(
        program_test_context,
        base_token_mint,
        &dummy_investor,
        &dummy_investor_token_account,
        &dummy_investor_lp_token_account,
        1_000_000,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Have our dummy borrower borrow some money and pay some interest
    // -- This will create some profits that will increase LP value
    // ---------------------------------------------------------------------

    // Create the credix-pass for the dummy investor
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
        multisig,
        &dummy_borrower.pubkey(),
        &credix_client::instruction::CreateCredixPass {
            _is_investor: false,
            _is_borrower: true,
            _release_timestamp: 0,
            _disable_withdrawal_fee: false,
            _bypass_withdraw_epochs: false,
        },
    )
    .await?;

    // Create a deal with the borrower
    program_credix::instructions::process_create_deal(
        program_test_context,
        multisig,
        &dummy_borrower.pubkey(),
        0,
    )
    .await?;

    // Set deal payment details
    program_credix::instructions::process_set_repayment_schedule(
        program_test_context,
        multisig,
        &dummy_borrower.pubkey(),
        0,
    )
    .await?;

    program_credix::instructions::process_set_tranches(
        program_test_context,
        multisig,
        &dummy_borrower.pubkey(),
        0,
    )
    .await?;

    // Open deal?
    program_credix::instructions::process_open_deal(
        program_test_context,
        multisig,
        &dummy_borrower.pubkey(),
        0,
    )
    .await?;

    // Done
    Ok(())
}
