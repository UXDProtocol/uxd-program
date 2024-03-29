use solana_program_test::ProgramTestBanksClientExt;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;

use async_trait::async_trait;

use crate::integration_tests::api::program_context;

#[async_trait]
impl program_context::ProgramContext for ProgramTestContext {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, program_context::ProgramError> {
        Ok(self.last_blockhash)
    }

    async fn get_rent_minimum_balance(
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

    async fn get_clock(&mut self) -> Result<Clock, program_context::ProgramError> {
        let clock = self
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .map_err(program_context::ProgramError::BanksClient)?;
        Ok(clock)
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
            .banks_client
            .get_new_latest_blockhash(&self.last_blockhash)
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
