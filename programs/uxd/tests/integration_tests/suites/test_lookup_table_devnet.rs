use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_program_test::tokio;
use solana_sdk::address_lookup_table_account::AddressLookupTableAccount;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::v0;
use solana_sdk::message::VersionedMessage;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;
use std::str::FromStr;

use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;

use solana_client::nonblocking::rpc_client::RpcClient;

fn create_keypair(secret: [u8; 64]) -> Result<Keypair, program_context::ProgramError> {
    Keypair::from_bytes(&secret)
        .map_err(|e| program_context::ProgramError::Signature(e.to_string()))
}

#[tokio::test]
async fn test_lookup_table_devnet() -> Result<(), program_context::ProgramError> {
    // ---------------------------------------------------------------------
    // -- Setup basic context and accounts needed for devnet setup
    // ---------------------------------------------------------------------

    let mut program_context: Box<dyn program_context::ProgramContext> =
        Box::new(RpcClient::new("https://api.devnet.solana.com".to_string()));

    // Eyh77zP5b7arPtPgpnCT8vsGmq9p5Z9HHnBSeQLnAFQi
    let payer = create_keypair([
        219, 139, 131, 236, 34, 125, 165, 13, 18, 248, 93, 160, 73, 236, 214, 251, 179, 235, 124,
        126, 56, 47, 222, 28, 166, 239, 130, 126, 66, 127, 26, 187, 207, 173, 205, 133, 48, 102, 2,
        219, 20, 234, 72, 102, 53, 122, 175, 166, 198, 11, 198, 248, 59, 40, 137, 208, 193, 138,
        197, 171, 147, 124, 212, 175,
    ])?;
    // aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC
    let authority = create_keypair([
        197, 246, 88, 131, 17, 216, 175, 8, 72, 13, 40, 236, 135, 104, 59, 108, 17, 106, 164, 234,
        46, 136, 171, 148, 111, 176, 32, 136, 59, 253, 224, 247, 8, 156, 98, 175, 196, 123, 178,
        151, 182, 220, 253, 138, 191, 233, 135, 182, 173, 175, 33, 68, 162, 191, 254, 166, 133,
        219, 8, 10, 17, 154, 146, 223,
    ])?;

    // Reuse somewhat common mint used by credix for devnet USDC
    let collateral_mint = Pubkey::from_str("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr").unwrap();

    // ---------------------------------------------------------------------
    // -- Useful ATAs
    // ---------------------------------------------------------------------

    let authority_collateral =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_context,
            &payer,
            &collateral_mint,
            &authority.pubkey(),
        )
        .await?;
    let authority_redeemable =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_context,
            &payer,
            &program_uxd::accounts::find_redeemable_mint_pda().0,
            &authority.pubkey(),
        )
        .await?;

    // ---------------------------------------------------------------------
    // -- Dummy lookup table check
    // ---------------------------------------------------------------------

    // Find needed accounts
    let controller = program_uxd::accounts::find_controller_pda().0;
    let redeemable_mint = program_uxd::accounts::find_redeemable_mint_pda().0;
    let identity_depository = program_uxd::accounts::find_identity_depository_pda().0;
    let identity_depository_collateral_vault =
        program_uxd::accounts::find_identity_depository_collateral_vault_pda().0;

    // Create lookup table
    let recent_slot = program_context.get_slot().await?;
    let (create_ix, table_pk) =
        solana_address_lookup_table_program::instruction::create_lookup_table(
            payer.pubkey(),
            payer.pubkey(),
            recent_slot,
        );
    program_context::process_instruction(&mut program_context, create_ix, &payer).await?;

    // Fill account list
    let accounts_keys = vec![
        authority.pubkey(),
        payer.pubkey(),
        controller,
        identity_depository,
        identity_depository_collateral_vault,
        redeemable_mint,
        authority_collateral,
        authority_redeemable,
        solana_sdk::system_program::ID,
        anchor_spl::token::ID,
        uxd::id(),
    ];
    let extend_ix = solana_address_lookup_table_program::instruction::extend_lookup_table(
        table_pk,
        payer.pubkey(),
        Some(payer.pubkey()),
        accounts_keys.clone(),
    );
    program_context::process_instruction(&mut program_context, extend_ix, &payer).await?;

    // Execute IX
    let accounts = uxd::accounts::MintWithIdentityDepository {
        authority: authority.pubkey(),
        user: authority.pubkey(),
        payer: payer.pubkey(),
        controller,
        depository: identity_depository,
        collateral_vault: identity_depository_collateral_vault,
        redeemable_mint,
        user_collateral: authority_collateral,
        user_redeemable: authority_redeemable,
        system_program: solana_sdk::system_program::ID,
        token_program: anchor_spl::token::ID,
        unused12: uxd::id(), // unused
        unused13: uxd::id(), // unused
        unused14: uxd::id(), // unused
        unused15: uxd::id(), // unused
        unused16: uxd::id(), // unused
        unused17: uxd::id(), // unused
        unused18: uxd::id(), // unused
        unused19: uxd::id(), // unused
        unused20: uxd::id(), // unused
        unused21: uxd::id(), // unused
        unused22: uxd::id(), // unused
        unused23: uxd::id(), // unused
        unused24: uxd::id(), // unused
        unused25: uxd::id(), // unused
        unused26: uxd::id(), // unused
        unused27: uxd::id(), // unused
        unused28: uxd::id(), // unused
        unused29: uxd::id(), // unused
        unused30: uxd::id(), // unused
        unused31: uxd::id(), // unused
        unused32: uxd::id(), // unused
        unused33: uxd::id(), // unused
        unused34: uxd::id(), // unused
        unused35: uxd::id(), // unused
        unused36: uxd::id(), // unused
        unused37: uxd::id(), // unused
        unused38: uxd::id(), // unused
        unused39: uxd::id(), // unused
        unused40: uxd::id(), // unused
    };
    let payload = uxd::instruction::MintWithIdentityDepository {
        collateral_amount: 1_000_000 / 100,
    };
    let instruction = Instruction {
        program_id: uxd::id(),
        accounts: accounts.to_account_metas(None),
        data: payload.data(),
    };
    let blockhash = program_context.get_latest_blockhash().await?;
    let tx = VersionedTransaction::try_new(
        VersionedMessage::V0(
            v0::Message::try_compile(
                &payer.pubkey(),
                &[instruction],
                &[AddressLookupTableAccount {
                    key: table_pk,
                    addresses: accounts_keys.clone(),
                }],
                blockhash,
            )
            .map_err(program_context::ProgramError::Compile)?,
        ),
        &[&payer, &authority],
    )
    .map_err(program_context::ProgramError::Signer)?;

    program_context.process_transaction_versionned(tx).await?;

    // Done
    Ok(())
}
