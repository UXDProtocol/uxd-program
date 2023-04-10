// Unit tests
#[cfg(test)]
mod test_compute_amount_fraction_ceil {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::compute_amount_fraction_ceil;

    #[test]
    fn test_correctness() -> Result<()> {
        // Test regular cases
        assert_eq!(compute_amount_fraction_ceil(0, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction_ceil(0, 10, 100)?, 0);
        assert_eq!(compute_amount_fraction_ceil(0, 100, 100)?, 0);

        assert_eq!(compute_amount_fraction_ceil(100, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction_ceil(100, 10, 100)?, 10);
        assert_eq!(compute_amount_fraction_ceil(100, 100, 100)?, 100);

        assert_eq!(compute_amount_fraction_ceil(10, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction_ceil(10, 10, 100)?, 1);
        assert_eq!(compute_amount_fraction_ceil(10, 100, 100)?, 10);

        assert_eq!(compute_amount_fraction_ceil(1000, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction_ceil(1000, 10, 100)?, 100);
        assert_eq!(compute_amount_fraction_ceil(1000, 100, 100)?, 1000);

        assert_eq!(compute_amount_fraction_ceil(1000, 0, 10000)?, 0);
        assert_eq!(compute_amount_fraction_ceil(1000, 10, 10000)?, 1);
        assert_eq!(compute_amount_fraction_ceil(1000, 100, 10000)?, 10);

        assert_eq!(compute_amount_fraction_ceil(1000, 200, 100)?, 2000);
        assert_eq!(compute_amount_fraction_ceil(1, 1000, 100)?, 10);

        // Test proper precision loss behavior
        assert_eq!(compute_amount_fraction_ceil(1000, 1, 10000)?, 1);
        assert_eq!(compute_amount_fraction_ceil(1000, 9999, 10000)?, 1000);

        // Test large amounts against u64 overflow
        assert_eq!(
            compute_amount_fraction_ceil(
                1_000_000_000_000_000_000,
                2_000_000_000_000_000_000,
                4_000_000_000_000_000_000
            )?,
            500_000_000_000_000_000
        );
        assert_eq!(
            compute_amount_fraction_ceil(u64::MAX, u64::MAX, u64::MAX)?,
            u64::MAX
        );
        assert_eq!(compute_amount_fraction_ceil(u64::MAX, 42, u64::MAX)?, 42);
        assert_eq!(compute_amount_fraction_ceil(42, u64::MAX, u64::MAX)?, 42);
        assert_eq!(compute_amount_fraction_ceil(u64::MAX, 42, 42)?, u64::MAX);

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert!(compute_amount_fraction_ceil(0, 0, 0).is_err());
        assert!(compute_amount_fraction_ceil(0, 10, 0).is_err());
        assert!(compute_amount_fraction_ceil(10, 0, 0).is_err());
        assert!(compute_amount_fraction_ceil(10, 10, 0).is_err());
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(amount: u64, numerator: u64, denominator: u64)| {
            let result = compute_amount_fraction_ceil(
                amount,
                numerator,
                denominator
            );
            // Some cases are supposed to fail
            if denominator == 0 {
                prop_assert!(result.is_err());
                return Ok(());
            }
            if u128::from(amount) * u128::from(numerator) / u128::from(denominator) > u128::from(u64::MAX) {
                prop_assert!(result.is_err());
                return Ok(());
            }
            // In all other cases, we should not panic
            prop_assert!(result.is_ok());
        });
        Ok(())
    }
}
