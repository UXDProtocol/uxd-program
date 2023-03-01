use solana_program::program_error::ProgramError;
use solana_program_test::BanksClientError;

#[derive(Debug)]
pub enum ProgramTestError {
    BanksClientError(BanksClientError),
    SignatureError(String),
    ProgramError(ProgramError),
    AnchorError(anchor_lang::error::Error),
    CustomError(&'static str),
}
