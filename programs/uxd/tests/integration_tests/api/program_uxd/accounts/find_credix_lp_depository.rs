use solana_program::pubkey::Pubkey;

use crate::integration_tests::api::program_credix;

pub struct CredixLpDepository {
    pub collateral_mint: Pubkey,
    pub depository: Pubkey,
    pub depository_collateral: Pubkey,
    pub depository_shares: Pubkey,
    pub credix_program_state: Pubkey,
    pub credix_global_market_state: Pubkey,
    pub credix_signing_authority: Pubkey,
    pub credix_liquidity_collateral: Pubkey,
    pub credix_shares_mint: Pubkey,
    pub credix_treasury_collateral: Pubkey,
}

pub fn find_credix_lp_depository(
    context: &program_credix::accounts::Context,
) -> CredixLpDepository {
    let collateral_mint = context.base_token_mint;

    let credix_program_state = context.program_state;
    let credix_global_market_state = context.global_market_state;
    let credix_signing_authority = context.signing_authority;
    let credix_liquidity_collateral = context.liquidity_pool_token_account;
    let credix_shares_mint = context.lp_token_mint;
    let credix_treasury_collateral = context.treasury_pool_token_account;

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

    CredixLpDepository {
        collateral_mint,
        depository,
        depository_collateral,
        depository_shares,
        credix_program_state,
        credix_global_market_state,
        credix_signing_authority,
        credix_liquidity_collateral,
        credix_shares_mint,
        credix_treasury_collateral,
    }
}
