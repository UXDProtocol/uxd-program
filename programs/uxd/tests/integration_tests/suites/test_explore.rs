use anchor_lang::prelude::Pubkey;
use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use std::error::Error;
use std::time::Duration;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::system_transaction;
use solana_program::instruction::Instruction;
use solana_sdk::transaction::Transaction;

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub async fn request_air_drop(
    rpc_client: &RpcClient,
    pub_key: &Pubkey,
    amount_sol: f64,
) -> Result<(), program_test_context::ProgramTestError> {
    let signature = rpc_client.request_airdrop(&pub_key, (amount_sol * (LAMPORTS_PER_SOL as f64)) as u64).await
    .map_err(program_test_context::ProgramTestError::Client)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&signature).await
        .map_err(program_test_context::ProgramTestError::Client)?;
        if confirmed {
            break;
        }
    }
    Ok(())
}

pub async fn process_instruction(
    rpc_client: &RpcClient,
    instruction: Instruction,
    payer: &Keypair,
) -> Result<(), program_test_context::ProgramTestError> {
    let mut transaction: Transaction =
        Transaction::new_with_payer(&[instruction.clone()], Some(&payer.pubkey()));

    let latest_blockhash = rpc_client.get_latest_blockhash().await.map_err(program_test_context::ProgramTestError::Client)?;
    transaction.partial_sign(&[payer], latest_blockhash);

    let result = rpc_client
        .send_and_confirm_transaction(&transaction)
        .await
        .map_err(program_test_context::ProgramTestError::Client)?;

    Ok(())
}

/*
pub async fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok((rpc_client.get_balance(&public_key)).await? as f64 / (LAMPORTS_PER_SOL as f64))
}
*/

/*
pub async fn transfer_funds(
    rpc_client: &RpcClient,
    sender_keypair: &Keypair,
    receiver_pub_key: &Pubkey,
    amount_sol: f64,
) -> core::result::Result<Signature, Box<dyn Error>> {
    let amount_lamports = (amount_sol * (LAMPORTS_PER_SOL as f64)) as u64;

    Ok(
        await rpc_client.send_and_confirm_transaction(&system_transaction::transfer(
            &sender_keypair,
            &receiver_pub_key,
            amount_lamports,
            rpc_client.get_latest_blockhash()?,
        ))?,
    )
}
*/

#[tokio::test]
async fn test_explore() -> Result<(), program_test_context::ProgramTestError> {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let sender = Keypair::from_base58_string(
        "5MaiiCavjCmn9Hs1o3eznqDEhRwxo8pXiAYez7keQUviUkbuRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG",
    );

    let receiver = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    request_air_drop(&rpc_client, &sender.pubkey(), 1.0).await?;

    /*
    check_balance(&rpc_client, &sender.pubkey()).await?;
check_balance(&rpc_client, &receiver.pubkey()).await?;
*/

//println!("Sender balance: {:?}", balance);

    // Done
    Ok(())
}
