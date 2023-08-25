use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

use async_trait::async_trait;

use crate::integration_tests::api::program_context;

#[async_trait]
pub trait ProgramContext {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, program_context::ProgramError>;

    async fn get_minimum_balance(
        &mut self,
        space: usize,
    ) -> Result<u64, program_context::ProgramError>;

    async fn get_clock_unix_timestamp(&mut self) -> Result<i64, program_context::ProgramError>;

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, program_context::ProgramError>;

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), program_context::ProgramError>;

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<(), program_context::ProgramError>;

    async fn move_clock_forward(
        &mut self,
        unix_timestamp_delta: u64,
        slot_delta: u64,
    ) -> Result<(), program_context::ProgramError>;
}
