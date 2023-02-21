// Unit tests
#[cfg(test)]
mod test_compute_decrease {
    use anchor_lang::Result;
    use uxd::utils::compute_decrease;

    #[test]
    fn test_decrease() -> Result<()> {
        assert_eq!(compute_decrease(0, 0)?, 0);
        assert_eq!(compute_decrease(1, 0)?, 1);
        assert_eq!(compute_decrease(1_000_000, 0)?, 1_000_000);
        assert_eq!(compute_decrease(2_000_000, 5)?, 1_999_995);
        assert_eq!(compute_decrease(5_000_000, 4_000_000)?, 1_000_000);
        Ok(())
    }

    #[test]
    fn test_increase() -> Result<()> {
        assert_eq!(compute_decrease(0, 1).is_err(), true);
        assert_eq!(compute_decrease(0, 1_000_000).is_err(), true);
        assert_eq!(compute_decrease(5, 2_000_000).is_err(), true);
        assert_eq!(compute_decrease(4_000_000, 5_000_000).is_err(), true);
        Ok(())
    }
}
