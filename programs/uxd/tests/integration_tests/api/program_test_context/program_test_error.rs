use solana_program::program_error::ProgramError;
use solana_program_test::BanksClientError;

#[derive(Debug)]
pub enum ProgramTestError {
    BanksClient(BanksClientError),
    Signature(String),
    Program(ProgramError),
    Anchor(anchor_lang::error::Error),
    Io(std::io::Error),
    Custom(&'static str),
}
