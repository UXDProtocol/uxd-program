use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::account::Account;

pub fn program_test_add_account_with_lamports(
    program_test: &mut ProgramTest,
    key: Pubkey,
    lamports: u64,
) {
    program_test.add_account(
        key,
        Account {
            lamports,
            ..Account::default()
        },
    );
}
