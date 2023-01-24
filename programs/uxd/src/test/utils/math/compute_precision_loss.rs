// Unit tests
#[cfg(test)]
mod test_compute_precision_loss {
    use crate::utils::compute_precision_loss;
    use anchor_lang::Result;
    use proptest::prelude::*;

    #[test]
    fn test_correctness() -> Result<()> {
        assert_eq!(compute_precision_loss(0, 0)?, 0);
        assert_eq!(compute_precision_loss(99, 0)?, 0);

        assert_eq!(compute_precision_loss(1, 10)?, 1);
        assert_eq!(compute_precision_loss(10, 10)?, 1);

        assert_eq!(compute_precision_loss(99, 100)?, 1);
        assert_eq!(compute_precision_loss(100, 100)?, 1);
        assert_eq!(compute_precision_loss(101, 100)?, 2);

        assert_eq!(compute_precision_loss(49, 10)?, 5);
        assert_eq!(compute_precision_loss(50, 10)?, 5);
        assert_eq!(compute_precision_loss(51, 10)?, 6);

        assert_eq!(compute_precision_loss(u64::MAX, 1)?, u64::MAX);
        assert_eq!(compute_precision_loss(u64::MAX, u64::MAX)?, 1);
        assert_eq!(compute_precision_loss(1, u64::MAX)?, 1);

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(compute_precision_loss(0, 1).is_err(), true);
        assert_eq!(compute_precision_loss(0, u64::MAX).is_err(), true);
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(total_shares_amount: u64, total_shares_value: u64)| {
            let result = compute_precision_loss(
                total_shares_amount,
                total_shares_value
            );
            // would get MathError in this case
            if total_shares_amount == 0 {
                prop_assert!(result.is_err());
                return Ok(());
            }
            prop_assert!(result.is_ok());
        });
        Ok(())
    }
}
