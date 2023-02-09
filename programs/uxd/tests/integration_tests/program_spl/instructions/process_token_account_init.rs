use solana_program::program_pack::Pack;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;

pub async fn process_token_account_init(
    program_test_context: &mut ProgramTestContext,
    payer: &Keypair,
    account: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<(), String> {
    let rent =
        crate::integration_tests::program_test_context::get_rent(program_test_context).await?;

    let instruction_create = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &account.pubkey(),
        rent.minimum_balance(Account::LEN),
        Account::LEN as u64,
        &spl_token::id(),
    );
    crate::integration_tests::program_test_context::process_instruction_with_signer(
        program_test_context,
        instruction_create,
        account,
        payer,
    )
    .await?;

    let instruction_init = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &account.pubkey(),
        mint,
        owner,
    )
    .map_err(|e| e.to_string())?;

    crate::integration_tests::program_test_context::process_instruction(
        program_test_context,
        instruction_init,
        payer,
    )
    .await?;

    Ok(())
}
