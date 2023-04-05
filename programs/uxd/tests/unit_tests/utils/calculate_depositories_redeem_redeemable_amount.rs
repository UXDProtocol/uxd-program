// Unit tests
#[cfg(test)]
mod test_calculate_depositories_redeem_redeemable_amount {
    use anchor_lang::Result;
    use proptest::prelude::*;
    use uxd::utils::calculate_depositories_redeem_redeemable_amount;
    use uxd::utils::is_within_range_inclusive;

    fn ui_to_native_amount(ui_amount: u64) -> u64 {
        ui_amount * 100_000
    }

    #[test]
    fn test_with_simplest_case() -> Result<()> {
        let input_redeem_redeemable_amount = ui_to_native_amount(1_000_000);

        // identity_depository is overflowing by a little bit
        let identity_depository_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let identity_depository_redeemable_amount_under_management = ui_to_native_amount(1_500_000);

        // mercurial_vault_depository_0 is overflowing by a lot
        let mercurial_vault_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let mercurial_vault_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(2_500_000);

        // Compute
        let depositories_redeem_redeemable_amount =
            calculate_depositories_redeem_redeemable_amount(
                input_redeem_redeemable_amount,
                identity_depository_target_redeemable_amount,
                mercurial_vault_depository_0_target_redeemable_amount,
                identity_depository_redeemable_amount_under_management.into(),
                mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            )?;

        // All depositories should be withdrawn from, weighted by the amount of overflow
        assert_eq!(
            depositories_redeem_redeemable_amount.identity_depository_redeem_redeemable_amount,
            ui_to_native_amount(250_000),
        );
        assert_eq!(
            depositories_redeem_redeemable_amount
                .mercurial_vault_depository_0_redeem_redeemable_amount,
            ui_to_native_amount(750_000),
        );

        Ok(())
    }

    #[test]
    fn test_with_ideal_overflow_and_underflow() -> Result<()> {
        let input_redeem_redeemable_amount = ui_to_native_amount(1_000_000);

        // identity_depository is not filled up (underflow)
        let identity_depository_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let identity_depository_redeemable_amount_under_management = ui_to_native_amount(500_000);

        // mercurial_vault_depository_0 is overflowing by a lot
        let mercurial_vault_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let mercurial_vault_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(2_500_000);

        // Compute
        let depositories_redeem_redeemable_amount =
            calculate_depositories_redeem_redeemable_amount(
                input_redeem_redeemable_amount,
                identity_depository_target_redeemable_amount,
                mercurial_vault_depository_0_target_redeemable_amount,
                identity_depository_redeemable_amount_under_management.into(),
                mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            )?;

        // Only mercurial should be withdrawn from since ideally there is no need to further unbalance identity
        assert_eq!(
            depositories_redeem_redeemable_amount.identity_depository_redeem_redeemable_amount,
            ui_to_native_amount(0),
        );
        assert_eq!(
            depositories_redeem_redeemable_amount
                .mercurial_vault_depository_0_redeem_redeemable_amount,
            ui_to_native_amount(1_000_000),
        );

        Ok(())
    }

    #[test]
    fn test_with_too_big_for_ideal_redeem() -> Result<()> {
        let input_redeem_redeemable_amount = ui_to_native_amount(1_000_000);

        // identity_depository is perfectly balanced
        let identity_depository_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let identity_depository_redeemable_amount_under_management = ui_to_native_amount(1_000_000);

        // mercurial_vault_depository_0 is overflowing by a little bit
        let mercurial_vault_depository_0_target_redeemable_amount = ui_to_native_amount(1_000_000);
        let mercurial_vault_depository_0_redeemable_amount_under_management =
            ui_to_native_amount(1_000_000);

        // Compute
        let depositories_redeem_redeemable_amount =
            calculate_depositories_redeem_redeemable_amount(
                input_redeem_redeemable_amount,
                identity_depository_target_redeemable_amount,
                mercurial_vault_depository_0_target_redeemable_amount,
                identity_depository_redeemable_amount_under_management.into(),
                mercurial_vault_depository_0_redeemable_amount_under_management.into(),
            )?;

        // Both depositories should be withdrawn from, because the redeem amount is too big to keep things balanced
        // More should be withdrawn from mercurial since it was overweight before the redeem
        assert_eq!(
            depositories_redeem_redeemable_amount.identity_depository_redeem_redeemable_amount,
            ui_to_native_amount(200_000),
        );
        assert_eq!(
            depositories_redeem_redeemable_amount
                .mercurial_vault_depository_0_redeem_redeemable_amount,
            ui_to_native_amount(750_000),
        );

        Ok(())
    }

    /*
    #[test]
    fn test_no_panic_and_match_input() -> Result<()> {
        proptest!(|(
            input_redeem_redeemable_amount: u64,
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
            let result = calculate_depositories_redeem_redeemable_amount(
                input_redeem_redeemable_amount,
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

            if input_redeem_redeemable_amount > total_mintable_collateral_amount {
                prop_assert!(result.is_err());
                return Ok(());
            }

            // Everything else should never panic
            let depositories_mint_collatteral_amount = result?;

            // The sum of all mint collateral amount should always exactly match the input collateral amount (minus precision loss)
            let total_redeem_redeemable_amount = depositories_mint_collatteral_amount.identity_depository_redeem_redeemable_amount
                + depositories_mint_collatteral_amount.mercurial_vault_depository_0_redeem_redeemable_amount
                + depositories_mint_collatteral_amount.credix_lp_depository_0_redeem_redeemable_amount;

            prop_assert!(total_redeem_redeemable_amount <= input_redeem_redeemable_amount);

            // Check for equality while allowing 1 of precision loss per depository (rounding errors)
            let allowed_precision_loss = 3;

            let value_max = input_redeem_redeemable_amount;
            let value_min = if input_redeem_redeemable_amount > allowed_precision_loss {
                input_redeem_redeemable_amount - allowed_precision_loss
            } else {
                0
            };

            prop_assert!(
                is_within_range_inclusive(total_redeem_redeemable_amount, value_min, value_max)
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
     */
}
