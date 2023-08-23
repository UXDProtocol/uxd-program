use solana_client::client_error::ClientError;
use solana_program_test::BanksClientError;
use solana_sdk::program_error::ProgramError;
use solana_sdk::signer::SignerError;
use solana_sdk::transaction::TransactionError;

#[derive(Debug)]
pub enum ProgramTestError {
    BanksClient(BanksClientError),
    Client(ClientError),
    Signer(SignerError),
    Signature(String),
    Transaction(TransactionError),
    Program(ProgramError),
    Anchor(anchor_lang::error::Error),
    Io(std::io::Error),
    Custom(&'static str),
}
