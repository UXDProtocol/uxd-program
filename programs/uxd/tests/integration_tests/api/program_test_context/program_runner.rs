use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::transaction::Transaction;
use solana_sdk::sysvar::clock::Clock;
use solana_program_test::ProgramTestContext;
use solana_program_test::ProgramTestBanksClientExt;

use async_trait::async_trait;

use crate::integration_tests::api::program_test_context;

#[async_trait]
pub trait ProgramRunner {
    async fn get_latest_blockhash(
        &mut self,
    ) -> Result<Hash, program_test_context::ProgramTestError>;

    async fn get_minimum_balance(&mut self) -> Result<u64, program_test_context::ProgramTestError>;

    async fn get_clock_unix_timestamp(
        &mut self,
    ) -> Result<i64, program_test_context::ProgramTestError>;

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, program_test_context::ProgramTestError>;

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), program_test_context::ProgramTestError>;

    async fn process_aidrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<(), program_test_context::ProgramTestError>;
}

#[async_trait]
impl ProgramRunner for ProgramTestContext {
    async fn get_latest_blockhash(
        &mut self,
    ) -> Result<Hash, program_test_context::ProgramTestError> {
        Ok(self.last_blockhash)
    }

    async fn get_minimum_balance(
        &mut self,
        space: usize,
    ) -> Result<u64, program_test_context::ProgramTestError> {
        let rent = self
            .banks_client
            .get_rent()
            .await
            .map_err(program_test_context::ProgramTestError::BanksClient)?;
        Ok(rent.minimum_balance(space))
    }

    async fn get_clock_unix_timestamp(
        &mut self,
        space: usize,
    ) -> Result<i64, program_test_context::ProgramTestError> {
        let clock = program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .map_err(program_test_context::ProgramTestError::BanksClient)?;
        Ok(clock.unix_timestamp)
    }

    async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, program_test_context::ProgramTestError> {
        self.banks_client
            .get_account(*address)
            .await
            .map_err(program_test_context::ProgramTestError::BanksClient)
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), program_test_context::ProgramTestError> {
        self.last_blockhash = self
            .get_new_latest_blockhash()
            .await
            .map_err(program_test_context::ProgramTestError::Io)?;
        self.banks_client
            .process_transaction(transaction)
            .await
            .map_err(program_test_context::ProgramTestError::BanksClient)
    }

    async fn process_aidrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<(), program_test_context::ProgramTestError> {
        let from = Keypair::from_bytes(&self.payer.to_bytes())
            .map_err(|e| program_test_context::ProgramTestError::Signature(e.to_string()))?;
        let instruction =
            solana_program::system_instruction::transfer(&from.pubkey(), to, lamports);
        program_test_context::process_instruction(self, instruction, &from).await
    }
}
