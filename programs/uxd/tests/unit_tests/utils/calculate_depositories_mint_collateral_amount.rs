// Unit tests
#[cfg(test)]
mod test_calculate_depositories_mint_collateral_amount {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_mint_collateral_amount;
    use uxd::utils::is_within_range_inclusive;
    use uxd::utils::DepositoryInfoForMintCollateralAmount;
    use uxd::ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX;
    use uxd::ROUTER_DEPOSITORIES_COUNT;
    use uxd::ROUTER_IDENTITY_DEPOSITORY_INDEX;
    use uxd::ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX;

    fn ui_to_native_amount(ui_amount: u64) -> u64 {
        ui_amount * 1_000_000
    }

    #[test]
    fn test_with_simplest_case() -> Result<()> {
        // Compute
        let depositories_target_and_redeemable_under_management = vec![
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: identity_depository_target_redeemable_amount,
                redeemable_amount_under_management:
                    identity_depository_redeemable_amount_under_management.into(),
            },
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: mercurial_vault_depository_0_target_redeemable_amount,
                redeemable_amount_under_management:
                    mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            },
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: credix_lp_depository_0_target_redeemable_amount,
                redeemable_amount_under_management:
                    credix_lp_depository_0_redeemable_amount_under_management.into(),
            },
        ];
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository has available space to mint
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000).into(),
                },
                // mercurial_vault_depository_0 has available space to mint
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(800_000).into(),
                },
                // credix_lp_depository_0 has available space to mint
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(700_000).into(),
                },
            ],
        )?;
        // All depositories should mint since they all have space
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(500_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX],
            ui_to_native_amount(200_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX],
            ui_to_native_amount(300_000),
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_unbalanced() -> Result<()> {
        // Compute
        let depositories_target_and_redeemable_under_management = vec![
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: identity_depository_target_redeemable_amount,
                redeemable_amount_under_management:
                    identity_depository_redeemable_amount_under_management.into(),
            },
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: mercurial_vault_depository_0_target_redeemable_amount,
                redeemable_amount_under_management:
                    mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            },
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: credix_lp_depository_0_target_redeemable_amount,
                redeemable_amount_under_management:
                    credix_lp_depository_0_redeemable_amount_under_management.into(),
            },
        ];
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository is overflowing
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_500_000).into(),
                },
                // mercurial_vault_depository_0 has available space to mint (a lot of it)
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(0).into(),
                },
                // credix_lp_depository_0 has available space to mint (a little of it)
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000).into(),
                },
            ],
        )?;
        // Identity should not mint, others should mint in proportion of their available space
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX],
            ui_to_native_amount(1_000_000) * 2 / 3,
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX],
            ui_to_native_amount(1_000_000) / 3,
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_underflow_and_overflow() -> Result<()> {
        // Compute
        let depositories_target_and_redeemable_under_management = vec![
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: identity_depository_target_redeemable_amount,
                redeemable_amount_under_management:
                    identity_depository_redeemable_amount_under_management.into(),
            },
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: mercurial_vault_depository_0_target_redeemable_amount,
                redeemable_amount_under_management:
                    mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            },
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: credix_lp_depository_0_target_redeemable_amount,
                redeemable_amount_under_management:
                    credix_lp_depository_0_redeemable_amount_under_management.into(),
            },
        ];
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository is overflowing
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_500_000).into(),
                },
                // mercurial_vault_depository_0 has available space to mint (a lot of it)
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(0).into(),
                },
                // credix_lp_depository_0 has no space
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_000_000).into(),
                },
            ],
        )?;
        // Identity should not mint, others should mint in proportion of their available space
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX],
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX],
            ui_to_native_amount(1_000_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX],
            ui_to_native_amount(0),
        );
        // Done
        Ok(())
    }

    #[test]
    fn test_with_not_enough_space() -> Result<()> {
        // Compute
        let depositories_target_and_redeemable_under_management = vec![
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: identity_depository_target_redeemable_amount,
                redeemable_amount_under_management:
                    identity_depository_redeemable_amount_under_management.into(),
            },
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: mercurial_vault_depository_0_target_redeemable_amount,
                redeemable_amount_under_management:
                    mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            },
            DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                target_redeemable_amount: credix_lp_depository_0_target_redeemable_amount,
                redeemable_amount_under_management:
                    credix_lp_depository_0_redeemable_amount_under_management.into(),
            },
        ];
        let result = calculate_depositories_mint_collateral_amount(
            ui_to_native_amount(1_000_000),
            &vec![
                // identity_depository is overflowing
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(1_500_000).into(),
                },
                // mercurial_vault_depository_0 is almost overflowing (but has a tiny space)
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(900_000).into(),
                },
                // credix_lp_depository_0 has available space to mint (not enough of it)
                DepositoryInfoForMintCollateralAmount {
                    target_redeemable_amount: ui_to_native_amount(1_000_000),
                    redeemable_amount_under_management: ui_to_native_amount(500_000).into(),
                },
            ],
        );
        // It should fail because there is not enough space
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_no_panic_and_match_input() -> Result<()> {
        proptest!(|(
            requested_mint_collateral_amount: u64,
            identity_depository_target_redeemable_amount: u64,
            mercurial_vault_depository_0_target_redeemable_amount: u64,
            credix_lp_depository_0_target_redeemable_amount: u64,
            identity_depository_redeemable_amount_under_management: u64,
            mercurial_vault_depository_0_redeemable_amount_under_management: u64,
            credix_lp_depository_0_redeemable_amount_under_management: u64,
        )| {
            // Check if the redeemable_amount_under_management will fit inside of a u64
            // If not, this function will not be expected to work
            let total_redeemable_amount_under_management = u128::from(identity_depository_redeemable_amount_under_management)
                + u128::from(mercurial_vault_depository_0_redeemable_amount_under_management)
                + u128::from(credix_lp_depository_0_redeemable_amount_under_management);
            if total_redeemable_amount_under_management > u128::from(u64::MAX) {
                return Ok(());
            }

            // Check if the target_redeemable_amount will fit inside of a u64
            // If not, this function will not be expected to work
            let total_target_redeemable_amount = u128::from(identity_depository_target_redeemable_amount)
                + u128::from(mercurial_vault_depository_0_target_redeemable_amount)
                + u128::from(credix_lp_depository_0_target_redeemable_amount);
            if total_target_redeemable_amount > u128::from(u64::MAX) {
                return Ok(());
            }

            // Compute
            let depositories_target_and_redeemable_under_management = vec![
                DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                    target_redeemable_amount: identity_depository_target_redeemable_amount,
                    redeemable_amount_under_management: identity_depository_redeemable_amount_under_management.into(),
                },
                DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                    target_redeemable_amount: mercurial_vault_depository_0_target_redeemable_amount,
                    redeemable_amount_under_management: mercurial_vault_depository_0_redeemable_amount_under_management.into(),
                },
                DepositoryTargetRedeemableAmountAndRedeemableAmountUnderManagement {
                    target_redeemable_amount: credix_lp_depository_0_target_redeemable_amount,
                    redeemable_amount_under_management: credix_lp_depository_0_redeemable_amount_under_management.into(),
                }
            ];
            let result = calculate_depositories_mint_collateral_amount(
                requested_mint_collateral_amount,
                &vec![
                    DepositoryInfoForMintCollateralAmount {
                        target_redeemable_amount: identity_depository_target_redeemable_amount,
                        redeemable_amount_under_management: identity_depository_redeemable_amount_under_management.into(),
                    },
                    DepositoryInfoForMintCollateralAmount {
                        target_redeemable_amount: mercurial_vault_depository_0_target_redeemable_amount,
                        redeemable_amount_under_management: mercurial_vault_depository_0_redeemable_amount_under_management.into(),
                    },
                    DepositoryInfoForMintCollateralAmount {
                        target_redeemable_amount: credix_lp_depository_0_target_redeemable_amount,
                        redeemable_amount_under_management: credix_lp_depository_0_redeemable_amount_under_management.into(),
                    }
                ],
            );

            // If there is not enough space within all depositories, we must fail
            let identity_depository_mintable_collateral_amount =
                test_calculate_depository_mintable_collateral_amount(
                    identity_depository_redeemable_amount_under_management,
                    identity_depository_target_redeemable_amount,
                );
            let mercurial_vault_depository_0_mintable_collateral_amount =
                test_calculate_depository_mintable_collateral_amount(
                    mercurial_vault_depository_0_redeemable_amount_under_management,
                    mercurial_vault_depository_0_target_redeemable_amount,
                );
            let credix_lp_depository_0_mintable_collateral_amount =
                test_calculate_depository_mintable_collateral_amount(
                    credix_lp_depository_0_redeemable_amount_under_management,
                    credix_lp_depository_0_target_redeemable_amount,
                );
            let total_mintable_collateral_amount = identity_depository_mintable_collateral_amount
                + mercurial_vault_depository_0_mintable_collateral_amount
                + credix_lp_depository_0_mintable_collateral_amount;

            if requested_mint_collateral_amount > total_mintable_collateral_amount {
                prop_assert!(result.is_err());
                return Ok(());
            }

            // Everything else should never panic
            let depositories_mint_collateral_amount = result?;

            // The sum of all mint collateral amount should always exactly match the input collateral amount (minus precision loss)
            let total_mint_collateral_amount = depositories_mint_collateral_amount[ROUTER_IDENTITY_DEPOSITORY_INDEX]
                + depositories_mint_collateral_amount[ROUTER_MERCURIAL_VAULT_DEPOSITORY_0_INDEX]
                + depositories_mint_collateral_amount[ROUTER_CREDIX_LP_DEPOSITORY_0_INDEX];

            // Check for equality while allowing 1 of precision loss per depository (rounding errors)
            let allowed_precision_loss = u64::try_from(ROUTER_DEPOSITORIES_COUNT).unwrap();

            let value_max = requested_mint_collateral_amount;
            let value_min = if requested_mint_collateral_amount > allowed_precision_loss {
                requested_mint_collateral_amount - allowed_precision_loss
            } else {
                0
            };

            prop_assert!(
                is_within_range_inclusive(total_mint_collateral_amount, value_min, value_max)
            );
        });
        Ok(())
    }

    fn test_calculate_depository_mintable_collateral_amount(
        depository_redeemable_amount_under_management: u64,
        depository_target_redeemable_amount: u64,
    ) -> u64 {
        if depository_target_redeemable_amount <= depository_redeemable_amount_under_management {
            return 0;
        }
        depository_target_redeemable_amount - depository_redeemable_amount_under_management
    }
}
