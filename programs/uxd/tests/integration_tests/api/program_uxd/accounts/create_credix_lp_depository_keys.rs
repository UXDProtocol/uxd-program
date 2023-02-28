use solana_program::pubkey::Pubkey;

use crate::integration_tests::api::program_credix;

pub struct CredixLpDepositoryKeys {
    pub depository: Pubkey,
    pub depository_collateral: Pubkey,
    pub depository_shares: Pubkey,
    pub credix_program_state: Pubkey,
    pub credix_global_market_state: Pubkey,
    pub credix_signing_authority: Pubkey,
    pub credix_liquidity_collateral: Pubkey,
    pub credix_shares_mint: Pubkey,
    pub credix_treasury_collateral: Pubkey,
    pub credix_pass: Pubkey,
    pub credix_program_keys: program_credix::accounts::ProgramKeys,
}

pub fn create_credix_lp_depository_keys(collateral_mint: &Pubkey) -> CredixLpDepositoryKeys {
    let credix_program_keys = program_credix::accounts::create_program_keys(collateral_mint);

    let credix_program_state = credix_program_keys.program_state;
    let credix_global_market_state = credix_program_keys.global_market_state;
    let credix_signing_authority = credix_program_keys.signing_authority;
    let credix_liquidity_collateral = credix_program_keys.liquidity_pool_token_account;
    let credix_shares_mint = credix_program_keys.lp_token_mint;
    let credix_treasury_collateral = credix_program_keys.treasury_pool_token_account;

    let depository = Pubkey::find_program_address(
        &[
            uxd::CREDIX_LP_DEPOSITORY_NAMESPACE.as_ref(),
            credix_global_market_state.as_ref(),
            collateral_mint.as_ref(),
        ],
        &uxd::id(),
    )
    .0;

    let depository_collateral =
        spl_associated_token_account::get_associated_token_address(&depository, &collateral_mint);
    let depository_shares = spl_associated_token_account::get_associated_token_address(
        &depository,
        &credix_shares_mint,
    );

    let credix_pass = credix_client::CredixPass::generate_pda(
        credix_program_keys.global_market_state,
        depository,
    )
    .0;

    CredixLpDepositoryKeys {
        depository,
        depository_collateral,
        depository_shares,
        credix_program_state,
        credix_global_market_state,
        credix_signing_authority,
        credix_liquidity_collateral,
        credix_shares_mint,
        credix_pass,
        credix_treasury_collateral,
        credix_program_keys,
    }
}
