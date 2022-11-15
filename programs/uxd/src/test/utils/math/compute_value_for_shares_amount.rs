// Unit tests
#[cfg(test)]
mod test_compute_value_for_shares_amount {
    use crate::utils::compute_value_for_shares_amount;
    use anchor_lang::Result;

    #[test]
    fn test_correctness() -> Result<()> {
        assert_eq!(compute_value_for_shares_amount(0, 1000, 1000)?, 0);
        assert_eq!(compute_value_for_shares_amount(1, 1000, 1000)?, 1);
        assert_eq!(compute_value_for_shares_amount(1000, 1000, 1000)?, 1000);

        assert_eq!(compute_value_for_shares_amount(0, 1000, 1)?, 0);
        assert_eq!(compute_value_for_shares_amount(999, 1000, 1)?, 0);
        assert_eq!(compute_value_for_shares_amount(1000, 1000, 1)?, 1);

        assert_eq!(compute_value_for_shares_amount(0, 10, 1000)?, 0);
        assert_eq!(compute_value_for_shares_amount(9, 10, 1000)?, 900);
        assert_eq!(compute_value_for_shares_amount(10, 10, 1000)?, 1000);

        assert_eq!(compute_value_for_shares_amount(0, 1000, 10)?, 0);
        assert_eq!(compute_value_for_shares_amount(499, 1000, 10)?, 4);
        assert_eq!(compute_value_for_shares_amount(500, 1000, 10)?, 5);
        assert_eq!(compute_value_for_shares_amount(501, 1000, 10)?, 5);
        assert_eq!(compute_value_for_shares_amount(1000, 1000, 10)?, 10);
        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(compute_value_for_shares_amount(1, 0, 0).is_err(), true);
        assert_eq!(compute_value_for_shares_amount(11, 10, 10).is_err(), true);
        assert_eq!(compute_value_for_shares_amount(10, 1, 10).is_err(), true);
        Ok(())
    }
}
