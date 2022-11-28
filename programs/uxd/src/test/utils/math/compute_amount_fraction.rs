// Unit tests
#[cfg(test)]
mod test_compute_amount_fraction {
    use crate::utils::compute_amount_fraction;
    use anchor_lang::Result;

    #[test]
    fn test_correctness() -> Result<()> {
        assert_eq!(compute_amount_fraction(0, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction(0, 10, 100)?, 0);
        assert_eq!(compute_amount_fraction(0, 100, 100)?, 0);

        assert_eq!(compute_amount_fraction(100, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction(100, 10, 100)?, 10);
        assert_eq!(compute_amount_fraction(100, 100, 100)?, 100);

        assert_eq!(compute_amount_fraction(10, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction(10, 10, 100)?, 1);
        assert_eq!(compute_amount_fraction(10, 100, 100)?, 10);

        assert_eq!(compute_amount_fraction(1000, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction(1000, 10, 100)?, 100);
        assert_eq!(compute_amount_fraction(1000, 100, 100)?, 1000);

        assert_eq!(compute_amount_fraction(1000, 0, 10000)?, 0);
        assert_eq!(compute_amount_fraction(1000, 10, 10000)?, 1);
        assert_eq!(compute_amount_fraction(1000, 100, 10000)?, 10);

        // Test proper precision loss behavior
        assert_eq!(compute_amount_fraction(1000, 1, 10000)?, 0);
        assert_eq!(compute_amount_fraction(1000, 9999, 10000)?, 999);
        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(compute_amount_fraction(0, 0, 0).is_err(), true);
        assert_eq!(compute_amount_fraction(0, 10, 0).is_err(), true);
        assert_eq!(compute_amount_fraction(10, 0, 0).is_err(), true);
        assert_eq!(compute_amount_fraction(10, 10, 0).is_err(), true);
        Ok(())
    }
}
