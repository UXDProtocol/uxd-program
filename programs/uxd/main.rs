use anchor_lang::prelude::Pubkey;
use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::instruction::Instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::Signature;
use solana_sdk::system_instruction;
use tokio; // 0.3.5
use solana_sdk::transaction::Transaction;
use std::error::Error;
use std::time::Duration;

pub async fn process_instruction(
    rpc_client: &RpcClient,
    instruction: Instruction,
    payer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let latest_blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .map_err(program_test_context::ProgramTestError::Client)?;

    let mut transaction: Transaction =
        Transaction::new_signed_with_payer(&[instruction.clone()], Some(&payer.pubkey()), &[payer], latest_blockhash);

    println!("transaction: {:?}", transaction);
    println!("transaction.is_signed(): {:?}", transaction.is_signed());
    println!(
        "transaction.verify_with_results(): {:?}",
        transaction.verify_with_results()
    );
    println!(
        "transaction.verify(): {:?}",
        transaction
            .verify()
            .map_err(program_test_context::ProgramTestError::Transaction)?
    );

    let result = rpc_client
        .send_transaction(&transaction)
        .await
        .map_err(program_test_context::ProgramTestError::Client)?;

    println!("result: {:?}", result);

    Ok(())
}

pub async fn process_instruction_with_signer(
    rpc_client: &RpcClient,
    instruction: Instruction,
    payer: &Keypair,
    signer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let latest_blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .map_err(program_test_context::ProgramTestError::Client)?;

    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction.clone()], Some(&payer.pubkey()));
    println!("transaction: {:?}", transaction);
    transaction
        .try_sign(&[signer, payer, signer], latest_blockhash)
        .map_err(program_test_context::ProgramTestError::Signer)?;

    println!("transaction: {:?}", transaction);
    println!("transaction.is_signed(): {:?}", transaction.is_signed());
    println!(
        "transaction.verify_with_results(): {:?}",
        transaction.verify_with_results()
    );
    println!(
        "transaction.verify(): {:?}",
        transaction
            .verify()
            .map_err(program_test_context::ProgramTestError::Transaction)?
    );

    let result = rpc_client
        .send_transaction(&transaction)
        .await
        .map_err(program_test_context::ProgramTestError::Client)?;

    println!("result: {:?}", result);
    assert!(false);

    Ok(())
}

pub async fn check_balance(
    rpc_client: &RpcClient,
    public_key: &Pubkey,
) -> Result<u64, program_test_context::ProgramTestError> {
    Ok(rpc_client
        .get_balance(&public_key)
        .await
        .map_err(program_test_context::ProgramTestError::Client)?)
}



pub async fn transfer_funds(
    rpc_client: &RpcClient,
    sender_keypair: &Keypair,
    receiver_pub_key: &Pubkey,
    lamports: u64,
) -> Result<(), program_test_context::ProgramTestError> {

    let ix = system_instruction::transfer(
        &sender_keypair.pubkey(),
        receiver_pub_key,
        lamports,
        
    );

     process_instruction(rpc_client, ix, sender_keypair).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), program_test_context::ProgramTestError> {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let payer = Keypair::from_base58_string(
        "5MaiiCavjCmn9Hs1o3eznqDEhRwxo8pXiAYez7keQUviUkbuRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG",
    );

    let receiver = Keypair::new();

    println!("payer: {:?}", payer.pubkey());
    println!("payer.is_on_curve: {:?}", payer.pubkey().is_on_curve());
    println!(
        "payer.balance: {:?}",
        check_balance(&rpc_client, &payer.pubkey()).await?
    );

    println!("receiver: {:?}", receiver.pubkey());
    println!(
        "receiver.is_on_curve: {:?}",
        receiver.pubkey().is_on_curve()
    );
    println!(
        "receiver.balance: {:?}",
        check_balance(&rpc_client, &receiver.pubkey()).await?
    );

    transfer_funds(&rpc_client, &payer, &receiver.pubkey(), 1_000_000).await?;

    // Done
    Ok(())
}
