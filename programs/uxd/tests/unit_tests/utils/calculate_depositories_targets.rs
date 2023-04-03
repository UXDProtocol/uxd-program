// Unit tests
#[cfg(test)]
mod test_calculate_depositories_targets {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::{utils::calculate_depositories_targets, BPS_UNIT_CONVERSION};

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
        let depositories_targets = calculate_depositories_targets(
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
            depositories_targets.identity_depository_target_amount,
            percent_of_supply(5, circulating_supply)
        );
        assert_eq!(
            depositories_targets.mercurial_vault_depository_0_target_amount,
            percent_of_supply(10, circulating_supply)
        );
        assert_eq!(
            depositories_targets.credix_lp_depository_0_target_amount,
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
        let depositories_targets = calculate_depositories_targets(
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
            depositories_targets.identity_depository_target_amount,
            identity_depository_hard_cap
        );
        assert_eq!(
            depositories_targets.mercurial_vault_depository_0_target_amount,
            percent_of_supply(50, circulating_supply)
        );
        assert_eq!(
            depositories_targets.credix_lp_depository_0_target_amount,
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
        let depositories_targets = calculate_depositories_targets(
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
            depositories_targets.identity_depository_target_amount,
            identity_depository_hard_cap
        );
        assert_eq!(
            depositories_targets.mercurial_vault_depository_0_target_amount,
            percent_of_supply(80, circulating_supply) // half full
        );
        assert_eq!(
            depositories_targets.credix_lp_depository_0_target_amount,
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

        // All depositories are oveflowing
        let identity_depository_hard_cap = percent_of_supply(10, circulating_supply);
        let mercurial_vault_depository_0_hard_cap = percent_of_supply(20, circulating_supply);
        let credix_lp_depository_0_hard_cap = percent_of_supply(15, circulating_supply);

        // Compute
        let depositories_targets = calculate_depositories_targets(
            circulating_supply.into(),
            identity_depository_weight_bps,
            mercurial_vault_depository_0_weight_bps,
            credix_lp_depository_0_weight_bps,
            identity_depository_hard_cap.into(),
            mercurial_vault_depository_0_hard_cap.into(),
            credix_lp_depository_0_hard_cap.into(),
        )?;

        // We expect all depositories to become filled up to their caps
        assert_eq!(
            depositories_targets.identity_depository_target_amount,
            identity_depository_hard_cap
        );
        assert_eq!(
            depositories_targets.mercurial_vault_depository_0_target_amount,
            mercurial_vault_depository_0_hard_cap
        );
        assert_eq!(
            depositories_targets.credix_lp_depository_0_target_amount,
            credix_lp_depository_0_hard_cap
        );

        Ok(())
    }

    #[test]
    fn test_with_invalid_weight() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;

        // Weights are invalid, does not add up to 100%
        let identity_depository_weight_bps = percent_to_weight_bps(0);
        let mercurial_vault_depository_0_weight_bps = percent_to_weight_bps(5);
        let credix_lp_depository_0_weight_bps = percent_to_weight_bps(10);

        // All depositories are oveflowing
        let identity_depository_hard_cap = percent_of_supply(100, circulating_supply);
        let mercurial_vault_depository_0_hard_cap = percent_of_supply(100, circulating_supply);
        let credix_lp_depository_0_hard_cap = percent_of_supply(100, circulating_supply);

        // Compute
        let depositories_targets = calculate_depositories_targets(
            circulating_supply.into(),
            identity_depository_weight_bps,
            mercurial_vault_depository_0_weight_bps,
            credix_lp_depository_0_weight_bps,
            identity_depository_hard_cap.into(),
            mercurial_vault_depository_0_hard_cap.into(),
            credix_lp_depository_0_hard_cap.into(),
        )?;

        // We expect depositories to obey their weight anyway as there is enough space available
        assert_eq!(
            depositories_targets.identity_depository_target_amount,
            percent_of_supply(0, circulating_supply)
        );
        assert_eq!(
            depositories_targets.mercurial_vault_depository_0_target_amount,
            percent_of_supply(5, circulating_supply)
        );
        assert_eq!(
            depositories_targets.credix_lp_depository_0_target_amount,
            percent_of_supply(10, circulating_supply)
        );

        Ok(())
    }

    #[test]
    fn test_no_panic_and_no_over_cap() -> Result<()> {
        proptest!(|(
            circulating_supply: u64,
            identity_depository_weight_bps: u16,
            mercurial_vault_depository_0_weight_bps: u16,
            credix_lp_depository_0_weight_bps: u16,
            identity_depository_hard_cap: u64,
            mercurial_vault_depository_0_hard_cap: u64,
            credix_lp_depository_0_hard_cap: u64,
        )| {
            // Compute
            let result = calculate_depositories_targets(
                circulating_supply.into(),
                identity_depository_weight_bps,
                mercurial_vault_depository_0_weight_bps,
                credix_lp_depository_0_weight_bps,
                identity_depository_hard_cap.into(),
                mercurial_vault_depository_0_hard_cap.into(),
                credix_lp_depository_0_hard_cap.into(),
            );

            // We dont check the case where the total weights exceed 100%
            // As this could lead to overflows if we specify insane values
            let total_weight_bps = u64::from(identity_depository_weight_bps)
                + u64::from(mercurial_vault_depository_0_weight_bps)
                + u64::from(credix_lp_depository_0_weight_bps);
            if total_weight_bps > BPS_UNIT_CONVERSION {
                return Ok(());
            }

            // Check if the hard caps will fit inside of a u64
            // If not, this function will not be expected to work
            let total_hard_caps = u128::from(identity_depository_hard_cap)
                + u128::from(mercurial_vault_depository_0_hard_cap)
                + u128::from(credix_lp_depository_0_hard_cap);
            if total_hard_caps > u128::from(u64::MAX) {
                return Ok(());
            }

            // Everything else should never panic
            let depositories_targets = result?;

            // The sum of all depositories targets should always be either:
            // - either equal to the circulating supply (minus precision loss)
            // - or equal to the sum of all depositories caps

            let total_target_amount = depositories_targets.identity_depository_target_amount
                + depositories_targets.mercurial_vault_depository_0_target_amount
                + depositories_targets.credix_lp_depository_0_target_amount;

            prop_assert!(total_target_amount <= circulating_supply);

            if total_hard_caps < u128::from(circulating_supply) {
                prop_assert_eq!(total_hard_caps, u128::from(total_target_amount));
            }
        });
        Ok(())
    }
}
