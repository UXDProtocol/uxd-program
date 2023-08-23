use solana_program_test::tokio;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;

use solana_client::nonblocking::rpc_client::RpcClient;

fn create_keypair(secret: [u8; 64]) -> Result<Keypair, program_context::ProgramError> {
    return Keypair::from_bytes(&secret)
        .map_err(|e| program_context::ProgramError::Signature(e.to_string()));
}

#[tokio::test]
async fn test_ensure_devnet() -> Result<(), program_context::ProgramError> {
    let mut program_context: Box<dyn program_context::ProgramContext> =
        Box::new(RpcClient::new("https://api.devnet.solana.com".to_string()));

    let collateral_mint = create_keypair([
        220, 61, 168, 61, 76, 248, 30, 169, 234, 135, 65, 81, 253, 127, 83, 70, 54, 122, 121, 230,
        58, 91, 213, 249, 142, 5, 144, 136, 74, 253, 196, 21, 227, 226, 242, 115, 178, 10, 175, 61,
        164, 129, 180, 11, 58, 110, 222, 58, 137, 147, 124, 239, 241, 87, 157, 27, 3, 18, 56, 185,
        124, 199, 37, 17,
    ])?;

    let payer = create_keypair([
        132, 55, 4, 19, 225, 250, 7, 65, 89, 245, 162, 71, 109, 45, 216, 164, 16, 234, 143, 19,
        127, 37, 141, 115, 118, 187, 215, 154, 154, 168, 79, 76, 80, 166, 74, 214, 184, 69, 164,
        24, 1, 86, 144, 9, 157, 201, 9, 66, 252, 95, 21, 185, 205, 70, 167, 141, 127, 176, 35, 149,
        244, 172, 45, 119,
    ])?;

    let authority = &payer;

    let controller = program_uxd::accounts::find_controller_pda().0;
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint_pda().0;
    let identity_depository = program_uxd::accounts::find_identity_depository_pda().0;

    if !program_context::read_account_exists(&mut program_context, &collateral_mint.pubkey())
        .await?
    {
        program_spl::instructions::process_token_mint_init(
            &mut program_context,
            &payer,
            &collateral_mint,
            6,
            &authority.pubkey(),
        )
        .await?;
    }

    if !program_context::read_account_exists(&mut program_context, &controller.pubkey()).await? {
        program_uxd::instructions::process_initialize_controller(
            &mut program_context,
            &payer,
            &authority,
            6,
        )
        .await?;
    }

    if !program_context::read_account_exists(&mut program_context, &identity_depository.pubkey())
        .await?
    {
        program_uxd::instructions::process_initialize_identity_depository(
            &mut program_context,
            &payer,
            &authority,
            &collateral_mint.pubkey(),
        )
        .await?;
    }
    // Done
    Ok(())
}
