use solana_program_test::tokio;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_test_context;
use crate::integration_tests::api::program_uxd;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::system_instruction;

pub async fn check_balance(
    rpc_client: &mut RpcClient,
    public_key: &Pubkey,
) -> Result<u64, program_test_context::ProgramTestError> {
    Ok(rpc_client
        .get_balance(&public_key)
        .await
        .map_err(program_test_context::ProgramTestError::Client)?)
}

fn create_keypair(secret: [u8; 64]) -> Result<Keypair, program_test_context::ProgramTestError> {
    return Keypair::from_bytes(&secret)
        .map_err(|e| program_test_context::ProgramTestError::Signature(e.to_string()));
}

#[tokio::test]
async fn test_ensure_devnet() -> Result<(), program_test_context::ProgramTestError> {
    let mut rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let payer = create_keypair([
        132, 55, 4, 19, 225, 250, 7, 65, 89, 245, 162, 71, 109, 45, 216, 164, 16, 234, 143, 19,
        127, 37, 141, 115, 118, 187, 215, 154, 154, 168, 79, 76, 80, 166, 74, 214, 184, 69, 164,
        24, 1, 86, 144, 9, 157, 201, 9, 66, 252, 95, 21, 185, 205, 70, 167, 141, 127, 176, 35, 149,
        244, 172, 45, 119,
    ])?;

    let collateral_mint = [
        220, 61, 168, 61, 76, 248, 30, 169, 234, 135, 65, 81, 253, 127, 83, 70, 54, 122, 121, 230,
        58, 91, 213, 249, 142, 5, 144, 136, 74, 253, 196, 21, 227, 226, 242, 115, 178, 10, 175, 61,
        164, 129, 180, 11, 58, 110, 222, 58, 137, 147, 124, 239, 241, 87, 157, 27, 3, 18, 56, 185,
        124, 199, 37, 17,
    ];

    let receiver = Keypair::new();

    println!("payer: {:?}", payer.pubkey());
    println!("payer.is_on_curve: {:?}", payer.pubkey().is_on_curve());
    println!(
        "payer.balance: {:?}",
        check_balance(&mut rpc_client, &payer.pubkey()).await?
    );

    println!("receiver: {:?}", receiver.pubkey());
    println!(
        "receiver.is_on_curve: {:?}",
        receiver.pubkey().is_on_curve()
    );
    println!(
        "receiver.balance: {:?}",
        check_balance(&mut rpc_client, &receiver.pubkey()).await?
    );

    transfer_funds(&mut rpc_client, &payer, &receiver.pubkey(), 1_000_000).await?;

    assert!(false);
    // Done
    Ok(())
}
