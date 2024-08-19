// Unit tests
#[cfg(test)]
mod test_calculate_depositories_sum_value {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_sum_value;

    #[test]
    fn test_no_panic_and_exact_result() -> Result<()> {
        proptest!(|(
            identity_depository_value: u64,
            mercurial_vault_depository_value: u64,
            credix_lp_depository_value: u64,
        )| {
            let depositories_value = [
                identity_depository_value,
                mercurial_vault_depository_value,
                credix_lp_depository_value,
            ];

            // Compute
            let result = calculate_depositories_sum_value(&depositories_value);

            // If the sum does not fit in a u64, we expect failure
            let total_value = u128::from(identity_depository_value)
                + u128::from(mercurial_vault_depository_value)
                + u128::from(credix_lp_depository_value);
            if total_value > u128::from(u64::MAX) {
                prop_assert!(result.is_err());
                return Ok(());
            }

            // Everything else should never panic
            prop_assert!(u128::from(result?) == total_value);
        });
        Ok(())
    }
}
