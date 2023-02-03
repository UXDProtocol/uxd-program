// Unit tests
#[cfg(test)]
mod test_compute_increase {
    use uxd::utils::compute_increase;
    use anchor_lang::Result;

    #[test]
    fn test_decrease() -> Result<()> {
        assert_eq!(compute_increase(1, 0).is_err(), true);
        assert_eq!(compute_increase(1_000_000, 0).is_err(), true);
        assert_eq!(compute_increase(2_000_000, 5).is_err(), true);
        assert_eq!(compute_increase(5_000_000, 4_000_000).is_err(), true);
        Ok(())
    }

    #[test]
    fn test_increase() -> Result<()> {
        assert_eq!(compute_increase(0, 0)?, 0);
        assert_eq!(compute_increase(0, 1)?, 1);
        assert_eq!(compute_increase(0, 1_000_000)?, 1_000_000);
        assert_eq!(compute_increase(5, 2_000_000)?, 1_999_995);
        assert_eq!(compute_increase(4_000_000, 5_000_000)?, 1_000_000);
        Ok(())
    }
}
