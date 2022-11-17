// Unit tests
#[cfg(test)]
mod test_credix_lp_depository {
    use crate::state::credix_lp_depository::CREDIX_LP_DEPOSITORY_SPACE;
    use anchor_lang::Result;

    #[test]
    fn test_credix_lp_depository_space() -> Result<()> {
        assert_eq!(credix_client::program::Credix);
        assert_eq!(CREDIX_LP_DEPOSITORY_SPACE, 1183);
        Ok(())
    }
}
