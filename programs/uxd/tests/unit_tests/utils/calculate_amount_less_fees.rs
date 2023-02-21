// Unit tests
#[cfg(test)]
mod test_calculate_amount_less_fees {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_amount_less_fees;

    #[test]
    fn test_zero_fees() -> Result<()> {
        assert_eq!(calculate_amount_less_fees(0, 0)?, 0);
        assert_eq!(calculate_amount_less_fees(1, 0)?, 1);
        assert_eq!(calculate_amount_less_fees(1_000_000, 0)?, 1_000_000);
        assert_eq!(calculate_amount_less_fees(2_000_000, 0)?, 2_000_000);
        assert_eq!(calculate_amount_less_fees(5_000_000, 0)?, 5_000_000);

        Ok(())
    }

    #[test]
    fn test_five_bps_fees() -> Result<()> {
        assert_eq!(calculate_amount_less_fees(0, 5)?, 0);
        assert_eq!(calculate_amount_less_fees(1, 5)?, 0);
        assert_eq!(calculate_amount_less_fees(1_000_000, 5)?, 999_500);
        assert_eq!(calculate_amount_less_fees(2_000_000, 5)?, 1_999_000);
        assert_eq!(calculate_amount_less_fees(5_000_000, 5)?, 4_997_500);

        Ok(())
    }

    #[test]
    fn test_maximum_bps_fees() -> Result<()> {
        assert_eq!(calculate_amount_less_fees(0, 255)?, 0);
        assert_eq!(calculate_amount_less_fees(1, 255)?, 0);
        assert_eq!(calculate_amount_less_fees(1_000_000, 255)?, 974_500);
        assert_eq!(calculate_amount_less_fees(2_000_000, 255)?, 1_949_000);
        assert_eq!(calculate_amount_less_fees(5_000_000, 255)?, 4_872_500);

        Ok(())
    }

    #[test]
    fn test_no_panic() -> Result<()> {
        proptest!(|(amount: u64, bps: u8)| {
            prop_assert!(calculate_amount_less_fees(amount, bps).is_ok());
        });
        Ok(())
    }

    #[test]
    fn test_no_increase() -> Result<()> {
        proptest!(|(amount: u64, bps: u8)| {
            prop_assert!(calculate_amount_less_fees(amount, bps)?<= amount);
        });
        Ok(())
    }
}
