// Unit tests
#[cfg(test)]
mod test_compute_amount_less_fraction {
    use crate::utils::compute_amount_less_fraction;
    use anchor_lang::Result;

    #[test]
    fn test_correctness() -> Result<()> {
        assert_eq!(compute_amount_less_fraction(0, 0, 100)?, 0);
        assert_eq!(compute_amount_less_fraction(0, 10, 100)?, 0);
        assert_eq!(compute_amount_less_fraction(0, 100, 100)?, 0);

        assert_eq!(compute_amount_less_fraction(100, 0, 100)?, 100);
        assert_eq!(compute_amount_less_fraction(100, 10, 100)?, 90);
        assert_eq!(compute_amount_less_fraction(100, 100, 100)?, 0);

        assert_eq!(compute_amount_less_fraction(10, 0, 100)?, 10);
        assert_eq!(compute_amount_less_fraction(10, 10, 100)?, 9);
        assert_eq!(compute_amount_less_fraction(10, 100, 100)?, 0);

        assert_eq!(compute_amount_less_fraction(1000, 0, 100)?, 1000);
        assert_eq!(compute_amount_less_fraction(1000, 10, 100)?, 900);
        assert_eq!(compute_amount_less_fraction(1000, 100, 100)?, 0);

        assert_eq!(compute_amount_less_fraction(1000, 0, 10000)?, 1000);
        assert_eq!(compute_amount_less_fraction(1000, 10, 10000)?, 999);
        assert_eq!(compute_amount_less_fraction(1000, 100, 10000)?, 990);

        // Test proper precision loss behavior
        assert_eq!(compute_amount_less_fraction(1000, 1, 10000)?, 999);
        assert_eq!(compute_amount_less_fraction(1000, 9999, 10000)?, 0);
        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(compute_amount_less_fraction(0, 0, 0).is_err(), true);
        assert_eq!(compute_amount_less_fraction(0, 10, 0).is_err(), true);
        assert_eq!(compute_amount_less_fraction(10, 0, 0).is_err(), true);
        assert_eq!(compute_amount_less_fraction(10, 10, 0).is_err(), true);

        assert_eq!(compute_amount_less_fraction(0, 101, 100).is_err(), true);
        Ok(())
    }
}
