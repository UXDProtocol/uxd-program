// Unit tests
#[cfg(test)]
mod test_compute_value_for_single_share_ceil {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::compute_value_for_single_share_ceil;

    #[test]
    fn test_correctness() -> Result<()> {
        assert_eq!(compute_value_for_single_share_ceil(1, 10)?, 1);
        assert_eq!(compute_value_for_single_share_ceil(10, 10)?, 1);

        assert_eq!(compute_value_for_single_share_ceil(99, 100)?, 1);
        assert_eq!(compute_value_for_single_share_ceil(100, 100)?, 1);
        assert_eq!(compute_value_for_single_share_ceil(101, 100)?, 2);

        assert_eq!(compute_value_for_single_share_ceil(49, 10)?, 5);
        assert_eq!(compute_value_for_single_share_ceil(50, 10)?, 5);
        assert_eq!(compute_value_for_single_share_ceil(51, 10)?, 6);

        assert_eq!(compute_value_for_single_share_ceil(u64::MAX, 1)?, u64::MAX);
        assert_eq!(compute_value_for_single_share_ceil(u64::MAX, u64::MAX)?, 1);
        assert_eq!(compute_value_for_single_share_ceil(1, u64::MAX)?, 1);

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(compute_value_for_single_share_ceil(0, 0).is_err(), true);
        assert_eq!(compute_value_for_single_share_ceil(0, 1).is_err(), true);
        assert_eq!(
            compute_value_for_single_share_ceil(0, u64::MAX).is_err(),
            true
        );
        assert_eq!(compute_value_for_single_share_ceil(1, 0).is_err(), true);
        assert_eq!(
            compute_value_for_single_share_ceil(u64::MAX, 0).is_err(),
            true
        );
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(total_shares_value: u64, total_shares_supply: u64)| {
            let result = compute_value_for_single_share_ceil(
                total_shares_value,
                total_shares_supply
            );
            // would get MathError in this case
            if total_shares_value == 0 {
                prop_assert!(result.is_err());
                return Ok(());
            }
            if total_shares_supply == 0 {
                prop_assert!(result.is_err());
                return Ok(());
            }
            prop_assert!(result.is_ok());
        });
        Ok(())
    }
}
