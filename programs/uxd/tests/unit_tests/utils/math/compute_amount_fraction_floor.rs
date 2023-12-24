// Unit tests
#[cfg(test)]
mod test_compute_amount_fraction_floor {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::compute_amount_fraction_floor;

    #[test]
    fn test_correctness() -> Result<()> {
        // Test regular cases
        assert_eq!(compute_amount_fraction_floor(0, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction_floor(0, 10, 100)?, 0);
        assert_eq!(compute_amount_fraction_floor(0, 100, 100)?, 0);

        assert_eq!(compute_amount_fraction_floor(100, 0, 100)?, 0);
        assert_eq!(compute_amount_fraction_floor(100, 10, 100)?, 10);
        assert_eq!(compute_amount_fraction_floor(100, 100, 100)?, 100);

        assert_eq!(compute_amount_fraction_floor(10, 0, 100)?, 10);
        assert_eq!(compute_amount_fraction_floor(10, 10, 100)?, 9);
        assert_eq!(compute_amount_fraction_floor(10, 100, 100)?, 0);

        assert_eq!(compute_amount_fraction_floor(1000, 0, 100)?, 1000);
        assert_eq!(compute_amount_fraction_floor(1000, 10, 100)?, 900);
        assert_eq!(compute_amount_fraction_floor(1000, 100, 100)?, 0);

        assert_eq!(compute_amount_fraction_floor(1000, 0, 10000)?, 1000);
        assert_eq!(compute_amount_fraction_floor(1000, 10, 10000)?, 999);
        assert_eq!(compute_amount_fraction_floor(1000, 100, 10000)?, 990);

        // Test proper precision loss behavior
        assert_eq!(compute_amount_fraction_floor(1000, 1, 10000)?, 999);
        assert_eq!(compute_amount_fraction_floor(1000, 9999, 10000)?, 0);

        // Test large amounts against u64 overflow
        assert_eq!(
            compute_amount_fraction_floor(
                1_000_000_000_000_000_000,
                2_000_000_000_000_000_000,
                4_000_000_000_000_000_000
            )?,
            500_000_000_000_000_000
        );
        assert_eq!(
            compute_amount_fraction_floor(u64::MAX, u64::MAX, u64::MAX)?,
            0
        );
        assert_eq!(
            compute_amount_fraction_floor(u64::MAX, 42, u64::MAX)?,
            u64::MAX - 42
        );
        assert_eq!(compute_amount_fraction_floor(42, u64::MAX, u64::MAX)?, 0);
        assert_eq!(compute_amount_fraction_floor(u64::MAX, 42, 42)?, 0);

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert!(compute_amount_fraction_floor(0, 0, 0).is_err());
        assert!(compute_amount_fraction_floor(0, 10, 0).is_err());
        assert!(compute_amount_fraction_floor(10, 0, 0).is_err());
        assert!(compute_amount_fraction_floor(10, 10, 0).is_err());

        assert!(compute_amount_fraction_floor(0, 101, 100).is_err());
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(amount: u64, numerator: u64, denominator: u64)| {
            let result = compute_amount_fraction_floor(
                amount,
                numerator,
                denominator
            );
            // Some cases are supposed to fail
            if denominator == 0 {
                prop_assert!(result.is_err());
                return Ok(());
            }
            if numerator > denominator {
                prop_assert!(result.is_err());
                return Ok(());
            }
            // In all other cases, we should not panic
            prop_assert!(result.is_ok());
        });
        Ok(())
    }
}
