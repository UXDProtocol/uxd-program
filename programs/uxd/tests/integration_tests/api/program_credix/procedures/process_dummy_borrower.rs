use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;

pub async fn process_dummy_borrower(
    program_test_context: &mut ProgramTestContext,
    market_seeds: &String,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
    base_token_authority: &Keypair,
    borrow_principal_amount: u64,
    borrow_interest_amount: u64,
    borrow_repay_amount: u64,
) -> Result<(), program_test_context::ProgramTestError> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Prepare our dummy borrower
    // -- Creating all their accounts
    // -- And airdroping the stuff they will need to act
    // ---------------------------------------------------------------------

    let dummy_borrower = Keypair::new();

    // Airdrop lamports to the dummy borrower wallet
    program_spl::instructions::process_lamports_airdrop(
        program_test_context,
        &dummy_borrower.pubkey(),
        1_000_000_000_000,
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

    // Give some collateral (base token) to our dummy borrower
    program_spl::instructions::process_token_mint_to(
        program_test_context,
        &dummy_borrower,
        base_token_mint,
        base_token_authority,
        &dummy_borrower_token_account,
        borrow_interest_amount,
    )
    .await?;

    // Create the credix-pass for the dummy investor
    program_credix::instructions::process_create_credix_pass(
        program_test_context,
        market_seeds,
        multisig,
        &dummy_borrower.pubkey(),
        &credix_client::instruction::CreateCredixPass {
            _is_investor: false,
            _is_borrower: true,
            _release_timestamp: 0,
            _amount_cap: None,
            _disable_withdrawal_fee: false,
            _bypass_withdraw_epochs: false,
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Create a simplified deal for our borrower to borrow from
    // -- The deal is structured with a single repayment and some fees
    // ---------------------------------------------------------------------

    program_credix::instructions::process_create_deal(
        program_test_context,
        market_seeds,
        multisig,
        &dummy_borrower.pubkey(),
        0,
    )
    .await?;
    program_credix::instructions::process_set_repayment_schedule(
        program_test_context,
        market_seeds,
        multisig,
        &dummy_borrower.pubkey(),
        0,
        borrow_principal_amount,
    )
    .await?;
    program_credix::instructions::process_set_tranches(
        program_test_context,
        market_seeds,
        multisig,
        &dummy_borrower.pubkey(),
        0,
        borrow_principal_amount,
        borrow_interest_amount,
    )
    .await?;
    program_credix::instructions::process_open_deal(
        program_test_context,
        market_seeds,
        multisig,
        &dummy_borrower.pubkey(),
        0,
    )
    .await?;
    program_credix::instructions::process_activate_deal(
        program_test_context,
        market_seeds,
        multisig,
        &dummy_borrower.pubkey(),
        0,
        base_token_mint,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Have our dummy borrower borrow the whole principal
    // -- Then our dummy borrower will repay the desired amount
    // ---------------------------------------------------------------------

    program_credix::instructions::process_withdraw_from_deal(
        program_test_context,
        market_seeds,
        &dummy_borrower,
        &dummy_borrower_token_account,
        0,
        base_token_mint,
        borrow_principal_amount,
    )
    .await?;

    program_credix::instructions::process_repay_deal(
        program_test_context,
        market_seeds,
        &dummy_borrower,
        &dummy_borrower_token_account,
        &multisig.pubkey(),
        0,
        base_token_mint,
        borrow_repay_amount,
    )
    .await?;

    // Done
    Ok(())
}
