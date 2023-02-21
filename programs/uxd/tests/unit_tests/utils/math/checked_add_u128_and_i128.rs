// Unit tests
#[cfg(test)]
mod test_compute_amount_after_change {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::checked_add_u128_and_i128;

    #[test]
    fn test_correctness() -> Result<()> {
        let i128_max_as_u128: u128 = u128::try_from(i128::MAX).unwrap();

        // Test flatline
        assert_eq!(checked_add_u128_and_i128(0, 0)?, 0);
        assert_eq!(checked_add_u128_and_i128(u128::MAX, 0)?, u128::MAX);

        // Test increases
        assert_eq!(checked_add_u128_and_i128(0, 10)?, 10);
        assert_eq!(checked_add_u128_and_i128(100, 10)?, 110);

        assert_eq!(checked_add_u128_and_i128(0, i128::MAX)?, i128_max_as_u128);
        assert_eq!(
            checked_add_u128_and_i128(i128_max_as_u128, i128::MAX)?,
            i128_max_as_u128 * 2
        );

        // Test decreases
        assert_eq!(checked_add_u128_and_i128(10, -10)?, 0);
        assert_eq!(checked_add_u128_and_i128(100, -10)?, 90);

        assert_eq!(
            checked_add_u128_and_i128(i128_max_as_u128, i128::MAX)?,
            i128_max_as_u128 + i128_max_as_u128
        );
        assert_eq!(checked_add_u128_and_i128(i128_max_as_u128, -i128::MAX)?, 0);
        assert_eq!(
            checked_add_u128_and_i128(i128_max_as_u128 + 1, i128::MIN)?,
            0
        );

        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(checked_add_u128_and_i128(0, -1).is_err(), true);
        assert_eq!(checked_add_u128_and_i128(u128::MAX, 1).is_err(), true);
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(value_before: u128, change_delta: i128)| {
            // Before we check the output, compute the result
            let result = checked_add_u128_and_i128(
                value_before,
                change_delta
            );
            // When its an increase
            if change_delta >= 0 {
                let increase: u128 = u128::try_from(change_delta).unwrap();
                // If the amounts are too big, overflow is expected
                let addition = value_before.checked_add(increase);
                if addition.is_none() {
                    prop_assert!(result.is_err());
                    return Ok(());
                }
                // If the amounts are not too big, the result should never fail
                prop_assert!(result.unwrap() == addition.unwrap());
                return Ok(());
            }
            // Otherwise When its a decrease
            if change_deltaContext< 0 {
                let decrease: u128;
                if change_delta == i128::MIN {
                    decrease = u128::try_from(u128::MAX).unwrap() + 1;
                } else {
                    decrease = u128::try_from(-change_delta).unwrap();
                }
                // If the decrease is bigger than the amount, expect failure
                if decrease > value_before {
                    prop_assert!(result.is_err());
                    return Ok(());
                }
                // Otherwise we should never fail
                prop_assert!(result.unwrap() == value_before - decrease);
                return Ok(());
            }
        });
        Ok(())
    }
}
