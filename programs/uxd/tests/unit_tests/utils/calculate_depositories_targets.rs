// Unit tests
#[cfg(test)]
mod test_calculate_depositories_targets {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_targets;

    fn percent_of_supply(percent: u64, supply: u64) -> u64 {
        return supply * percent / 100;
    }

    fn percent_to_weight_bps(percent: u16) -> u16 {
        return percent * 100;
    }

    #[test]
    fn test_with_simplest_case() -> Result<()> {
        let circulating_supply = 10000;

        // Weights adds up to 100% and are not evenly distributed
        let identity_depository_weight_bps = percent_to_weight_bps(5);
        let mercurial_vault_depository_0_weight_bps = percent_to_weight_bps(10);
        let credix_lp_depository_0_weight_bps = percent_to_weight_bps(85);

        // Each depository can fit at least the whole circulating supply (no overflow anywhere)
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
    fn test_with_overflow() -> Result<()> {
        let circulating_supply = 10000;

        // Weights adds up to 100% and the identity depository receives everything
        let identity_depository_weight_bps = percent_to_weight_bps(100);
        let mercurial_vault_depository_0_weight_bps = percent_to_weight_bps(0);
        let credix_lp_depository_0_weight_bps = percent_to_weight_bps(0);

        // The identity depository is fully overflowing, but the other are nots
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

    /*
    #[test]
    fn test_no_panic() -> Result<()> {
        proptest!(|(amount: u64, bps: u8)| {
            prop_assert!(calculate_depositories_targets(amount, bps).is_ok());
        });
        Ok(())
    }

    #[test]
    fn test_no_increase() -> Result<()> {
        proptest!(|(amount: u64, bps: u8)| {
            prop_assert!(calculate_depositories_targets(amount, bps)? <= amount);
        });
        Ok(())
    }
     */
}
