// Unit tests
#[cfg(test)]
mod test_checked_i64_to_u64 {
    use crate::utils::checked_i64_to_u64;
    use anchor_lang::Result;

    #[test]
    fn test_correctness() -> Result<()> {
        assert_eq!(checked_i64_to_u64(0)?, 0);
        assert_eq!(checked_i64_to_u64(1)?, 1);
        assert_eq!(checked_i64_to_u64(9999)?, 9999);
        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(checked_i64_to_u64(-1).is_err(), true);
        assert_eq!(checked_i64_to_u64(-9999).is_err(), true);
        Ok(())
    }
}
