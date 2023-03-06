// Unit tests
#[cfg(test)]
mod test_is_within_range_inclusive {
    use anchor_lang::Result;
    use uxd::utils::is_within_range_inclusive;

    #[test]
    fn test_equality() -> Result<()> {
        assert!(is_within_range_inclusive(0, 0, 1));
        assert!(is_within_range_inclusive(1, 0, 1));
        assert!(is_within_range_inclusive(1000, 0, 1000));
        assert!(is_within_range_inclusive(999, 0, 1000));
        assert!(is_within_range_inclusive(1, 0, 1000));
        assert!(is_within_range_inclusive(0, 0, 1000));
        assert!(is_within_range_inclusive(2000, 1000, 2000));
        assert!(is_within_range_inclusive(1999, 1000, 2000));
        assert!(is_within_range_inclusive(1001, 1000, 2000));
        assert!(is_within_range_inclusive(1000, 1000, 2000));
        Ok(())
    }

    #[test]
    fn test_inequality() -> Result<()> {
        assert!(!is_within_range_inclusive(2, 0, 1));
        assert!(!is_within_range_inclusive(1001, 0, 1000));
        assert!(!is_within_range_inclusive(2001, 1000, 2000));
        assert!(!is_within_range_inclusive(999, 1000, 2000));
        Ok(())
    }
}
