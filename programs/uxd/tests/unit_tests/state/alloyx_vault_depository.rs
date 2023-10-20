// Unit tests
#[cfg(test)]
mod test_alloyx_vault_depository {
    use anchor_lang::Result;
    use std::mem::size_of;
    use uxd::state::alloyx_vault_depository::ALLOYX_VAULT_DEPOSITORY_SPACE;

    #[test]
    fn test_alloyx_vault_depository_space() -> Result<()> {
        assert_eq!(ALLOYX_VAULT_DEPOSITORY_SPACE, 1152);
        assert_eq!(
            size_of::<uxd::state::alloyx_vault_depository::AlloyxVaultDepository>(),
            ALLOYX_VAULT_DEPOSITORY_SPACE - 8
        );
        Ok(())
    }
}
