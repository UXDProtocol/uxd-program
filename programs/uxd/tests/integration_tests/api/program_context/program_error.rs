#[derive(Debug, Display)]
pub enum ProgramError {
    BanksClient(solana_program_test::BanksClientError),
    Client(solana_client::client_error::ClientError),
    Signature(String),
    Program(solana_sdk::program_error::ProgramError),
    Anchor(anchor_lang::error::Error),
    Io(std::io::Error),
    Custom(&'static str),
}
