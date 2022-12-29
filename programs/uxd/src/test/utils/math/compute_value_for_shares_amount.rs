// Unit tests
#[cfg(test)]
mod test_compute_value_for_shares_amount {
    use crate::utils::compute_value_for_shares_amount;
    use anchor_lang::Result;

    #[test]
    fn test_correctness() -> Result<()> {
        // Test regular cases and check correct rounding
        assert_eq!(compute_value_for_shares_amount(0, 0, 0)?, 0);
        assert_eq!(compute_value_for_shares_amount(0, 10, 0)?, 0);
        assert_eq!(compute_value_for_shares_amount(0, 0, 10)?, 0);

        assert_eq!(compute_value_for_shares_amount(0, 100, 100)?, 0);
        assert_eq!(compute_value_for_shares_amount(1, 100, 100)?, 1);
        assert_eq!(compute_value_for_shares_amount(99, 100, 100)?, 99);
        assert_eq!(compute_value_for_shares_amount(100, 100, 100)?, 100);

        assert_eq!(compute_value_for_shares_amount(0, 100, 1)?, 0);
        assert_eq!(compute_value_for_shares_amount(1, 100, 1)?, 0);
        assert_eq!(compute_value_for_shares_amount(99, 100, 1)?, 0);
        assert_eq!(compute_value_for_shares_amount(100, 100, 1)?, 1);

        assert_eq!(compute_value_for_shares_amount(0, 10, 100)?, 0);
        assert_eq!(compute_value_for_shares_amount(1, 10, 100)?, 10);
        assert_eq!(compute_value_for_shares_amount(9, 10, 100)?, 90);
        assert_eq!(compute_value_for_shares_amount(10, 10, 100)?, 100);

        assert_eq!(compute_value_for_shares_amount(0, 100, 10)?, 0);
        assert_eq!(compute_value_for_shares_amount(1, 100, 10)?, 0);
        assert_eq!(compute_value_for_shares_amount(49, 100, 10)?, 4);
        assert_eq!(compute_value_for_shares_amount(50, 100, 10)?, 5);
        assert_eq!(compute_value_for_shares_amount(51, 100, 10)?, 5);
        assert_eq!(compute_value_for_shares_amount(100, 100, 10)?, 10);

        // Test large amounts against u64 overflow
        assert_eq!(
            compute_value_for_shares_amount(
                1_000_000_000_000_000_000,
                2_000_000_000_000_000_000,
                4_000_000_000_000_000_000
            )?,
            2_000_000_000_000_000_000
        );
        assert_eq!(
            compute_value_for_shares_amount(u64::MAX, u64::MAX, u64::MAX)?,
            u64::MAX
        );
        assert_eq!(compute_value_for_shares_amount(42, u64::MAX, u64::MAX)?, 42);
        assert_eq!(compute_value_for_shares_amount(u64::MAX, u64::MAX, 42)?, 42);
        assert_eq!(compute_value_for_shares_amount(42, 42, u64::MAX)?, u64::MAX);
        assert_eq!(compute_value_for_shares_amount(u64::MAX, 42, 42)?, u64::MAX);

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(compute_value_for_shares_amount(1, 0, 0).is_err(), true);
        assert_eq!(compute_value_for_shares_amount(1, 0, 10).is_err(), true);
        assert_eq!(compute_value_for_shares_amount(1, 10, 0).is_err(), true);
        Ok(())
    }
}
