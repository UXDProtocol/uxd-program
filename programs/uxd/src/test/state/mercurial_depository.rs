#[cfg(test)]
mod test_mercurial_depository {
    use crate::state::mercurial_vault_depository::MERCURIAL_VAULT_DEPOSITORY_SPACE;
    use anchor_lang::Result;

    #[test]
    fn test_mercurial_depository_space() -> Result<()> {
        assert_eq!(MERCURIAL_VAULT_DEPOSITORY_SPACE, 900);
        Ok(())
    }
}
