// Unit tests
#[cfg(test)]
mod test_maple_pool_depository {
    use crate::state::maple_pool_depository::MAPLE_POOL_DEPOSITORY_SPACE;
    use anchor_lang::Result;

    #[test]
    fn test_maple_pool_depository_space() -> Result<()> {
        assert_eq!(MAPLE_POOL_DEPOSITORY_SPACE, 1214);
        Ok(())
    }
}
