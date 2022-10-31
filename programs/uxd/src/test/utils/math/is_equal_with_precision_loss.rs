// Unit tests
#[cfg(test)]
mod test_is_equal_with_precision_loss {
    use crate::utils::is_equal_with_precision_loss;
    use anchor_lang::Result;

    #[test]
    fn test_equality() -> Result<()> {
        assert_eq!(is_equal_with_precision_loss(0, 0, 0)?, true);
        assert_eq!(is_equal_with_precision_loss(1, 1, 0)?, true);
        assert_eq!(is_equal_with_precision_loss(1, 0, 1)?, true);
        assert_eq!(is_equal_with_precision_loss(1000, 1000, 0)?, true);
        assert_eq!(is_equal_with_precision_loss(1000, 999, 1)?, true);
        assert_eq!(is_equal_with_precision_loss(1000, 0, 1000)?, true);
        assert_eq!(is_equal_with_precision_loss(2000, 1000, 1000)?, true);
        Ok(())
    }

    #[test]
    fn test_inequality() -> Result<()> {
        assert_eq!(is_equal_with_precision_loss(0, 1, 0)?, false);
        assert_eq!(is_equal_with_precision_loss(1, 0, 0)?, false);
        assert_eq!(is_equal_with_precision_loss(1000, 1001, 0)?, false);
        assert_eq!(is_equal_with_precision_loss(1000, 999, 0)?, false);
        assert_eq!(is_equal_with_precision_loss(1000, 1001, 1)?, false);
        assert_eq!(is_equal_with_precision_loss(1000, 998, 1)?, false);
        assert_eq!(is_equal_with_precision_loss(2000, 2001, 1000)?, false);
        assert_eq!(is_equal_with_precision_loss(2000, 999, 1000)?, false);
        Ok(())
    }
}
