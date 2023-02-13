use solana_sdk::pubkey::Pubkey;

pub fn find_identity_depository_address() -> Pubkey {
    let (identity_depository, _) =
        Pubkey::find_program_address(&[uxd::IDENTITY_DEPOSITORY_NAMESPACE.as_ref()], &uxd::id());
    assert_eq!(
        "BgkHf7mAtNwtnu2uCJqSJWbFdiXCoMBpNZmgVJJmsGLW",
        identity_depository.to_string()
    );
    return identity_depository;
}
