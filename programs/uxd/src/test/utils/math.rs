// Unit tests
#[cfg(test)]
mod test_math {
    mod test_checked_i64_to_u64 {
        use crate::utils::math_checked_i64_to_u64;
        use anchor_lang::Result;

        #[test]
        fn test_basics() -> Result<()> {
            assert_eq!(math_checked_i64_to_u64(0)?, 0);
            assert_eq!(math_checked_i64_to_u64(1)?, 1);
            assert_eq!(math_checked_i64_to_u64(9999)?, 9999);
            assert_eq!(math_checked_i64_to_u64(-1).is_err(), true);
            assert_eq!(math_checked_i64_to_u64(-9999).is_err(), true);
            Ok(())
        }
    }

    mod test_compute_delta {
        use crate::utils::math_compute_delta;
        use anchor_lang::Result;

        #[test]
        fn test_decrease() -> Result<()> {
            assert_eq!(math_compute_delta(0, 0)?, 0);
            assert_eq!(math_compute_delta(1, 0)?, -1);
            assert_eq!(math_compute_delta(1_000_000, 0)?, -1_000_000);
            assert_eq!(math_compute_delta(2_000_000, 5)?, -1_999_995);
            assert_eq!(math_compute_delta(5_000_000, 4_000_000)?, -1_000_000);
            Ok(())
        }

        #[test]
        fn test_increase() -> Result<()> {
            assert_eq!(math_compute_delta(0, 0)?, 0);
            assert_eq!(math_compute_delta(0, 1)?, 1);
            assert_eq!(math_compute_delta(0, 1_000_000)?, 1_000_000);
            assert_eq!(math_compute_delta(5, 2_000_000)?, 1_999_995);
            assert_eq!(math_compute_delta(4_000_000, 5_000_000)?, 1_000_000);
            Ok(())
        }
    }

    mod test_is_equal_accounting_for_precision_loss {
        use crate::utils::math_is_equal_with_precision_loss;
        use anchor_lang::Result;

        #[test]
        fn test_equality() -> Result<()> {
            assert_eq!(math_is_equal_with_precision_loss(0, 0, 0)?, true);
            assert_eq!(math_is_equal_with_precision_loss(1, 1, 0)?, true);
            assert_eq!(math_is_equal_with_precision_loss(1, 0, 1)?, true);
            assert_eq!(math_is_equal_with_precision_loss(1000, 1000, 0)?, true);
            assert_eq!(math_is_equal_with_precision_loss(1000, 999, 1)?, true);
            assert_eq!(math_is_equal_with_precision_loss(1000, 0, 1000)?, true);
            assert_eq!(math_is_equal_with_precision_loss(2000, 1000, 1000)?, true);
            Ok(())
        }

        #[test]
        fn test_inequality() -> Result<()> {
            assert_eq!(math_is_equal_with_precision_loss(0, 1, 0)?, false);
            assert_eq!(math_is_equal_with_precision_loss(1, 0, 0)?, false);
            assert_eq!(math_is_equal_with_precision_loss(1000, 1001, 0)?, false);
            assert_eq!(math_is_equal_with_precision_loss(1000, 999, 0)?, false);
            assert_eq!(math_is_equal_with_precision_loss(1000, 1001, 1)?, false);
            assert_eq!(math_is_equal_with_precision_loss(1000, 998, 1)?, false);
            assert_eq!(math_is_equal_with_precision_loss(2000, 2001, 1000)?, false);
            assert_eq!(math_is_equal_with_precision_loss(2000, 999, 1000)?, false);
            Ok(())
        }
    }
}
