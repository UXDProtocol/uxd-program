#[cfg(test)]
mod test_mercurial_vault_depository {
    use anchor_lang::Result;
    use std::mem::size_of;
    use uxd::state::mercurial_vault_depository::MERCURIAL_VAULT_DEPOSITORY_SPACE;

    #[test]
    fn test_mercurial_vault_depository_space() -> Result<()> {
        assert_eq!(MERCURIAL_VAULT_DEPOSITORY_SPACE, 900);
        assert_eq!(
            size_of::<uxd::state::mercurial_vault_depository::MercurialVaultDepository>(),
            MERCURIAL_VAULT_DEPOSITORY_SPACE - 8
        );
        Ok(())
    }
}
