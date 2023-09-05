#[derive(Debug)]
pub enum ProgramError {
    BanksClient(solana_program_test::BanksClientError),
    Client(solana_client::client_error::ClientError),
    Program(solana_sdk::program_error::ProgramError),
    Anchor(anchor_lang::error::Error),
    Compile(solana_sdk::message::CompileError),
    Signer(solana_sdk::signer::SignerError),
    Signature(String),
    Io(std::io::Error),
    Custom(&'static str),
}
