use solana_program_test::BanksClientError;
use solana_sdk::instruction::InstructionError;
use solana_sdk::message::CompileError;
use solana_sdk::program_error::ProgramError;
use solana_sdk::signer::SignerError;

#[derive(Debug)]
pub enum ProgramTestError {
    BanksClient(BanksClientError),
    Signature(String),
    Program(ProgramError),
    Compile(CompileError),
    Signer(SignerError),
    Anchor(anchor_lang::error::Error),
    Instruction(InstructionError),
    Io(std::io::Error),
    Custom(&'static str),
}
