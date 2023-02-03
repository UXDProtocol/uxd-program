// Unit tests
#[cfg(test)]
mod test_compute_amount_after_change {
    use crate::utils::compute_amount_after_change;
    use anchor_lang::Result;
    use proptest::prelude::*;

    #[test]
    fn test_correctness() -> Result<()> {
        let i128_max_as_u128: u128 = u128::try_from(i128::MAX).unwrap();

        // Test flatline
        assert_eq!(compute_amount_after_change(0, 0)?, 0);
        assert_eq!(compute_amount_after_change(u128::MAX, 0)?, u128::MAX);

        // Test increases
        assert_eq!(compute_amount_after_change(0, 10)?, 10);
        assert_eq!(compute_amount_after_change(100, 10)?, 110);

        assert_eq!(compute_amount_after_change(0, i128::MAX)?, i128_max_as_u128);
        assert_eq!(
            compute_amount_after_change(i128_max_as_u128, i128::MAX)?,
            i128_max_as_u128 * 2
        );

        // Test decreases
        assert_eq!(compute_amount_after_change(10, -10)?, 0);
        assert_eq!(compute_amount_after_change(100, -10)?, 90);

        assert_eq!(
            compute_amount_after_change(i128_max_as_u128, i128::MAX)?,
            i128_max_as_u128 + i128_max_as_u128
        );
        assert_eq!(
            compute_amount_after_change(i128_max_as_u128, -i128::MAX)?,
            0
        );
        assert_eq!(
            compute_amount_after_change(i128_max_as_u128 + 1, i128::MIN)?,
            0
        );

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(compute_amount_after_change(0, -1).is_err(), true);
        assert_eq!(compute_amount_after_change(u128::MAX, 1).is_err(), true);
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(amount_before_change: u128, change: i128)| {
            let result = compute_amount_after_change(
                amount_before_change,
                change
            );

            // When its an increase
            if change >= 0 {
                let increase: u128 = u128::try_from(change).unwrap();
                // If the amounts are too big, overflow is expected
                let addition = amount_before_change.checked_add(increase);
                if addition.is_none() {
                    prop_assert!(result.is_err());
                    return Ok(());
                }
                // If the amounts are not too big, the result should never fail
                prop_assert!(result.unwrap() == addition.unwrap());
                return Ok(());
            }

            // When its a decrease
            if change < 0 {
                let decrease: u128;
                if change == i128::MIN {
                    decrease = u128::try_from(u128::MAX).unwrap() + 1;
                } else {
                    decrease = u128::try_from(-change).unwrap();
                }
                // If the decrease is bigger than the amount, expect failure
                if decrease > amount_before_change {
                    prop_assert!(result.is_err());
                    return Ok(());
                }
                // Otherwise we should never fail
                prop_assert!(result.unwrap() == amount_before_change - decrease);
                return Ok(());
            }
        });
        Ok(())
    }
}
