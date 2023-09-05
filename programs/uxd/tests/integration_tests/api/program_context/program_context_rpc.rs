use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;
use solana_sdk::sysvar::Sysvar;

use async_trait::async_trait;

use crate::integration_tests::api::program_context;

#[async_trait]
impl program_context::ProgramContext for RpcClient {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, program_context::ProgramError> {
        RpcClient::get_latest_blockhash(self)
            .await
            .map_err(program_context::ProgramError::Client)
    }

    async fn get_rent_minimum_balance(
        &mut self,
        space: usize,
    ) -> Result<u64, program_context::ProgramError> {
        self.get_minimum_balance_for_rent_exemption(space)
            .await
            .map_err(program_context::ProgramError::Client)
    }

    async fn get_clock(&mut self) -> Result<Clock, program_context::ProgramError> {
        Clock::get().map_err(program_context::ProgramError::Program)
    }

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, program_context::ProgramError> {
        let response = self
            .get_account_with_commitment(address, CommitmentConfig::processed())
            .await
            .map_err(program_context::ProgramError::Client)?;
        Ok(response.value)
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), program_context::ProgramError> {
        let signature = self
            .send_transaction(&transaction)
            .await
            .map_err(program_context::ProgramError::Client)?;
        println!("process_transaction signature:{:?}", signature);
        loop {
            let confirmed = self
                .confirm_transaction(&signature)
                .await
                .map_err(program_context::ProgramError::Client)?;
            if confirmed {
                break;
            }
        }
        Ok(())
    }

    async fn process_airdrop(
        &mut self,
        _to: &Pubkey,
        _lamports: u64,
    ) -> Result<(), program_context::ProgramError> {
        Err(program_context::ProgramError::Custom(
            "Airdrop not supported",
        ))
    }

    async fn move_clock_forward(
        &mut self,
        _unix_timestamp_delta: u64,
        _slot_delta: u64,
    ) -> Result<(), program_context::ProgramError> {
        Err(program_context::ProgramError::Custom(
            "Clock forward not supported",
        ))
    }
}
