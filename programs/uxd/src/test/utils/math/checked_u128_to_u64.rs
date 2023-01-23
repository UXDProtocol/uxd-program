// Unit tests
#[cfg(test)]
mod test_checked_u128_to_u64 {
    use crate::utils::checked_u128_to_u64;
    use anchor_lang::Result;
    use proptest::prelude::*;

    #[test]
    fn test_correctness() -> Result<()> {
        assert_eq!(checked_u128_to_u64(0)?, 0);
        assert_eq!(checked_u128_to_u64(1)?, 1);
        assert_eq!(checked_u128_to_u64(9999)?, 9999);
        assert_eq!(checked_u128_to_u64(1_000_000_000_000)?, 1_000_000_000_000);
        assert_eq!(checked_u128_to_u64(u64::MAX.into())?, u64::MAX);
        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(checked_u128_to_u64(u128::from(u64::MAX) + 1).is_err(), true);
        assert_eq!(checked_u128_to_u64(u128::MAX).is_err(), true);
        Ok(())
    }

    #[test]
    fn test_panic_cases() -> Result<()> {
        proptest!(|(amount: u128)| {
            let result = checked_u128_to_u64(amount);
            // Some cases are supposed to fail
            if amount > u128::from(u64::MAX) {
                prop_assert!(result.is_err());
                return Ok(());
            }
            // In all other cases, we should not panic
            prop_assert!(u128::from(result?) == amount);
        });
        Ok(())
    }
}
