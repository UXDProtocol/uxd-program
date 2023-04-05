// Unit tests
#[cfg(test)]
mod test_calculate_depositories_mint_collateral_amount {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_mint_collateral_amount;
    use uxd::utils::is_within_range_inclusive;

    fn ui_to_native_amount(ui_amount: u64) -> u64 {
        ui_amount * 100_000
    }

    #[test]
    fn test_with_simplest_case() -> Result<()> {
        let input_mint_collateral_amount = ui_to_native_amount(1_000_000);

        // identity_depository has available space to mint
        let identity_depository_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let identity_depository_redeemable_amount_under_management = ui_to_native_amount(500_000);

        // mercurial_vault_depository_0 has available space to mint
        let mercurial_vault_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let mercurial_vault_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(800_000);

        // credix_lp_depository_0 has available space to mint
        let credix_lp_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let credix_lp_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(700_000);

        // Compute
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            input_mint_collateral_amount,
            identity_depository_target_redeemable_amount,
            mercurial_vault_depository_0_target_redeemable_amount,
            credix_lp_depository_0_target_redeemable_amount,
            identity_depository_redeemable_amount_under_management.into(),
            mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            credix_lp_depository_0_redeemable_amount_under_management.into(),
        )?;

        // All depositories should mint since they all have space
        assert_eq!(
            depositories_mint_collateral_amount.identity_depository_mint_collateral_amount,
            ui_to_native_amount(500_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount.mercurial_vault_depository_0_mint_collateral_amount,
            ui_to_native_amount(200_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount.credix_lp_depository_0_mint_collateral_amount,
            ui_to_native_amount(300_000),
        );

        Ok(())
    }

    #[test]
    fn test_with_unbalanced() -> Result<()> {
        let input_mint_collateral_amount = ui_to_native_amount(1_000_000);

        // identity_depository is overflowing
        let identity_depository_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let identity_depository_redeemable_amount_under_management = ui_to_native_amount(1_500_000);

        // mercurial_vault_depository_0 has available space to mint (a lot of it)
        let mercurial_vault_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let mercurial_vault_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(0);

        // credix_lp_depository_0 has available space to mint (a little of it)
        let credix_lp_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let credix_lp_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(500_000);

        // Compute
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            input_mint_collateral_amount,
            identity_depository_target_redeemable_amount,
            mercurial_vault_depository_0_target_redeemable_amount,
            credix_lp_depository_0_target_redeemable_amount,
            identity_depository_redeemable_amount_under_management.into(),
            mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            credix_lp_depository_0_redeemable_amount_under_management.into(),
        )?;

        // Identity should not mint, others should mint in proportion of their available space
        assert_eq!(
            depositories_mint_collateral_amount.identity_depository_mint_collateral_amount,
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_mint_collateral_amount.mercurial_vault_depository_0_mint_collateral_amount,
            ui_to_native_amount(1_000_000) * 2 / 3,
        );
        assert_eq!(
            depositories_mint_collateral_amount.credix_lp_depository_0_mint_collateral_amount,
            ui_to_native_amount(1_000_000) / 3,
        );

        Ok(())
    }

    #[test]
    fn test_with_underflow_and_overflow() -> Result<()> {
        let input_mint_collateral_amount = ui_to_native_amount(1_000_000);

        // identity_depository is overflowing
        let identity_depository_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let identity_depository_redeemable_amount_under_management = ui_to_native_amount(1_500_000);

        // mercurial_vault_depository_0 has available space to mint (a lot of it)
        let mercurial_vault_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let mercurial_vault_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(0);

        // credix_lp_depository_0 has no space
        let credix_lp_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let credix_lp_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(1_000_000);

        // Compute
        let depositories_mint_collateral_amount = calculate_depositories_mint_collateral_amount(
            input_mint_collateral_amount,
            identity_depository_target_redeemable_amount,
            mercurial_vault_depository_0_target_redeemable_amount,
            credix_lp_depository_0_target_redeemable_amount,
            identity_depository_redeemable_amount_under_management.into(),
            mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            credix_lp_depository_0_redeemable_amount_under_management.into(),
        )?;

        // Identity should not mint, others should mint in proportion of their available space
        assert_eq!(
            depositories_mint_collateral_amount.identity_depository_mint_collateral_amount,
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_mint_collateral_amount.mercurial_vault_depository_0_mint_collateral_amount,
            ui_to_native_amount(1_000_000),
        );
        assert_eq!(
            depositories_mint_collateral_amount.credix_lp_depository_0_mint_collateral_amount,
            ui_to_native_amount(0),
        );

        Ok(())
    }

    #[test]
    fn test_with_not_enough_space() -> Result<()> {
        let input_mint_collateral_amount = ui_to_native_amount(1_000_000);

        // identity_depository is overflowing
        let identity_depository_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let identity_depository_redeemable_amount_under_management = ui_to_native_amount(1_500_000);

        // mercurial_vault_depository_0 is almost overflowing (but has a tiny space)
        let mercurial_vault_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let mercurial_vault_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(900_000);

        // credix_lp_depository_0 has available space to mint (not enough of it)
        let credix_lp_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let credix_lp_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(500_000);

        // Compute
        let result = calculate_depositories_mint_collateral_amount(
            input_mint_collateral_amount,
            identity_depository_target_redeemable_amount,
            mercurial_vault_depository_0_target_redeemable_amount,
            credix_lp_depository_0_target_redeemable_amount,
            identity_depository_redeemable_amount_under_management.into(),
            mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            credix_lp_depository_0_redeemable_amount_under_management.into(),
        );

        // It should fail because there is not enough space
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_no_panic_and_match_input() -> Result<()> {
        proptest!(|(
            input_mint_collateral_amount: u64,
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
            let result = calculate_depositories_mint_collateral_amount(
                input_mint_collateral_amount,
                identity_depository_target_redeemable_amount,
                mercurial_vault_depository_0_target_redeemable_amount,
                credix_lp_depository_0_target_redeemable_amount,
                identity_depository_redeemable_amount_under_management.into(),
                mercurial_vault_depository_0_redeemable_amount_under_management.into(),
                credix_lp_depository_0_redeemable_amount_under_management.into(),
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

            if input_mint_collateral_amount > total_mintable_collateral_amount {
                prop_assert!(result.is_err());
                return Ok(());
            }

            // Everything else should never panic
            let depositories_mint_collatteral_amount = result?;

            // The sum of all mint collateral amount should always exactly match the input collateral amount (minus precision loss)
            let total_mint_collateral_amount = depositories_mint_collatteral_amount.identity_depository_mint_collateral_amount
                + depositories_mint_collatteral_amount.mercurial_vault_depository_0_mint_collateral_amount
                + depositories_mint_collatteral_amount.credix_lp_depository_0_mint_collateral_amount;

            prop_assert!(total_mint_collateral_amount <= input_mint_collateral_amount);

            // Check for equality while allowing 1 of precision loss per depository (rounding errors)
            let allowed_precision_loss = 3;

            let value_max = input_mint_collateral_amount;
            let value_min = if input_mint_collateral_amount > allowed_precision_loss {
                input_mint_collateral_amount - allowed_precision_loss
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
