use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;
use crate::integration_tests::utils::ui_amount_to_native_amount;

pub async fn process_dummy_actors_behaviors(
    program_context: &mut Box<dyn program_context::ProgramContext>,
    multisig: &Keypair,
    base_token_mint: &Pubkey,
    base_token_authority: &Keypair,
    base_token_decimals: u8,
) -> Result<(), program_context::ProgramError> {
    // The amounts we will be moving around
    let dummy_investor_deposit_amount = ui_amount_to_native_amount(100_000, base_token_decimals);
    let dummy_borrower_borrow_principal_amount = dummy_investor_deposit_amount;
    let dummy_borrower_borrow_interest_amount =
        ui_amount_to_native_amount(20_000, base_token_decimals);

    // Initialize the lp pool by having an investor deposit money
    program_credix::procedures::process_dummy_investor(
        program_context,
        multisig,
        base_token_mint,
        base_token_authority,
        dummy_investor_deposit_amount,
        0, // no withdraw request needed
    )
    .await?;

    // Increase the LP value of slightly by having a borrower borrow and pay interestes
    program_credix::procedures::process_dummy_borrower(
        program_context,
        multisig,
        base_token_mint,
        base_token_authority,
        dummy_borrower_borrow_principal_amount,
        dummy_borrower_borrow_interest_amount,
        dummy_borrower_borrow_principal_amount + dummy_borrower_borrow_interest_amount, // repay everything immediately with interests
    )
    .await?;

    // Done
    Ok(())
}
