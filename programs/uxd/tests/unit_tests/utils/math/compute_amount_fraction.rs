// Unit tests
#[cfg(test)]
mod test_compute_amount_fraction {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::compute_amount_fraction;

    #[test]
    fn test_correctness() -> Result<()> {
        // Test regular cases
        assert_eq!(compute_amount_fraction(0, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction(0, 10, 100)?, 0);
        assert_eq!(compute_amount_fraction(0, 100, 100)?, 0);
        assert_eq!(compute_amount_fraction(0, 900, 100)?, 0);

        assert_eq!(compute_amount_fraction(100, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction(100, 10, 100)?, 10);
        assert_eq!(compute_amount_fraction(100, 100, 100)?, 100);
        assert_eq!(compute_amount_fraction(100, 900, 100)?, 900);

        assert_eq!(compute_amount_fraction(10, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction(10, 10, 100)?, 1);
        assert_eq!(compute_amount_fraction(10, 100, 100)?, 10);
        assert_eq!(compute_amount_fraction(10, 900, 100)?, 90);

        assert_eq!(compute_amount_fraction(1000, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction(1000, 10, 100)?, 100);
        assert_eq!(compute_amount_fraction(1000, 100, 100)?, 1000);
        assert_eq!(compute_amount_fraction(1000, 900, 100)?, 9000);

        assert_eq!(compute_amount_fraction(1000, 0, 10000)?, 0);
        assert_eq!(compute_amount_fraction(1000, 10, 10000)?, 1);
        assert_eq!(compute_amount_fraction(1000, 100, 10000)?, 10);

        // Test proper precision loss behavior
        assert_eq!(compute_amount_fraction(1000, 1, 10000)?, 0);
        assert_eq!(compute_amount_fraction(1000, 9999, 10000)?, 999);
        assert_eq!(compute_amount_fraction(1000, 10001, 10000)?, 1000);

        // Test large amounts against u64 overflow
        assert_eq!(
            compute_amount_fraction(
                1_000_000_000_000_000_000,
                2_000_000_000_000_000_000,
                4_000_000_000_000_000_000
            )?,
            500_000_000_000_000_000
        );
        assert_eq!(
            compute_amount_fraction(
                1_000_000_000_000_000_000,
                4_000_000_000_000_000_000,
                2_000_000_000_000_000_000,
            )?,
            2_000_000_000_000_000_000
        );
        assert_eq!(
            compute_amount_fraction(u64::MAX, u64::MAX, u64::MAX)?,
            u64::MAX
        );
        assert_eq!(compute_amount_fraction(u64::MAX, 42, u64::MAX)?, 42);
        assert_eq!(compute_amount_fraction(42, u64::MAX, u64::MAX)?, 42);
        assert_eq!(compute_amount_fraction(u64::MAX, 42, 42)?, u64::MAX);

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert!(compute_amount_fraction(0, 0, 0).is_err());
        assert!(compute_amount_fraction(0, 10, 0).is_err());
        assert!(compute_amount_fraction(10, 0, 0).is_err());
        assert!(compute_amount_fraction(10, 10, 0).is_err());
        assert!(compute_amount_fraction(u64::MAX, 101, 100).is_err());
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(amount: u64, numerator: u64, denominator: u64)| {
            let result = compute_amount_fraction(
                amount,
                numerator,
                denominator
            );
            // Some cases are supposed to fail
            if denominator == 0 {
                prop_assert!(result.is_err());
                return Ok(());
            }
            // As long as we're not going past the amount value,
            // We should not panic, otherwise we could go past the amount's u64
            if numerator < denominator {
                prop_assert!(result.is_ok());
            }
        });
        Ok(())
    }
}