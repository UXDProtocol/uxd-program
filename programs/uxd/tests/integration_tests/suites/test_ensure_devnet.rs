use solana_program_test::tokio;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use std::str::FromStr;
use uxd::instructions::EditAlloyxVaultDepositoryFields;

use uxd::instructions::EditControllerFields;
use uxd::instructions::EditCredixLpDepositoryFields;
use uxd::instructions::EditDepositoriesRoutingWeightBps;
use uxd::instructions::EditIdentityDepositoryFields;
use uxd::instructions::EditMercurialVaultDepositoryFields;
use uxd::instructions::EditRouterDepositories;

use crate::integration_tests::api::program_alloyx;
use crate::integration_tests::api::program_context;
use crate::integration_tests::api::program_credix;
use crate::integration_tests::api::program_mercurial;
use crate::integration_tests::api::program_spl;
use crate::integration_tests::api::program_uxd;
use crate::integration_tests::utils::ui_amount_to_native_amount;

use solana_client::nonblocking::rpc_client::RpcClient;

fn create_keypair(secret: [u8; 64]) -> Result<Keypair, program_context::ProgramError> {
    Keypair::from_bytes(&secret)
        .map_err(|e| program_context::ProgramError::Signature(e.to_string()))
}

#[tokio::test]
async fn test_ensure_devnet() -> Result<(), program_context::ProgramError> {
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

    // Create a collateral account for our profits_beneficiary (owned by authority)
    let profits_beneficiary_collateral =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_context,
            &payer,
            &collateral_mint,
            &authority.pubkey(),
        )
        .await?;

    // Set all caps to a very large amount (1B)
    let supply_cap = ui_amount_to_native_amount(1_000_000_000, 6);

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
    // -- Setup onchain dependency mercurial vault instance
    // ---------------------------------------------------------------------

    // tewho86AFqTGmMvtKEvnNegHZfce4tTzDYENa58TLCq
    let mercurial_vault_lp_mint = create_keypair([
        90, 138, 35, 214, 209, 183, 0, 86, 76, 138, 199, 70, 48, 104, 9, 227, 94, 43, 67, 26, 233,
        128, 61, 117, 130, 99, 181, 114, 127, 100, 200, 129, 13, 59, 134, 19, 81, 172, 155, 180,
        150, 234, 35, 53, 105, 199, 116, 239, 239, 77, 142, 60, 202, 215, 83, 80, 173, 34, 95, 47,
        34, 66, 44, 26,
    ])?;
    if !program_context::read_account_exists(
        &mut program_context,
        &mercurial_vault_lp_mint.pubkey(),
    )
    .await?
    {
        program_mercurial::procedures::process_deploy_program(
            &mut program_context,
            &payer,
            &collateral_mint,
            &mercurial_vault_lp_mint,
            6,
        )
        .await?;
    }

    let authority_mercurial_shares =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_context,
            &payer,
            &mercurial_vault_lp_mint.pubkey(),
            &authority.pubkey(),
        )
        .await?;
    program_mercurial::instructions::process_deposit(
        &mut program_context,
        &collateral_mint,
        &mercurial_vault_lp_mint.pubkey(),
        &authority,
        &authority_collateral,
        &authority_mercurial_shares,
        10_000, // 0.01 collateral
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Setup onchain dependency alloyx vault instance
    // ---------------------------------------------------------------------

    // CBQcnyoVjdCyPf2nnhPjbMJL18FEtTuPA9nQPrS7wJPF
    let alloyx_vault_mint = create_keypair([
        24, 152, 214, 2, 31, 95, 161, 3, 227, 120, 36, 77, 192, 51, 24, 16, 64, 187, 121, 207, 7,
        13, 244, 15, 232, 77, 205, 88, 244, 20, 110, 43, 166, 27, 17, 120, 31, 187, 68, 35, 240,
        48, 173, 183, 137, 158, 95, 45, 136, 43, 150, 41, 190, 10, 108, 96, 249, 84, 99, 61, 60,
        216, 62, 30,
    ])?;
    if !program_context::read_account_exists(&mut program_context, &alloyx_vault_mint.pubkey())
        .await?
    {
        program_alloyx::procedures::process_deploy_program(
            &mut program_context,
            &payer,
            &collateral_mint,
            &alloyx_vault_mint,
            6,
        )
        .await?;
    }
    let authority_alloyx_shares =
        program_spl::instructions::process_associated_token_account_get_or_init(
            &mut program_context,
            &payer,
            &alloyx_vault_mint.pubkey(),
            &authority.pubkey(),
        )
        .await?;
    program_alloyx::instructions::process_whitelist(
        &mut program_context,
        &payer,
        &authority.pubkey(),
    )
    .await?;
    program_alloyx::instructions::process_deposit(
        &mut program_context,
        &collateral_mint,
        &alloyx_vault_mint.pubkey(),
        &authority,
        &authority_collateral,
        &authority_alloyx_shares,
        10_000, // 0.01 collateral
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Setup controller
    // ---------------------------------------------------------------------

    let controller = program_uxd::accounts::find_controller_pda().0;
    if !program_context::read_account_exists(&mut program_context, &controller).await? {
        program_uxd::instructions::process_initialize_controller(
            &mut program_context,
            &payer,
            &authority,
            6,
        )
        .await?;
    }

    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: Some(u128::from(supply_cap)),
            depositories_routing_weight_bps: None,
            router_depositories: None,
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: Some(100 * 100), // 100%
            slots_per_epoch: Some(172800),
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Setup identity_depository
    // ---------------------------------------------------------------------

    let identity_depository = program_uxd::accounts::find_identity_depository_pda().0;
    if !program_context::read_account_exists(&mut program_context, &identity_depository).await? {
        program_uxd::instructions::process_initialize_identity_depository(
            &mut program_context,
            &payer,
            &authority,
            &collateral_mint,
        )
        .await?;
    }
    program_uxd::instructions::process_edit_identity_depository(
        &mut program_context,
        &payer,
        &authority,
        &EditIdentityDepositoryFields {
            redeemable_amount_under_management_cap: Some(u128::from(supply_cap)),
            minting_disabled: Some(false),
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Setup mercurial_vault_depository
    // ---------------------------------------------------------------------

    let mercurial_base = program_mercurial::accounts::find_base();
    let mercurial_vault =
        program_mercurial::accounts::find_vault_pda(&collateral_mint, &mercurial_base.pubkey()).0;
    let mercurial_vault_depository = program_uxd::accounts::find_mercurial_vault_depository_pda(
        &collateral_mint,
        &mercurial_vault,
    )
    .0;

    if !program_context::read_account_exists(&mut program_context, &mercurial_vault_depository)
        .await?
    {
        program_uxd::instructions::process_register_mercurial_vault_depository(
            &mut program_context,
            &payer,
            &authority,
            &collateral_mint,
            &mercurial_vault_lp_mint.pubkey(),
            0,
            0,
            0,
        )
        .await?;
    }
    program_uxd::instructions::process_edit_mercurial_vault_depository(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint,
        &EditMercurialVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(u128::from(supply_cap)),
            minting_fee_in_bps: Some(100),
            redeeming_fee_in_bps: Some(100),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: Some(profits_beneficiary_collateral),
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Setup credix_lp_depository
    // ---------------------------------------------------------------------

    let credix_market_seeds = program_credix::accounts::find_market_seeds();
    let credix_global_market_state =
        program_credix::accounts::find_global_market_state_pda(&credix_market_seeds).0;
    let credix_lp_depository = program_uxd::accounts::find_credix_lp_depository_pda(
        &collateral_mint,
        &credix_global_market_state,
    )
    .0;

    if !program_context::read_account_exists(&mut program_context, &credix_lp_depository).await? {
        program_uxd::instructions::process_register_credix_lp_depository(
            &mut program_context,
            &payer,
            &authority,
            &collateral_mint,
            0,
            0,
            0,
        )
        .await?;
    }
    program_uxd::instructions::process_edit_credix_lp_depository(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint,
        &EditCredixLpDepositoryFields {
            redeemable_amount_under_management_cap: Some(u128::from(supply_cap)),
            minting_fee_in_bps: Some(100),
            redeeming_fee_in_bps: Some(100),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: Some(profits_beneficiary_collateral),
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Setup alloyx_vault_depository
    // ---------------------------------------------------------------------

    let alloyx_vault_id = program_alloyx::accounts::find_vault_id();
    let alloyx_vault_info = program_alloyx::accounts::find_vault_info(&alloyx_vault_id).0;
    let alloyx_vault_depository = program_uxd::accounts::find_alloyx_vault_depository_pda(
        &alloyx_vault_info,
        &collateral_mint,
    )
    .0;
    if !program_context::read_account_exists(&mut program_context, &alloyx_vault_depository).await?
    {
        program_uxd::instructions::process_register_alloyx_vault_depository(
            &mut program_context,
            &payer,
            &authority,
            &collateral_mint,
            &alloyx_vault_mint.pubkey(),
            0,
            0,
            0,
        )
        .await?;
    }
    program_uxd::instructions::process_edit_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint,
        &EditAlloyxVaultDepositoryFields {
            redeemable_amount_under_management_cap: Some(supply_cap),
            minting_fee_in_bps: Some(100),
            redeeming_fee_in_bps: Some(100),
            minting_disabled: Some(false),
            profits_beneficiary_collateral: Some(profits_beneficiary_collateral),
        },
    )
    .await?;
    program_alloyx::instructions::process_whitelist(
        &mut program_context,
        &payer,
        &alloyx_vault_depository,
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Setup router
    // ---------------------------------------------------------------------

    program_uxd::instructions::process_edit_controller(
        &mut program_context,
        &payer,
        &authority,
        &EditControllerFields {
            redeemable_global_supply_cap: None,
            depositories_routing_weight_bps: Some(EditDepositoriesRoutingWeightBps {
                identity_depository_weight_bps: 30 * 100,        // 30%
                mercurial_vault_depository_weight_bps: 30 * 100, // 30%
                credix_lp_depository_weight_bps: 30 * 100,       // 30%
                alloyx_vault_depository_weight_bps: 10 * 100,    // 10%
            }),
            router_depositories: Some(EditRouterDepositories {
                identity_depository,
                mercurial_vault_depository,
                credix_lp_depository,
                alloyx_vault_depository,
            }),
            outflow_limit_per_epoch_amount: None,
            outflow_limit_per_epoch_bps: None,
            slots_per_epoch: None,
        },
    )
    .await?;

    // ---------------------------------------------------------------------
    // -- Check that all mints work
    // ---------------------------------------------------------------------

    program_uxd::instructions::process_mint_with_identity_depository(
        &mut program_context,
        &payer,
        &authority,
        &authority,
        &authority_collateral,
        &authority_redeemable,
        10_000, // 0.01 collateral
    )
    .await?;

    program_uxd::instructions::process_mint_with_mercurial_vault_depository(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint,
        &mercurial_vault_lp_mint.pubkey(),
        &authority,
        &authority_collateral,
        &authority_redeemable,
        10_000, // 0.01 collateral
    )
    .await?;

    program_uxd::instructions::process_mint_with_credix_lp_depository(
        &mut program_context,
        &payer,
        &authority,
        &collateral_mint,
        &authority,
        &authority_collateral,
        &authority_redeemable,
        10_000, // 0.01 collateral
    )
    .await?;

    program_uxd::instructions::process_rebalance_alloyx_vault_depository(
        &mut program_context,
        &payer,
        &collateral_mint,
        &alloyx_vault_mint.pubkey(),
        &profits_beneficiary_collateral,
        None, // We dont care about the output, just that it suceeded
        None, // We dont care about the output, just that it suceeded
    )
    .await?;

    // Done
    Ok(())
}
