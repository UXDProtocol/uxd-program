// Unit tests
#[cfg(test)]
mod test_compute_shares_amount_for_value_floor {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::compute_shares_amount_for_value_floor;

    #[test]
    fn test_correctness() -> Result<()> {
        // Test regular cases and check correct rounding
        assert_eq!(compute_shares_amount_for_value_floor(0, 0, 0)?, 0);
        assert_eq!(compute_shares_amount_for_value_floor(0, 10, 0)?, 0);
        assert_eq!(compute_shares_amount_for_value_floor(0, 0, 10)?, 0);

        assert_eq!(compute_shares_amount_for_value_floor(0, 100, 1)?, 0);
        assert_eq!(compute_shares_amount_for_value_floor(1, 100, 1)?, 100);

        assert_eq!(compute_shares_amount_for_value_floor(0, 100, 100)?, 0);
        assert_eq!(compute_shares_amount_for_value_floor(1, 100, 100)?, 1);
        assert_eq!(compute_shares_amount_for_value_floor(49, 100, 100)?, 49);
        assert_eq!(compute_shares_amount_for_value_floor(100, 100, 100)?, 100);

        assert_eq!(compute_shares_amount_for_value_floor(0, 10, 100)?, 0);
        assert_eq!(compute_shares_amount_for_value_floor(1, 10, 100)?, 0);
        assert_eq!(compute_shares_amount_for_value_floor(49, 10, 100)?, 4);
        assert_eq!(compute_shares_amount_for_value_floor(100, 10, 100)?, 10);

        assert_eq!(compute_shares_amount_for_value_floor(0, 100, 10)?, 0);
        assert_eq!(compute_shares_amount_for_value_floor(1, 100, 10)?, 10);
        assert_eq!(compute_shares_amount_for_value_floor(4, 100, 10)?, 40);
        assert_eq!(compute_shares_amount_for_value_floor(10, 100, 10)?, 100);

        // Test large amounts against u64 overflow
        assert_eq!(
            compute_shares_amount_for_value_floor(
                1_000_000_000_000_000_000,
                2_000_000_000_000_000_000,
                4_000_000_000_000_000_000
            )?,
            500_000_000_000_000_000
        );
        assert_eq!(
            compute_shares_amount_for_value_floor(u64::MAX, u64::MAX, u64::MAX)?,
            u64::MAX
        );
        assert_eq!(
            compute_shares_amount_for_value_floor(42, u64::MAX, u64::MAX)?,
            42
        );
        assert_eq!(
            compute_shares_amount_for_value_floor(u64::MAX, 42, u64::MAX)?,
            42
        );
        assert_eq!(
            compute_shares_amount_for_value_floor(u64::MAX, 42, 42)?,
            u64::MAX
        );
        assert_eq!(
            compute_shares_amount_for_value_floor(42, u64::MAX, 42)?,
            u64::MAX
        );

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert!(compute_shares_amount_for_value_floor(1, 0, 0).is_err());
        assert!(compute_shares_amount_for_value_floor(1, 10, 0).is_err());
        assert!(compute_shares_amount_for_value_floor(1, 0, 10).is_err());
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(value: u64, total_shares_supply: u64, total_shares_value: u64)| {
            let result = compute_shares_amount_for_value_floor(
                value,
                total_shares_supply,
                total_shares_value
            );
            // Some basic cases are supposed to fail
            if total_shares_supply == 0 {
                prop_assert!(result.is_err());
                return Ok(());
            }
            if total_shares_value == 0 {
                prop_assert!(result.is_err());
                return Ok(());
            }
            // u64 is the limit for token amount in solana, we fail if we overflow that
            let value: u128 = value.into();
            let total_shares_supply: u128 = total_shares_supply.into();
            let total_shares_value: u128 = total_shares_value.into();
            let max_supply: u128 = u64::MAX.into();
            if value * total_shares_supply / total_shares_value > max_supply {
                prop_assert!(result.is_err());
                return Ok(());
            }
            // We should not fail in any other case
            prop_assert!(result.is_ok());
        });
        Ok(())
    }
}
