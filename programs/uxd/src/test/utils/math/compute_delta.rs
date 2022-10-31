// Unit tests
#[cfg(test)]
mod test_compute_delta {
    use crate::utils::compute_delta;
    use anchor_lang::Result;

    #[test]
    fn test_decrease() -> Result<()> {
        assert_eq!(compute_delta(0, 0)?, 0);
        assert_eq!(compute_delta(1, 0)?, -1);
        assert_eq!(compute_delta(1_000_000, 0)?, -1_000_000);
        assert_eq!(compute_delta(2_000_000, 5)?, -1_999_995);
        assert_eq!(compute_delta(5_000_000, 4_000_000)?, -1_000_000);
        Ok(())
    }

    #[test]
    fn test_increase() -> Result<()> {
        assert_eq!(compute_delta(0, 0)?, 0);
        assert_eq!(compute_delta(0, 1)?, 1);
        assert_eq!(compute_delta(0, 1_000_000)?, 1_000_000);
        assert_eq!(compute_delta(5, 2_000_000)?, 1_999_995);
        assert_eq!(compute_delta(4_000_000, 5_000_000)?, 1_000_000);
        Ok(())
    }
}
