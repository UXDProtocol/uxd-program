// Unit tests
#[cfg(test)]
mod test_credix_lp_depository {
    use anchor_lang::Result;
    use std::mem::size_of;
    use uxd::state::credix_lp_depository::CREDIX_LP_DEPOSITORY_SPACE;

    #[test]
    fn test_credix_lp_depository_space() -> Result<()> {
        assert_eq!(CREDIX_LP_DEPOSITORY_SPACE, 1197);
        assert_eq!(
            size_of::<uxd::state::credix_lp_depository::CredixLpDepository>(),
            CREDIX_LP_DEPOSITORY_SPACE - 8
        );
        Ok(())
    }
}
