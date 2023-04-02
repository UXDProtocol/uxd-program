// Unit tests
#[cfg(test)]
mod test_calculate_depositories_targets {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_targets;

    #[test]
    fn test_simple() -> Result<()> {
        let depositories_targets = calculate_depositories_targets(0, 5);

        Ok(())
    }

    #[test]
    fn test_five_bps_fees() -> Result<()> {
        assert_eq!(calculate_depositories_targets(0, 5)?, 0);
        assert_eq!(calculate_depositories_targets(1, 5)?, 0);
        assert_eq!(calculate_depositories_targets(1_000_000, 5)?, 999_500);
        assert_eq!(calculate_depositories_targets(2_000_000, 5)?, 1_999_000);
        assert_eq!(calculate_depositories_targets(5_000_000, 5)?, 4_997_500);

        Ok(())
    }

    #[test]
    fn test_no_panic() -> Result<()> {
        proptest!(|(amount: u64, bps: u8)| {
            prop_assert!(calculate_depositories_targets(amount, bps).is_ok());
        });
        Ok(())
    }

    #[test]
    fn test_no_increase() -> Result<()> {
        proptest!(|(amount: u64, bps: u8)| {
            prop_assert!(calculate_depositories_targets(amount, bps)? <= amount);
        });
        Ok(())
    }
}
