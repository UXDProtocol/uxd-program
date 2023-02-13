use solana_sdk::pubkey::Pubkey;

pub fn find_identity_collateral_vault_address() -> Pubkey {
    let (identity_collateral_vault, _) = Pubkey::find_program_address(
        &[uxd::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE.as_ref()],
        &uxd::id(),
    );
    assert_eq!(
        "5dT7SJWz9kLFF5jA9czkSwwMeUhHj76mhF1jtvdbAoSL",
        identity_collateral_vault.to_string()
    );
    return identity_collateral_vault;
}
