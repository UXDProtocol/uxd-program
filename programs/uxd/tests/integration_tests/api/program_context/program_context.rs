use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::clock::Clock;
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

#[async_trait]
impl ProgramContext for ProgramTestContext {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, program_context::ProgramError> {
        Ok(self.last_blockhash)
    }

    async fn get_minimum_balance(
        &mut self,
        space: usize,
    ) -> Result<u64, program_context::ProgramError> {
        let rent = self
            .banks_client
            .get_rent()
            .await
            .map_err(program_context::ProgramError::BanksClient)?;
        Ok(rent.minimum_balance(space))
    }

    async fn get_clock_unix_timestamp(&mut self) -> Result<i64, program_context::ProgramError> {
        let clock = self
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .map_err(program_context::ProgramError::BanksClient)?;
        Ok(clock.unix_timestamp)
    }

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, program_context::ProgramError> {
        self.banks_client
            .get_account(*address)
            .await
            .map_err(program_context::ProgramError::BanksClient)
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), program_context::ProgramError> {
        self.last_blockhash = self
            .get_new_latest_blockhash()
            .await
            .map_err(program_context::ProgramError::Io)?;
        self.banks_client
            .process_transaction(transaction)
            .await
            .map_err(program_context::ProgramError::BanksClient)
    }

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<(), program_context::ProgramError> {
        let from = Keypair::from_bytes(&self.payer.to_bytes())
            .map_err(|e| program_context::ProgramError::Signature(e.to_string()))?;
        let instruction = solana_sdk::system_instruction::transfer(&from.pubkey(), to, lamports);
        let latest_blockhash = self.get_latest_blockhash().await?;
        let mut transaction: Transaction =
            Transaction::new_with_payer(&[instruction.clone()], Some(&from.pubkey()));
        transaction.partial_sign(&[&from], latest_blockhash);
        self.process_transaction(transaction).await
    }

    async fn move_clock_forward(
        &mut self,
        unix_timestamp_delta: u64,
        slot_delta: u64,
    ) -> Result<(), program_context::ProgramError> {
        let current_clock = self
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .map_err(program_context::ProgramError::BanksClient)?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.epoch += 1;
        forwarded_clock.slot += slot_delta;
        forwarded_clock.unix_timestamp += i64::try_from(unix_timestamp_delta).unwrap();
        self.set_sysvar::<Clock>(&forwarded_clock);
        Ok(())
    }
}

#[async_trait]
impl ProgramContext for RpcClient {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, program_context::ProgramError> {
        RpcClient::get_latest_blockhash(self)
            .await
            .map_err(program_context::ProgramError::Client)
    }

    async fn get_minimum_balance(
        &mut self,
        space: usize,
    ) -> Result<u64, program_context::ProgramError> {
        self.get_minimum_balance_for_rent_exemption(space)
            .await
            .map_err(program_context::ProgramError::Client)
    }

    async fn get_clock_unix_timestamp(&mut self) -> Result<i64, program_context::ProgramError> {
        Ok(0) // TODO
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
