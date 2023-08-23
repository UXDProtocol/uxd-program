use solana_sdk::pubkey::Pubkey;
pub fn find_controller_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE], &uxd::id())
}

pub fn find_redeemable_mint_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[uxd::REDEEMABLE_MINT_NAMESPACE], &uxd::id())
}

pub fn find_identity_depository_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[uxd::IDENTITY_DEPOSITORY_NAMESPACE], &uxd::id())
}

pub fn find_identity_depository_collateral_vault_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[uxd::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE], &uxd::id())
}

pub fn find_mercurial_vault_depository_pda(
    collateral_mint: &Pubkey,
    mercurial_vault: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            uxd::MERCURIAL_VAULT_DEPOSITORY_NAMESPACE,
            mercurial_vault.as_ref(),
            collateral_mint.as_ref(),
        ],
        &uxd::id(),
    )
}

pub fn find_mercurial_vault_depository_lp_token_vault_pda(
    collateral_mint: &Pubkey,
    mercurial_vault: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            uxd::MERCURIAL_VAULT_DEPOSITORY_LP_TOKEN_VAULT_NAMESPACE,
            mercurial_vault.as_ref(),
            collateral_mint.as_ref(),
        ],
        &uxd::id(),
    )
}

pub fn find_credix_lp_depository_pda(
    collateral_mint: &Pubkey,
    credix_global_market_state: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            uxd::CREDIX_LP_DEPOSITORY_NAMESPACE,
            credix_global_market_state.as_ref(),
            collateral_mint.as_ref(),
        ],
        &uxd::id(),
    )
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
