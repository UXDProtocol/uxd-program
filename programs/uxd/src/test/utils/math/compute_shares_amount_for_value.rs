// Unit tests
#[cfg(test)]
mod test_compute_shares_amount_for_value {
    use crate::utils::compute_shares_amount_for_value;
    use anchor_lang::Result;

    #[test]
    fn test_correctness() -> Result<()> {
        assert_eq!(compute_shares_amount_for_value(0, 0, 0)?, 0);
        assert_eq!(compute_shares_amount_for_value(0, 10, 0)?, 0);
        assert_eq!(compute_shares_amount_for_value(0, 0, 10)?, 0);

        assert_eq!(compute_shares_amount_for_value(0, 100, 1)?, 0);
        assert_eq!(compute_shares_amount_for_value(1, 100, 1)?, 100);

        assert_eq!(compute_shares_amount_for_value(0, 100, 100)?, 0);
        assert_eq!(compute_shares_amount_for_value(1, 100, 100)?, 1);
        assert_eq!(compute_shares_amount_for_value(49, 100, 100)?, 49);
        assert_eq!(compute_shares_amount_for_value(100, 100, 100)?, 100);

        assert_eq!(compute_shares_amount_for_value(0, 10, 100)?, 0);
        assert_eq!(compute_shares_amount_for_value(1, 10, 100)?, 0);
        assert_eq!(compute_shares_amount_for_value(49, 10, 100)?, 4);
        assert_eq!(compute_shares_amount_for_value(100, 10, 100)?, 10);

        assert_eq!(compute_shares_amount_for_value(0, 100, 10)?, 0);
        assert_eq!(compute_shares_amount_for_value(1, 100, 10)?, 10);
        assert_eq!(compute_shares_amount_for_value(4, 100, 10)?, 40);
        assert_eq!(compute_shares_amount_for_value(10, 100, 10)?, 100);
        Ok(())
    }

    #[test]
    fn test_incorrectness() -> Result<()> {
        assert_eq!(compute_shares_amount_for_value(1, 0, 0).is_err(), true);
        assert_eq!(compute_shares_amount_for_value(1, 100, 0).is_err(), true);
        assert_eq!(compute_shares_amount_for_value(11, 0, 10).is_err(), true);
        assert_eq!(compute_shares_amount_for_value(11, 100, 10).is_err(), true);
        Ok(())
    }
}
