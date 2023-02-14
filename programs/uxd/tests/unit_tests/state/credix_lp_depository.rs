// Unit tests
#[cfg(test)]
mod test_credix_lp_depository {
    use anchor_lang::Result;
    use uxd::state::CREDIX_LP_DEPOSITORY_SPACE;

    #[test]
    fn test_credix_lp_depository_space() -> Result<()> {
        assert_eq!(CREDIX_LP_DEPOSITORY_SPACE, 1197);
        Ok(())
    }
}
