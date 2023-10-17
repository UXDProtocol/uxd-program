// Unit tests
#[cfg(test)]
mod test_calculate_depositories_target_redeemable_amount {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_target_redeemable_amount;
    use uxd::utils::is_within_range_inclusive;
    use uxd::utils::DepositoryInfoForTargetRedeemableAmount;
    use uxd::BPS_POWER;
    use uxd::ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX;
    use uxd::ROUTER_CREDIX_LP_DEPOSITORY_INDEX;
    use uxd::ROUTER_DEPOSITORIES_COUNT;
    use uxd::ROUTER_IDENTITY_DEPOSITORY_INDEX;
    use uxd::ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX;

    fn percent_of_supply(percent: u64, supply: u64) -> u64 {
        supply * percent / 100
    }

    fn percent_to_weight_bps(percent: u16) -> u16 {
        percent * 100
    }

    #[test]
    fn test_with_simplest_case() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;
        // Compute
        let depositories_target_redeemable_amount =
            calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                // Weights adds up to 100% and are not evenly distributed
                // Each depository can fit at least the whole circulating supply (no overflow possible)
                &vec![
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(5),
                        redeemable_amount_under_management_cap: circulating_supply.into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(10),
                        redeemable_amount_under_management_cap: circulating_supply.into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(65),
                        redeemable_amount_under_management_cap: circulating_supply.into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(20),
                        redeemable_amount_under_management_cap: circulating_supply.into(),
                    },
                ],
            )?;
        // The targets should match the raw weights since we dont have any overflow
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            percent_of_supply(5, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
            percent_of_supply(10, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            percent_of_supply(65, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX],
            percent_of_supply(20, circulating_supply)
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_overflow_reallocation() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;
        // Compute
        let depositories_target_redeemable_amount =
            calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                &vec![
                    // Weights adds up to 100% and the identity depository receives most of the weight
                    // The identity depository is fully overflowing, but the others have enough space (with various proportions)
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(90),
                        redeemable_amount_under_management_cap: 0,
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(0),
                        redeemable_amount_under_management_cap: circulating_supply.into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(0),
                        redeemable_amount_under_management_cap: circulating_supply.into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(10),
                        redeemable_amount_under_management_cap: circulating_supply.into(),
                    },
                ],
            )?;
        // We expect the identity depository to be at the cap, and the overflow to be pushed to other depositories
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            0
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
            percent_of_supply(30, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            percent_of_supply(30, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            percent_of_supply(40, circulating_supply)
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_overflow_proportions() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;
        // Compute
        let depositories_target_redeemable_amount =
            calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                &vec![
                    // Weights adds up to 100% and the identity depository receives most of the weight
                    // The identity depository is overflowing
                    // mercurial a lot of space
                    // credix has a tiny space
                    // alloyx has no space at all
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(90),
                        redeemable_amount_under_management_cap: percent_of_supply(
                            10,
                            circulating_supply,
                        )
                        .into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(0),
                        redeemable_amount_under_management_cap: percent_of_supply(
                            140,
                            circulating_supply,
                        )
                        .into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(0),
                        redeemable_amount_under_management_cap: percent_of_supply(
                            20,
                            circulating_supply,
                        )
                        .into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(10),
                        redeemable_amount_under_management_cap: percent_of_supply(
                            10,
                            circulating_supply,
                        )
                        .into(),
                    },
                ],
            )?;
        // We expect the identity depository to be at the cap,
        // And the overflow to be in other depositories
        // the amounts of overflow should be in the same proportion as the available space
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            percent_of_supply(10, circulating_supply) // full
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
            percent_of_supply(70, circulating_supply) // half full
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            percent_of_supply(10, circulating_supply) // half full
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX],
            percent_of_supply(10, circulating_supply) // full
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_too_big_supply() -> Result<()> {
        let circulating_supply = 1_000_000_000_000;
        // Compute
        let depositories_target_redeemable_amount =
            calculate_depositories_target_redeemable_amount(
                circulating_supply.into(),
                &vec![
                    // Weights adds up to 100%, somewhat fair split
                    // All depositories are oveflowing, except the identity depository, but the total cannot fit in all depositories
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(25),
                        redeemable_amount_under_management_cap: percent_of_supply(
                            40,
                            circulating_supply,
                        )
                        .into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(25),
                        redeemable_amount_under_management_cap: percent_of_supply(
                            20,
                            circulating_supply,
                        )
                        .into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(25),
                        redeemable_amount_under_management_cap: percent_of_supply(
                            10,
                            circulating_supply,
                        )
                        .into(),
                    },
                    DepositoryInfoForTargetRedeemableAmount {
                        weight_bps: percent_to_weight_bps(25),
                        redeemable_amount_under_management_cap: percent_of_supply(
                            5,
                            circulating_supply,
                        )
                        .into(),
                    },
                ],
            )?;
        // We expect all depositories to become filled up to their caps, which is not sufficient for fitting the whole ciculating supply
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            percent_of_supply(40, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
            percent_of_supply(20, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            percent_of_supply(10, circulating_supply)
        );
        assert_eq!(
            depositories_target_redeemable_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX],
            percent_of_supply(5, circulating_supply)
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_no_panic_and_always_over_supply() -> Result<()> {
        proptest!(|(
            circulating_supply: u64,
            identity_depository_weight_random: u16,
            mercurial_vault_depository_weight_random: u16,
            credix_lp_depository_weight_random: u16,
            alloyx_vault_depository_weight_random: u16,
            identity_depository_hard_cap: u64,
            mercurial_vault_depository_hard_cap: u64,
            credix_lp_depository_hard_cap: u64,
            alloyx_vault_depository_hard_cap: u64,
        )| {
            // Check if the hard caps will fit inside of a u64
            // If not, this function will not be expected to work
            let total_hard_caps = u128::from(identity_depository_hard_cap)
                + u128::from(mercurial_vault_depository_hard_cap)
                + u128::from(credix_lp_depository_hard_cap)+ u128::from(alloyx_vault_depository_hard_cap);
            if total_hard_caps > u128::from(u64::MAX) {
                return Ok(());
            }

            // Enforce for our testing that the weights are always equivalent to 100%
            // To do this, we just generate a bunch of weights based on relative values of the random parameters
            // This is because otherwise, proptest will never generate weights that add up exacly to 100%
            let identity_depository_weight_arbitrary = u64::from(identity_depository_weight_random) + 1;
            let mercurial_vault_depository_weight_arbitrary = u64::from(mercurial_vault_depository_weight_random) + 1;
            let credix_lp_depository_weight_arbitrary = u64::from(credix_lp_depository_weight_random) + 1;
            let alloyx_vault_depository_weight_arbitrary = u64::from(alloyx_vault_depository_weight_random) + 1;

            let total_weight_arbitrary = identity_depository_weight_arbitrary + mercurial_vault_depository_weight_arbitrary + credix_lp_depository_weight_arbitrary + alloyx_vault_depository_weight_arbitrary;

            let identity_depository_weight_bps = identity_depository_weight_arbitrary * BPS_POWER / total_weight_arbitrary;
            let mercurial_vault_depository_weight_bps = mercurial_vault_depository_weight_arbitrary * BPS_POWER / total_weight_arbitrary;
            let credix_lp_depository_weight_bps = credix_lp_depository_weight_arbitrary * BPS_POWER / total_weight_arbitrary;
            let alloyx_vault_depository_weight_bps = alloyx_vault_depository_weight_arbitrary * BPS_POWER / total_weight_arbitrary;

            // In case of rounding error, we add the rounding errors to identity depository to keep the sum EXACTLY to 100%
            let total_weight_bps = identity_depository_weight_bps + mercurial_vault_depository_weight_bps + credix_lp_depository_weight_bps + alloyx_vault_depository_weight_bps;
            let identity_depository_weight_bps = identity_depository_weight_bps + BPS_POWER - total_weight_bps;

            // Everything else should never panic
            let depositories_target_redeemable_amount =
                calculate_depositories_target_redeemable_amount(
                    circulating_supply.into(),
                    &vec![
                        DepositoryInfoForTargetRedeemableAmount {
                            weight_bps: u16::try_from(identity_depository_weight_bps).unwrap(),
                            redeemable_amount_under_management_cap: identity_depository_hard_cap.into(),
                        },
                        DepositoryInfoForTargetRedeemableAmount {
                            weight_bps: u16::try_from(mercurial_vault_depository_weight_bps).unwrap(),
                            redeemable_amount_under_management_cap: mercurial_vault_depository_hard_cap.into(),
                        },
                        DepositoryInfoForTargetRedeemableAmount {
                            weight_bps: u16::try_from(credix_lp_depository_weight_bps).unwrap(),
                            redeemable_amount_under_management_cap: credix_lp_depository_hard_cap.into(),
                        },
                        DepositoryInfoForTargetRedeemableAmount {
                            weight_bps: u16::try_from(alloyx_vault_depository_weight_bps).unwrap(),
                            redeemable_amount_under_management_cap: alloyx_vault_depository_hard_cap.into(),
                        },
                    ],
                )?;

            // The sum of all depositories targets should always be either:
            // - equal to the circulating supply
            // - or equal to the sum of all depositories caps

            let maximum_redeemable_amount = std::cmp::min(circulating_supply, u64::try_from(total_hard_caps).unwrap());

            let total_target_redeemable_amount = depositories_target_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX]
                + depositories_target_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX]
                + depositories_target_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX]
                + depositories_target_redeemable_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX];

            // Check for equality while allowing 1 of rounding errors per depository
            let allowed_precision_loss = u64::try_from(ROUTER_DEPOSITORIES_COUNT).unwrap();

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
