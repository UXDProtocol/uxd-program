use solana_program::program_error::ProgramError;
use solana_program_test::BanksClientError;
use solana_sdk::transaction::TransactionError;

#[derive(Debug)]
pub enum ProgramTestError {
    BanksClient(BanksClientError),
    Transaction(TransactionError),
    Signature(String),
    Program(ProgramError),
    Anchor(anchor_lang::error::Error),
    Io(std::io::Error),
    Custom(&'static str),
}
