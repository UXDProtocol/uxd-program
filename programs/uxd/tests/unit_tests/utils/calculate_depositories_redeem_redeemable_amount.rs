// Unit tests
#[cfg(test)]
mod test_calculate_depositories_redeem_redeemable_amount {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_redeem_redeemable_amount;
    use uxd::utils::is_within_range_inclusive;
    use uxd::utils::DepositoryInfoForRedeemRedeemableAmount;
    use uxd::ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX;
    use uxd::ROUTER_DEPOSITORIES_COUNT;
    use uxd::ROUTER_IDENTITY_DEPOSITORY_INDEX;
    use uxd::ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX;

    fn ui_to_native_amount(ui_amount: u64) -> u64 {
        ui_amount * 100_000
    }

    #[test]
    fn test_with_simplest_case() -> Result<()> {
        // Compute
        let depositories_redeem_redeemable_amount =
            calculate_depositories_redeem_redeemable_amount(
                ui_to_native_amount(1_000_000),
                vec![
                    // identity_depository is overflowing by a little bit
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(1_000_000),
                        redeemable_amount_under_management: ui_to_native_amount(1_500_000).into(),
                    },
                    // mercurial_vault_depository_0 is overflowing by a lot
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(1_000_000),
                        redeemable_amount_under_management: ui_to_native_amount(2_500_000).into(),
                    },
                    // credix_lp_depository is illiquid and cannot be redeemed from
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: false,
                        target_redeemable_amount: 99, // this should not matter on illiquid dppository
                        redeemable_amount_under_management: 42, // this should not matter on illiquid depository
                    },
                ],
            )?;
        // All depositories should be withdrawn from, weighted by the amount of overflow
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(250_000),
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX],
            ui_to_native_amount(750_000),
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX],
            0, // except illiquid ones
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_ideal_overflow_and_underflow() -> Result<()> {
        // Compute
        let depositories_redeem_redeemable_amount =
            calculate_depositories_redeem_redeemable_amount(
                ui_to_native_amount(1_000_000),
                vec![
                    // identity_depository is not filled up (underflow)
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(1_000_000),
                        redeemable_amount_under_management: ui_to_native_amount(500_000).into(),
                    },
                    // mercurial_vault_depository_0 is overflowing by a lot
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(1_000_000),
                        redeemable_amount_under_management: ui_to_native_amount(2_500_000).into(),
                    },
                    // credix_lp_depository is illiquid and cannot be redeemed from
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: false,
                        target_redeemable_amount: 99, // this should not matter on illiquid dppository
                        redeemable_amount_under_management: 42, // this should not matter on illiquid depository
                    },
                ],
            )?;
        // Only mercurial should be withdrawn from since ideally there is no need to further unbalance identity
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            0,
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX],
            ui_to_native_amount(1_000_000),
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX],
            0, // except illiquid ones
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_without_any_ideal_amount() -> Result<()> {
        // Compute
        let depositories_redeem_redeemable_amount =
            calculate_depositories_redeem_redeemable_amount(
                ui_to_native_amount(1_000_000),
                vec![
                    // identity_depository is perfectly balanced
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(1_000_000),
                        redeemable_amount_under_management: ui_to_native_amount(1_000_000).into(),
                    },
                    // mercurial_vault_depository_0 is perfectly balanced
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(500_000),
                        redeemable_amount_under_management: ui_to_native_amount(500_000).into(),
                    },
                    // credix_lp_depository is illiquid and cannot be redeemed from
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: false,
                        target_redeemable_amount: 99, // this should not matter on illiquid dppository
                        redeemable_amount_under_management: 42, // this should not matter on illiquid depository
                    },
                ],
            )?;
        // Both depositories should be withdrawn from, because the redeem amount is too big to keep things balanced
        // More should be withdrawn from identity since it was bigger before the redeem
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(1_000_000) * 2 / 3,
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX],
            ui_to_native_amount(1_000_000) / 3,
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX],
            0, // except illiquid ones
        );

        Ok(())
    }

    #[test]
    fn test_with_too_big_for_ideal_redeem() -> Result<()> {
        // Compute
        let depositories_redeem_redeemable_amount =
            calculate_depositories_redeem_redeemable_amount(
                ui_to_native_amount(1_000_000),
                vec![
                    // identity_depository is overflowing by a little bit
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(1_000_000),
                        redeemable_amount_under_management: ui_to_native_amount(1_200_000).into(),
                    },
                    // mercurial_vault_depository_0 is overflowing by a little bit (but is smaller)
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(500_000),
                        redeemable_amount_under_management: ui_to_native_amount(700_000).into(),
                    },
                    // credix_lp_depository is illiquid and cannot be redeemed from
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: false,
                        target_redeemable_amount: 99, // this should not matter on illiquid dppository
                        redeemable_amount_under_management: 42, // this should not matter on illiquid depository
                    },
                ],
            )?;
        // Both depositories should be withdrawn from, because the redeem amount is too big to keep things balanced
        // More should be withdrawn from identity since it was bigger and overweight before the redeem
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(600_000),
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX],
            ui_to_native_amount(400_000),
        );
        assert_eq!(
            // except illiquid ones
            depositories_redeem_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX],
            0,
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_complete_underweight() -> Result<()> {
        // Compute
        let depositories_redeem_redeemable_amount =
            calculate_depositories_redeem_redeemable_amount(
                ui_to_native_amount(1_000_000),
                vec![
                    // identity_depository is underweight
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(1_000_000),
                        redeemable_amount_under_management: ui_to_native_amount(800_000).into(),
                    },
                    // mercurial_vault_depository_0 is underweight (but is smaller)
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: true,
                        target_redeemable_amount: ui_to_native_amount(500_000),
                        redeemable_amount_under_management: ui_to_native_amount(400_000).into(),
                    },
                    // credix_lp_depository is illiquid and cannot be redeemed from
                    DepositoryInfoForRedeemRedeemableAmount {
                        is_liquid: false,
                        target_redeemable_amount: 99, // this should not matter on illiquid dppository
                        redeemable_amount_under_management: 42, // this should not matter on illiquid depository
                    },
                ],
            )?;
        // Both depositories should be withdrawn from,
        // More should be withdrawn from identity since it was bigger before the redeem
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(1_000_000) * 2 / 3,
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX],
            ui_to_native_amount(1_000_000) / 3,
        );
        assert_eq!(
            depositories_redeem_redeemable_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX],
            0, // except illiquid ones
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_no_panic_and_match_input() -> Result<()> {
        proptest!(|(
            requested_redeem_redeemable_amount: u64,
            identity_depository_target_redeemable_amount: u64,
            mercurial_vault_depository_0_target_redeemable_amount: u64,
            identity_depository_redeemable_amount_under_management: u64,
            mercurial_vault_depository_0_redeemable_amount_under_management: u64,
        )| {
            // Check if the redeemable_amount_under_management will fit inside of a u64
            // If not, this function will not be expected to work
            let total_redeemable_amount_under_management = u128::from(identity_depository_redeemable_amount_under_management)
                + u128::from(mercurial_vault_depository_0_redeemable_amount_under_management);
            if total_redeemable_amount_under_management > u128::from(u64::MAX) {
                return Ok(());
            }

            // Check if the target_redeemable_amount will fit inside of a u64
            // If not, this function will not be expected to work
            let total_target_redeemable_amount = u128::from(identity_depository_target_redeemable_amount)
                + u128::from(mercurial_vault_depository_0_target_redeemable_amount);
            if total_target_redeemable_amount > u128::from(u64::MAX) {
                return Ok(());
            }

            // Compute
            let result =
                calculate_depositories_redeem_redeemable_amount(
                    requested_redeem_redeemable_amount,
                    vec![
                        DepositoryInfoForRedeemRedeemableAmount {
                            is_liquid: true,
                            target_redeemable_amount: identity_depository_target_redeemable_amount,
                            redeemable_amount_under_management:
                                identity_depository_redeemable_amount_under_management.into(),
                        },
                        DepositoryInfoForRedeemRedeemableAmount {
                            is_liquid: true,
                            target_redeemable_amount: mercurial_vault_depository_0_target_redeemable_amount,
                            redeemable_amount_under_management:
                                mercurial_vault_depository_0_redeemable_amount_under_management.into(),
                        },
                        DepositoryInfoForRedeemRedeemableAmount {
                            is_liquid: false,
                            target_redeemable_amount: 99, // this should not matter on illiquid dppository
                            redeemable_amount_under_management: 42, // this should not matter on illiquid depository
                        },
                    ],
                );

            // If there is not enough space within all depositories, we must fail
            if requested_redeem_redeemable_amount > u64::try_from(total_redeemable_amount_under_management).unwrap() {
                prop_assert!(result.is_err());
                return Ok(());
            }

            // Everything else should never panic
            let depositories_redeem_redeemable_amount = result?;

            // The sum of all mint collateral amount should always exactly match the input collateral amount (minus precision loss)
            let total_redeem_redeemable_amount = depositories_redeem_redeemable_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX]
                + depositories_redeem_redeemable_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX];

            // Check for equality while allowing 1 of precision loss per depository (rounding errors)
            let allowed_precision_loss = u64::try_from(ROUTER_DEPOSITORIES_COUNT).unwrap();

            let value_max = requested_redeem_redeemable_amount;
            let value_min = if requested_redeem_redeemable_amount > allowed_precision_loss {
                requested_redeem_redeemable_amount - allowed_precision_loss
            } else {
                0
            };
            prop_assert!(
                is_within_range_inclusive(total_redeem_redeemable_amount, value_min, value_max)
            );
        });
        Ok(())
    }
}
