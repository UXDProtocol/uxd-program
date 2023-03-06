use solana_program::pubkey::Pubkey;

pub fn find_controller() -> Pubkey {
    Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE], &uxd::id()).0
}

pub fn find_redeemable_mint() -> Pubkey {
    Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE], &uxd::id()).0
}

pub fn find_identity_depository() -> Pubkey {
    Pubkey::find_program_address(&[uxd::IDENTITY_DEPOSITORY_NAMESPACE], &uxd::id()).0
}

pub fn find_identity_depository_collateral_vault() -> Pubkey {
    Pubkey::find_program_address(&[uxd::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE], &uxd::id()).0
}

pub fn find_credix_lp_depository(
    collateral_mint: &Pubkey,
    credix_global_market_state: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &[
            uxd::CREDIX_LP_DEPOSITORY_NAMESPACE,
            credix_global_market_state.as_ref(),
            collateral_mint.as_ref(),
        ],
        &uxd::id(),
    )
    .0
}

pub fn find_credix_lp_depository_collateral(
    depository: &Pubkey,
    collateral_mint: &Pubkey,
) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(depository, collateral_mint)
}

pub fn find_credix_lp_depository_shares(
    depository: &Pubkey,
    credix_shares_mint: &Pubkey,
) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(depository, credix_shares_mint)
}
