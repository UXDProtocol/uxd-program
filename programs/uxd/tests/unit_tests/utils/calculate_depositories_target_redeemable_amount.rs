// Unit tests
#[cfg(test)]
mod test_calculate_depositories_target_redeemable_amount {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_target_redeemable_amount;
    use uxd::utils::is_within_range_inclusive;
    use uxd::BPS_UNIT_CONVERSION;

    fn percent_of_supply(percent: u64, supply: u64) -> u64 {
        supply * percent / 100
    }

    fn percent_to_weight_bps(percent: u16) -> u16 {
        percent * 100
    }

    #[test]
    fn test_with_simplest_case() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;

        // Weights adds up to 100% and are not evenly distributed
        let identity_depository_weight_bps = percent_to_weight_bps(5);
        let mercurial_vault_depository_0_weight_bps = percent_to_weight_bps(10);
        let credix_lp_depository_0_weight_bps = percent_to_weight_bps(85);

        // Each depository can fit at least the whole circulating supply (no overflow possible)
        let identity_depository_hard_cap = percent_of_supply(100, circulating_supply);
        let mercurial_vault_depository_0_hard_cap = percent_of_supply(100, circulating_supply);
        let credix_lp_depository_0_hard_cap = percent_of_supply(100, circulating_supply);

        // Compute
        let depositories_target_redeemable_amount =
            calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                identity_depository_weight_bps,
                mercurial_vault_depository_0_weight_bps,
                credix_lp_depository_0_weight_bps,
                identity_depository_hard_cap.into(),
                mercurial_vault_depository_0_hard_cap.into(),
                credix_lp_depository_0_hard_cap.into(),
            )?;

        // The targets should match the raw weights since we dont have any overflow
        assert_eq!(
            depositories_target_redeemable_amount.identity_depository_target_redeemable_amount,
            percent_of_supply(5, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount
                .mercurial_vault_depository_0_target_redeemable_amount,
            percent_of_supply(10, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount.credix_lp_depository_0_target_redeemable_amount,
            percent_of_supply(85, circulating_supply)
        );

        Ok(())
    }

    #[test]
    fn test_with_overflow_reallocation() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;

        // Weights adds up to 100% and the identity depository receives everything
        let identity_depository_weight_bps = percent_to_weight_bps(100);
        let mercurial_vault_depository_0_weight_bps = percent_to_weight_bps(0);
        let credix_lp_depository_0_weight_bps = percent_to_weight_bps(0);

        // The identity depository is fully overflowing, but the other have enough space
        let identity_depository_hard_cap = percent_of_supply(0, circulating_supply);
        let mercurial_vault_depository_0_hard_cap = percent_of_supply(100, circulating_supply);
        let credix_lp_depository_0_hard_cap = percent_of_supply(100, circulating_supply);

        // Compute
        let depositories_target_redeemable_amount =
            calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                identity_depository_weight_bps,
                mercurial_vault_depository_0_weight_bps,
                credix_lp_depository_0_weight_bps,
                identity_depository_hard_cap.into(),
                mercurial_vault_depository_0_hard_cap.into(),
                credix_lp_depository_0_hard_cap.into(),
            )?;

        // We expect the identity depository to be at the cap, and the overflow to be in other depositories
        assert_eq!(
            depositories_target_redeemable_amount.identity_depository_target_redeemable_amount,
            identity_depository_hard_cap
        );
        assert_eq!(
            depositories_target_redeemable_amount
                .mercurial_vault_depository_0_target_redeemable_amount,
            percent_of_supply(50, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount.credix_lp_depository_0_target_redeemable_amount,
            percent_of_supply(50, circulating_supply)
        );

        Ok(())
    }

    #[test]
    fn test_with_overflow_proportions() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;

        // Weights adds up to 100% and the identity depository receives everything
        let identity_depository_weight_bps = percent_to_weight_bps(100);
        let mercurial_vault_depository_0_weight_bps = percent_to_weight_bps(0);
        let credix_lp_depository_0_weight_bps = percent_to_weight_bps(0);

        // The identity depository is overflowing, mercurial a lot of space and credix has a tiny space
        let identity_depository_hard_cap = percent_of_supply(10, circulating_supply);
        let mercurial_vault_depository_0_hard_cap = percent_of_supply(160, circulating_supply);
        let credix_lp_depository_0_hard_cap = percent_of_supply(20, circulating_supply);

        // Compute
        let depositories_target_redeemable_amount =
            calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                identity_depository_weight_bps,
                mercurial_vault_depository_0_weight_bps,
                credix_lp_depository_0_weight_bps,
                identity_depository_hard_cap.into(),
                mercurial_vault_depository_0_hard_cap.into(),
                credix_lp_depository_0_hard_cap.into(),
            )?;

        // We expect the identity depository to be at the cap,
        // And the overflow to be in other depositories
        // the amounts of overflow should be in the same proportion as the available space
        assert_eq!(
            depositories_target_redeemable_amount.identity_depository_target_redeemable_amount,
            identity_depository_hard_cap
        );
        assert_eq!(
            depositories_target_redeemable_amount
                .mercurial_vault_depository_0_target_redeemable_amount,
            percent_of_supply(80, circulating_supply) // half full
        );
        assert_eq!(
            depositories_target_redeemable_amount.credix_lp_depository_0_target_redeemable_amount,
            percent_of_supply(10, circulating_supply) // half full
        );

        Ok(())
    }

    #[test]
    fn test_with_too_big_supply() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;

        // Weights adds up to 100%, somewhat fair split
        let identity_depository_weight_bps = percent_to_weight_bps(34);
        let mercurial_vault_depository_0_weight_bps = percent_to_weight_bps(33);
        let credix_lp_depository_0_weight_bps = percent_to_weight_bps(33);

        // All depositories are oveflowing, except the identity depository, but the total cannot fit in all depositories
        let identity_depository_hard_cap = percent_of_supply(40, circulating_supply);
        let mercurial_vault_depository_0_hard_cap = percent_of_supply(20, circulating_supply);
        let credix_lp_depository_0_hard_cap = percent_of_supply(15, circulating_supply);

        // Compute
        let depositories_target_redeemable_amount =
            calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                identity_depository_weight_bps,
                mercurial_vault_depository_0_weight_bps,
                credix_lp_depository_0_weight_bps,
                identity_depository_hard_cap.into(),
                mercurial_vault_depository_0_hard_cap.into(),
                credix_lp_depository_0_hard_cap.into(),
            )?;

        // We expect all depositories to become filled up to their caps, which is not sufficient for fitting the whole ciculating supply
        assert_eq!(
            depositories_target_redeemable_amount.identity_depository_target_redeemable_amount,
            identity_depository_hard_cap
        );
        assert_eq!(
            depositories_target_redeemable_amount
                .mercurial_vault_depository_0_target_redeemable_amount,
            mercurial_vault_depository_0_hard_cap
        );
        assert_eq!(
            depositories_target_redeemable_amount.credix_lp_depository_0_target_redeemable_amount,
            credix_lp_depository_0_hard_cap
        );

        Ok(())
    }

    #[test]
    fn test_no_panic_and_always_over_supply() -> Result<()> {
        proptest!(|(
            circulating_supply: u64,
            identity_depository_weight_random: u16,
            mercurial_vault_depository_0_weight_random: u16,
            credix_lp_depository_0_weight_random: u16,
            identity_depository_hard_cap: u64,
            mercurial_vault_depository_0_hard_cap: u64,
            credix_lp_depository_0_hard_cap: u64,
        )| {
            // Check if the hard caps will fit inside of a u64
            // If not, this function will not be expected to work
            let total_hard_caps = u128::from(identity_depository_hard_cap)
                + u128::from(mercurial_vault_depository_0_hard_cap)
                + u128::from(credix_lp_depository_0_hard_cap);
            if total_hard_caps > u128::from(u64::MAX) {
                return Ok(());
            }

            // Enforce for our testing that the weights are always equivalent to 100%
            // To do this, we just generate a bunch of weights based on relative values of the random parameters
            // This is because otherwise, proptest will never generate weights that add up exacly to 100%
            let identity_depository_weight_arbitrary = u64::from(identity_depository_weight_random) + 1;
            let mercurial_vault_depository_0_weight_arbitrary = u64::from(mercurial_vault_depository_0_weight_random) + 1;
            let credix_lp_depository_0_weight_arbitrary = u64::from(credix_lp_depository_0_weight_random) + 1;

            let total_weight_arbitrary = identity_depository_weight_arbitrary + mercurial_vault_depository_0_weight_arbitrary + credix_lp_depository_0_weight_arbitrary;

            let identity_depository_weight_bps = identity_depository_weight_arbitrary * BPS_UNIT_CONVERSION / total_weight_arbitrary;
            let mercurial_vault_depository_0_weight_bps = mercurial_vault_depository_0_weight_arbitrary * BPS_UNIT_CONVERSION / total_weight_arbitrary;
            let credix_lp_depository_0_weight_bps = credix_lp_depository_0_weight_arbitrary * BPS_UNIT_CONVERSION / total_weight_arbitrary;

            // In case of rounding error, we add the rounding errors to identity depository to keep the sum EXACTLY to 100%
            let total_weight_bps = identity_depository_weight_bps + mercurial_vault_depository_0_weight_bps + credix_lp_depository_0_weight_bps;
            let identity_depository_weight_bps = identity_depository_weight_bps + BPS_UNIT_CONVERSION - total_weight_bps;

            // Everything else should never panic
            let depositories_target_redeemable_amount = calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                u16::try_from(identity_depository_weight_bps).unwrap(),
                u16::try_from(mercurial_vault_depository_0_weight_bps).unwrap(),
                u16::try_from(credix_lp_depository_0_weight_bps).unwrap(),
                identity_depository_hard_cap.into(),
                mercurial_vault_depository_0_hard_cap.into(),
                credix_lp_depository_0_hard_cap.into(),
            )?;

            // The sum of all depositories targets should always be either:
            // - equal to the circulating supply
            // - or equal to the sum of all depositories caps

            let maximum_redeemable_amount = std::cmp::min(circulating_supply, u64::try_from(total_hard_caps).unwrap());

            let total_target_redeemable_amount = depositories_target_redeemable_amount.identity_depository_target_redeemable_amount
                + depositories_target_redeemable_amount.mercurial_vault_depository_0_target_redeemable_amount
                + depositories_target_redeemable_amount.credix_lp_depository_0_target_redeemable_amount;

            // Check for equality while allowing 1 of rounding error per depository
            let allowed_precision_loss = 3;

            let value_min = maximum_redeemable_amount;
            let value_max = maximum_redeemable_amount + allowed_precision_loss;

            prop_assert!(is_within_range_inclusive(
                total_target_redeemable_amount,
                value_min,
                value_max
            ));
        });
        Ok(())
    }
}
