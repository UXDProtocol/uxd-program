// Unit tests
#[cfg(test)]
mod test_calculate_depositories_mint_collateral_amount {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_mint_collateral_amount;
    use uxd::utils::is_within_range_inclusive;
    use uxd::utils::DepositoryInfoForMintCollateralAmount;
    use uxd::ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX;
    use uxd::ROUTER_CREDIX_LP_DEPOSITORY_INDEX;
    use uxd::ROUTER_DEPOSITORIES_COUNT;
    use uxd::ROUTER_IDENTITY_DEPOSITORY_INDEX;
    use uxd::ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX;

    fn ui_to_native_amount(ui_amount: u64) -> u64 {
        ui_amount * 1_000_000
    }

    #[test]
    fn test_with_simplest_case() -> Result<()> {
        // Compute
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository has available space to mint
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // mercurial_vault_depository has available space to mint
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(800_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // credix_lp_depository has available space to mint
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(700_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // alloyx_vault_depository has available space (but cannot be directly minted)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: false,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(100_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
            ],
        )?;
        // All depositories should mint since they all have space (for depositories that can be directly minted to)
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(500_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
            ui_to_native_amount(200_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            ui_to_native_amount(300_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_unbalanced() -> Result<()> {
        // Compute
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository is overflowing
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // mercurial_vault_depository has available space to mint (a lot of it)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(0),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // credix_lp_depository has available space to mint (a little of it)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // alloyx_vault_depository has available space (but cannot be directly minted)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: false,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
            ],
        )?;
        // Identity should not mint, others should mint in proportion of their available space (for depositories that can be directly minted to)
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
            ui_to_native_amount(1_000_000) * 2 / 3,
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            ui_to_native_amount(1_000_000) / 3,
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_underflow_and_overflow() -> Result<()> {
        // Compute
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository is overflowing
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // mercurial_vault_depository has available space to mint (a lot of it)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(0),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // credix_lp_depository has no space at all
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
                // alloyx_vault_depository is overflowing (and cannot be directly minted)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: false,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000_000_000), // no-cap
                },
            ],
        )?;
        // Identity should not mint, others should mint in proportion of their available space
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
            ui_to_native_amount(1_000_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_not_enough_space() -> Result<()> {
        // Compute
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository is overflowing and is already at its cap
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_500_000),
                },
                // mercurial_vault_depository is almost overflowing, and a large cap (but has a tiny space)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(800_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(3_000_000),
                },
                // credix_lp_depository has available space to mint and a small cap (not enough of it)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(2_000_000),
                },
                // alloyx_vault_depository has available space to mint (but cannot be directly minted)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: false,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(3_000_000),
                },
            ],
        )?;
        // Identity should not mint, others should mint in proportion of their available space
        // Since we dont have enough total space under the targets,
        // the remainder of the mint should go to depositories that are not yet capped (but should still succeed)
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX],
            ui_to_native_amount(400_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX],
            ui_to_native_amount(600_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_cap_overflow() -> Result<()> {
        // Compute
        let result = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository is overflowing and is already at its cap
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_500_000),
                },
                // mercurial_vault_depository is almost overflowing (but has a tiny space)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(800_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000),
                },
                // credix_lp_depository has available space to mint (not enough of it)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: true,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(1_000_000),
                },
                // alloyx_vault_depository has available space to mint (but cannot be directly minted)
                DepositoryInfoForMintCollateralAmount {
                    directly_mintable: false,
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000),
                    redeemable_amount_under_management_cap: ui_to_native_amount(3_000_000),
                },
            ],
        );
        // Mint should fail since all the mintable depositories are at their caps
        assert!(result.is_err());
        // Done
        Ok(())
    }

    #[test]
    fn test_no_panic_and_match_input() -> Result<()> {
        proptest!(|(
            requested_mint_collateral_amount: u64,
            identity_depository_target_redeemable_amount: u64,
            mercurial_vault_depository_target_redeemable_amount: u64,
            credix_lp_depository_target_redeemable_amount: u64,
            alloyx_vault_depository_target_redeemable_amount: u64,
            identity_depository_redeemable_amount_under_management: u64,
            mercurial_vault_depository_redeemable_amount_under_management: u64,
            credix_lp_depository_redeemable_amount_under_management: u64,
            alloyx_vault_depository_redeemable_amount_under_management: u64,
            identity_depository_redeemable_amount_under_management_cap: u64,
            mercurial_vault_depository_redeemable_amount_under_management_cap: u64,
            credix_lp_depository_redeemable_amount_under_management_cap: u64,
            alloyx_vault_depository_redeemable_amount_under_management_cap: u64,
        )| {

            // Check if the target_redeemable_amount will fit inside of a u64
            // If not, this function will not be expected to work
            let total_target_redeemable_amount = u128::from(identity_depository_target_redeemable_amount)
                + u128::from(mercurial_vault_depository_target_redeemable_amount)
                + u128::from(credix_lp_depository_target_redeemable_amount)
                + u128::from(alloyx_vault_depository_target_redeemable_amount);
            if total_target_redeemable_amount > u128::from(u64::MAX) {
                return Ok(());
            }

            // Check if the redeemable_amount_under_management will fit inside of a u64
            // If not, this function will not be expected to work
            let total_redeemable_amount_under_management = u128::from(identity_depository_redeemable_amount_under_management)
                + u128::from(mercurial_vault_depository_redeemable_amount_under_management)
                + u128::from(credix_lp_depository_redeemable_amount_under_management)
                + u128::from(alloyx_vault_depository_redeemable_amount_under_management);
            if total_redeemable_amount_under_management > u128::from(u64::MAX) {
                return Ok(());
            }

            // Check if the redeemable_amount_under_management_cap will fit inside of a u64
            // If not, this function will not be expected to work
            let total_redeemable_amount_under_management_cap = u128::from(identity_depository_redeemable_amount_under_management_cap)
                + u128::from(mercurial_vault_depository_redeemable_amount_under_management_cap)
                + u128::from(credix_lp_depository_redeemable_amount_under_management_cap)
                + u128::from(alloyx_vault_depository_redeemable_amount_under_management_cap);
            if total_redeemable_amount_under_management_cap > u128::from(u64::MAX) {
                return Ok(());
            }

            // Compute
            let result = calculate_depositories_mint_collateral_amount(
                requested_mint_collateral_amount,
                &vec![
                    DepositoryInfoForMintCollateralAmount {
                        directly_mintable: true,
                        target_redeemable_amount: identity_depository_target_redeemable_amount,
                        redeemable_amount_under_management: identity_depository_redeemable_amount_under_management,
                        redeemable_amount_under_management_cap: identity_depository_redeemable_amount_under_management_cap,
                    },
                    DepositoryInfoForMintCollateralAmount {
                        directly_mintable: true,
                        target_redeemable_amount: mercurial_vault_depository_target_redeemable_amount,
                        redeemable_amount_under_management: mercurial_vault_depository_redeemable_amount_under_management,
                        redeemable_amount_under_management_cap: mercurial_vault_depository_redeemable_amount_under_management_cap,
                    },
                    DepositoryInfoForMintCollateralAmount {
                        directly_mintable: true,
                        target_redeemable_amount: credix_lp_depository_target_redeemable_amount,
                        redeemable_amount_under_management: credix_lp_depository_redeemable_amount_under_management,
                        redeemable_amount_under_management_cap: credix_lp_depository_redeemable_amount_under_management_cap,
                    },
                    DepositoryInfoForMintCollateralAmount {
                        directly_mintable: false,
                        target_redeemable_amount: alloyx_vault_depository_target_redeemable_amount,
                        redeemable_amount_under_management: alloyx_vault_depository_redeemable_amount_under_management,
                        redeemable_amount_under_management_cap: alloyx_vault_depository_redeemable_amount_under_management_cap,
                    },
                ],
            );

            // Compute the amounts for the mintable depositories only
            let relevant_redeemable_amount_under_management = identity_depository_redeemable_amount_under_management
                + mercurial_vault_depository_redeemable_amount_under_management
                + credix_lp_depository_redeemable_amount_under_management;
            let relevant_redeemable_amount_under_management_cap = identity_depository_redeemable_amount_under_management_cap
                + mercurial_vault_depository_redeemable_amount_under_management_cap
                + credix_lp_depository_redeemable_amount_under_management_cap;

            // Since we set the depositories caps equal to the target
            // we want to make sure we fail when the requested amount is larger than the remaining space
            if relevant_redeemable_amount_under_management + requested_mint_collateral_amount > relevant_redeemable_amount_under_management_cap {
                prop_assert!(result.is_err());
                return Ok(());
            }

            // Everything else should never panic
            let depositories_mint_collateral_amount = result?;

            // The sum of all mint collateral amount should always exactly match the input collateral amount (minus precision loss)
            let relevant_mint_collateral_amount = depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX]
                + depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_INDEX]
                + depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_INDEX];

            // The not directly mintable depositories should not be used
            prop_assert!(
                depositories_mint_collateral_amount[ROUTER_ALLOYX_VAULT_DEPOSITORY_INDEX] == 0
            );

            // Check for equality while allowing 2 of precision loss per depository (rounding errors)
            // 1 rounding error allowed per depository for the primary up-to-target split
            // 1 rounding error allowed per depository for the backup up-to-cap split
            let allowed_precision_loss = u64::try_from(ROUTER_DEPOSITORIES_COUNT * 2).unwrap();

            let value_max = requested_mint_collateral_amount;
            let value_min = if requested_mint_collateral_amount > allowed_precision_loss {
                requested_mint_collateral_amount - allowed_precision_loss
            } else {
                0
            };

            prop_assert!(
                is_within_range_inclusive(relevant_mint_collateral_amount, value_min, value_max)
            );
        });
        Ok(())
    }
}
