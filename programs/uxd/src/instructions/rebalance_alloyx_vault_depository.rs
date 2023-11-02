use crate::error::UxdError;
use crate::state::alloyx_vault_depository::AlloyxVaultDepository;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::utils::calculate_router_depositories_target_redeemable_amount;
use crate::utils::checked_add;
use crate::utils::checked_sub;
use crate::utils::compute_decrease;
use crate::utils::compute_increase;
use crate::utils::compute_shares_amount_for_value_floor;
use crate::utils::compute_value_for_shares_amount_floor;
use crate::utils::is_within_range_inclusive;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::ALLOYX_VAULT_DEPOSITORY_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

#[derive(Accounts)]
pub struct RebalanceAlloyxVaultDepository<'info> {
    /// #1
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2 - This account will be responsible for paying the precision loss
    #[account(
        mut,
        token::authority = payer,
        token::mint = collateral_mint,
    )]
    pub payer_collateral: Box<Account<'info, TokenAccount>>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.identity_depository == identity_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.mercurial_vault_depository == mercurial_vault_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.credix_lp_depository == credix_lp_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.alloyx_vault_depository == alloyx_vault_depository.key() @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #5
    #[account(
        mut,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = identity_depository.load()?.collateral_vault == identity_depository_collateral.key() @UxdError::InvalidDepositoryCollateral,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #6
    #[account(mut)]
    pub identity_depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #7
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub mercurial_vault_depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #8
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub credix_lp_depository: AccountLoader<'info, CredixLpDepository>,

    /// #9
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = alloyx_vault_depository.load()?.depository_collateral == alloyx_vault_depository_collateral.key() @UxdError::InvalidDepositoryCollateral,
        constraint = alloyx_vault_depository.load()?.depository_shares == alloyx_vault_depository_shares.key() @UxdError::InvalidDepositoryShares,
        has_one = alloyx_vault_info @UxdError::InvalidAlloyxVaultInfo,
        has_one = alloyx_vault_collateral @UxdError::InvalidAlloyxVaultCollateral,
        has_one = alloyx_vault_shares @UxdError::InvalidAlloyxVaultShares,
        has_one = alloyx_vault_mint @UxdError::InvalidAlloyxVaultMint,
        has_one = profits_beneficiary_collateral @UxdError::InvalidProfitsBeneficiaryCollateral,
    )]
    pub alloyx_vault_depository: AccountLoader<'info, AlloyxVaultDepository>,

    /// #10
    #[account(mut)]
    pub alloyx_vault_depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(mut)]
    pub alloyx_vault_depository_shares: Box<Account<'info, TokenAccount>>,

    /// #12
    pub alloyx_vault_info: Box<Account<'info, alloyx_cpi::VaultInfo>>,

    /// #13
    #[account(mut)]
    pub alloyx_vault_collateral: Box<Account<'info, TokenAccount>>,

    /// #14
    #[account(mut)]
    pub alloyx_vault_shares: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(mut)]
    pub alloyx_vault_mint: Box<Account<'info, Mint>>,

    /// #16
    #[account(
        constraint = alloyx_vault_pass.investor == alloyx_vault_depository.key() @UxdError::InvalidAlloyxVaultPass,
    )]
    pub alloyx_vault_pass: Account<'info, alloyx_cpi::PassInfo>,

    /// #17
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub profits_beneficiary_collateral: Box<Account<'info, TokenAccount>>,

    /// #18
    pub system_program: Program<'info, System>,
    /// #19
    pub token_program: Program<'info, Token>,
    /// #20
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #21
    pub alloyx_program: Program<'info, alloyx_cpi::program::AlloyxSolana>,
}

pub(crate) fn handler(ctx: Context<RebalanceAlloyxVaultDepository>, vault_id: &str) -> Result<()> {
    let redeemable_amount_under_management = ctx
        .accounts
        .alloyx_vault_depository
        .load()?
        .redeemable_amount_under_management;

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- If possible, starts with actually withdrawing all profits we can
    // ---------------------------------------------------------------------

    let profits_collateral_amount = {
        let liquidity_collateral_amount = ctx.accounts.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount = ctx.accounts.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply = ctx.accounts.alloyx_vault_mint.supply;
        let total_shares_value =
            checked_add(liquidity_collateral_amount, outstanding_collateral_amount)?;
        let owned_shares_amount = ctx.accounts.alloyx_vault_depository_shares.amount;
        let owned_shares_value = compute_value_for_shares_amount_floor(
            owned_shares_amount,
            total_shares_supply,
            total_shares_value,
        )?;
        checked_sub(owned_shares_value, redeemable_amount_under_management)?
    };
    msg!(
        "[rebalance_alloyx_vault_depository:profits_collateral_amount:{}]",
        profits_collateral_amount
    );

    let withdrawn_profits_collateral_amount = ctx
        .accounts
        .withdraw_from_alloyx_vault_to_identity_depository(profits_collateral_amount, vault_id)?;
    if withdrawn_profits_collateral_amount > 0 {
        // Transfer the withdrawn collateral to the beneficiary
        let identity_depository_pda_signer: &[&[&[u8]]] = &[&[
            IDENTITY_DEPOSITORY_NAMESPACE,
            &[ctx.accounts.identity_depository.load()?.bump],
        ]];
        token::transfer(
            ctx.accounts
                .into_transfer_identity_depository_collateral_to_profits_beneficiary_collateral_context()
                .with_signer(identity_depository_pda_signer),
                withdrawn_profits_collateral_amount,
        )?;
        // Reload after transfer
        ctx.accounts.identity_depository_collateral.reload()?;
        // Profit collection accounting updates
        let mut controller = ctx.accounts.controller.load_mut()?;
        let mut alloyx_vault_depository = ctx.accounts.alloyx_vault_depository.load_mut()?;
        controller.update_onchain_accounting_following_profits_collection(
            withdrawn_profits_collateral_amount,
        )?;
        alloyx_vault_depository.update_onchain_accounting_following_profits_collection(
            withdrawn_profits_collateral_amount,
        )?;
    }

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Compute the target redeemable amount, then we can decide if we withdraw or deposit
    // ---------------------------------------------------------------------

    let redeemable_amount_under_management_target_amount =
        calculate_router_depositories_target_redeemable_amount(
            &ctx.accounts.controller,
            &ctx.accounts.identity_depository,
            &ctx.accounts.mercurial_vault_depository,
            &ctx.accounts.credix_lp_depository,
            &ctx.accounts.alloyx_vault_depository,
        )?
        .alloyx_vault_depository_target_redeemable_amount;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- If the alloyx depository is under-target
    // -- We withdraw what we can from identity_depository
    // -- then deposit into alloyx_vault_depository's vault
    // ---------------------------------------------------------------------
    if redeemable_amount_under_management < redeemable_amount_under_management_target_amount {
        let underflow_value = checked_sub(
            redeemable_amount_under_management_target_amount,
            redeemable_amount_under_management,
        )?;
        msg!(
            "[rebalance_alloyx_vault_depository:underflow_value:{}]",
            underflow_value
        );
        let deposited_underflow_collateral = ctx
            .accounts
            .deposit_to_alloyx_vault_from_identity_depository(underflow_value, vault_id)?;
        if deposited_underflow_collateral > 0 {
            // Update accounting
            let mut identity_depository = ctx.accounts.identity_depository.load_mut()?;
            let mut alloyx_vault_depository = ctx.accounts.alloyx_vault_depository.load_mut()?;
            // Collateral amount deposited accounting updates
            identity_depository.collateral_amount_deposited = checked_sub(
                identity_depository.collateral_amount_deposited,
                deposited_underflow_collateral.into(),
            )?;
            alloyx_vault_depository.collateral_amount_deposited = checked_add(
                alloyx_vault_depository.collateral_amount_deposited,
                deposited_underflow_collateral,
            )?;
            // Redeemable under management accounting updates
            identity_depository.redeemable_amount_under_management = checked_sub(
                identity_depository.redeemable_amount_under_management,
                deposited_underflow_collateral.into(),
            )?;
            alloyx_vault_depository.redeemable_amount_under_management = checked_add(
                alloyx_vault_depository.redeemable_amount_under_management,
                deposited_underflow_collateral,
            )?;
        }
    }
    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- If the alloyx depository is over-target
    // -- We withdraw what we can from the available liquidity of alloyx_vault_depository's vault
    // -- then deposit it into the identity_depository
    // ---------------------------------------------------------------------
    else {
        let overflow_value = checked_sub(
            redeemable_amount_under_management,
            redeemable_amount_under_management_target_amount,
        )?;
        msg!(
            "[rebalance_alloyx_vault_depository:overflow_value:{}]",
            overflow_value
        );
        let withdrawn_overflow_collateral = ctx
            .accounts
            .withdraw_from_alloyx_vault_to_identity_depository(overflow_value, vault_id)?;
        if withdrawn_overflow_collateral > 0 {
            // Update accounting
            let mut identity_depository = ctx.accounts.identity_depository.load_mut()?;
            let mut alloyx_vault_depository = ctx.accounts.alloyx_vault_depository.load_mut()?;
            // Collateral amount deposited accounting updates
            identity_depository.collateral_amount_deposited = checked_add(
                identity_depository.collateral_amount_deposited,
                withdrawn_overflow_collateral.into(),
            )?;
            alloyx_vault_depository.collateral_amount_deposited = checked_sub(
                alloyx_vault_depository.collateral_amount_deposited,
                withdrawn_overflow_collateral,
            )?;
            // Redeemable under management accounting updates
            identity_depository.redeemable_amount_under_management = checked_add(
                identity_depository.redeemable_amount_under_management,
                withdrawn_overflow_collateral.into(),
            )?;
            alloyx_vault_depository.redeemable_amount_under_management = checked_sub(
                alloyx_vault_depository.redeemable_amount_under_management,
                withdrawn_overflow_collateral,
            )?;
        }
    }

    // Done
    Ok(())
}

impl<'info> RebalanceAlloyxVaultDepository<'info> {
    // Deposit to alloyx's vault from the identity_depository,
    // on a best effort basis, while double-checking all the output results
    // returns the exact amount deposited
    pub fn deposit_to_alloyx_vault_from_identity_depository(
        &mut self,
        desired_collateral_amount: u64,
        vault_id: &str,
    ) -> Result<u64> {
        if desired_collateral_amount == 0 {
            return Ok(0);
        }

        // Read onchain state before CPI
        let payer_collateral_amount_before = self.payer_collateral.amount;
        let identity_depository_collateral_amount_before =
            self.identity_depository_collateral.amount;
        let alloyx_vault_depository_collateral_amount_before =
            self.alloyx_vault_depository_collateral.amount;
        let liquidity_collateral_amount_before = self.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount_before = self.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply_before = self.alloyx_vault_mint.supply;
        let total_shares_value_before = checked_add(
            liquidity_collateral_amount_before,
            outstanding_collateral_amount_before,
        )?;
        let owned_shares_amount_before = self.alloyx_vault_depository_shares.amount;
        let owned_shares_value_before = compute_value_for_shares_amount_floor(
            owned_shares_amount_before,
            total_shares_supply_before,
            total_shares_value_before,
        )?;

        // Compute deposited amount
        let collateral_amount_before_precision_loss = std::cmp::min(
            desired_collateral_amount,
            identity_depository_collateral_amount_before,
        );
        let shares_amount = compute_shares_amount_for_value_floor(
            collateral_amount_before_precision_loss,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
        if shares_amount == 0 {
            return Ok(0);
        }
        let collateral_amount_after_precision_loss = compute_value_for_shares_amount_floor(
            shares_amount,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
        if collateral_amount_after_precision_loss == 0 {
            return Ok(0);
        }
        let collateral_amount_delta_precision_loss = checked_sub(
            collateral_amount_before_precision_loss,
            collateral_amount_after_precision_loss,
        )?;

        // Actually runs the CPI
        let alloyx_vault_info = self.alloyx_vault_depository.load()?.alloyx_vault_info;
        let collateral_mint = self.alloyx_vault_depository.load()?.collateral_mint;
        let identity_depository_pda_signer: &[&[&[u8]]] = &[&[
            IDENTITY_DEPOSITORY_NAMESPACE,
            &[self.identity_depository.load()?.bump],
        ]];
        let alloyx_vault_depository_pda_signer: &[&[&[u8]]] = &[&[
            ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
            alloyx_vault_info.as_ref(),
            collateral_mint.as_ref(),
            &[self.alloyx_vault_depository.load()?.bump],
        ]];
        token::transfer(
            self.into_transfer_identity_depository_collateral_to_alloyx_vault_depository_collateral_context()
                .with_signer(identity_depository_pda_signer),
            collateral_amount_after_precision_loss,
        )?;
        token::transfer(
            self.into_transfer_payer_collateral_to_alloyx_vault_depository_collateral_context(),
            collateral_amount_delta_precision_loss,
        )?;
        alloyx_cpi::cpi::deposit(
            self.into_deposit_alloyx_vault_depository_to_alloyx_vault_context()
                .with_signer(alloyx_vault_depository_pda_signer),
            vault_id.to_owned(),
            collateral_amount_before_precision_loss,
        )?;

        // Reload onchain data
        self.payer_collateral.reload()?;
        self.identity_depository_collateral.reload()?;
        self.alloyx_vault_depository_collateral.reload()?;
        self.alloyx_vault_depository_shares.reload()?;
        self.alloyx_vault_info.reload()?;
        self.alloyx_vault_collateral.reload()?;
        self.alloyx_vault_mint.reload()?;

        // Read onchain state after CPI
        let payer_collateral_amount_after = self.payer_collateral.amount;
        let identity_depository_collateral_amount_after =
            self.identity_depository_collateral.amount;
        let alloyx_vault_depository_collateral_amount_after =
            self.alloyx_vault_depository_collateral.amount;
        let liquidity_collateral_amount_after = self.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount_after = self.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply_after = self.alloyx_vault_mint.supply;
        let total_shares_value_after = checked_add(
            liquidity_collateral_amount_after,
            outstanding_collateral_amount_after,
        )?;
        let owned_shares_amount_after = self.alloyx_vault_depository_shares.amount;
        let owned_shares_value_after = compute_value_for_shares_amount_floor(
            owned_shares_amount_after,
            total_shares_supply_after,
            total_shares_value_after,
        )?;

        // Compute funds movements
        let payer_collateral_amount_decrease = compute_decrease(
            payer_collateral_amount_before,
            payer_collateral_amount_after,
        )?;
        let identity_depository_collateral_amount_decrease = compute_decrease(
            identity_depository_collateral_amount_before,
            identity_depository_collateral_amount_after,
        )?;
        let total_shares_supply_increase =
            compute_increase(total_shares_supply_before, total_shares_supply_after)?;
        let total_shares_value_increase =
            compute_increase(total_shares_value_before, total_shares_value_after)?;
        let owned_shares_amount_increase =
            compute_increase(owned_shares_amount_before, owned_shares_amount_after)?;
        let owned_shares_value_increase =
            compute_increase(owned_shares_value_before, owned_shares_value_after)?;

        // Verify that everything went exactly like according to plan
        require!(
            payer_collateral_amount_decrease == collateral_amount_delta_precision_loss,
            UxdError::CollateralDepositAmountsDoesntMatch,
        );
        require!(
            identity_depository_collateral_amount_decrease
                == collateral_amount_after_precision_loss,
            UxdError::CollateralDepositAmountsDoesntMatch,
        );
        require!(
            alloyx_vault_depository_collateral_amount_before
                == alloyx_vault_depository_collateral_amount_after,
            UxdError::CollateralDepositHasRemainingDust
        );
        require!(
            total_shares_supply_increase == shares_amount,
            UxdError::CollateralDepositAmountsDoesntMatch,
        );
        require!(
            owned_shares_amount_increase == shares_amount,
            UxdError::CollateralDepositAmountsDoesntMatch,
        );
        require!(
            is_within_range_inclusive(
                total_shares_value_increase,
                collateral_amount_after_precision_loss,
                collateral_amount_before_precision_loss,
            ),
            UxdError::CollateralDepositDoesntMatchTokenValue,
        );
        require!(
            is_within_range_inclusive(
                owned_shares_value_increase,
                collateral_amount_after_precision_loss,
                collateral_amount_before_precision_loss,
            ),
            UxdError::CollateralDepositDoesntMatchTokenValue,
        );

        // Return exactly how much was deposited
        Ok(collateral_amount_after_precision_loss)
    }

    // Withdraw from alloyx's vault toward the identity_depository,
    // on a best effort basis, while double-checking all the output results
    // returns the exact amount withdrawn
    pub fn withdraw_from_alloyx_vault_to_identity_depository(
        &mut self,
        desired_collateral_amount: u64,
        vault_id: &str,
    ) -> Result<u64> {
        if desired_collateral_amount == 0 {
            return Ok(0);
        }

        // Read onchain state before CPI
        let payer_collateral_amount_before = self.payer_collateral.amount;
        let identity_depository_collateral_amount_before =
            self.identity_depository_collateral.amount;
        let alloyx_vault_depository_collateral_amount_before =
            self.alloyx_vault_depository_collateral.amount;
        let liquidity_collateral_amount_before = self.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount_before = self.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply_before = self.alloyx_vault_mint.supply;
        let total_shares_value_before = checked_add(
            liquidity_collateral_amount_before,
            outstanding_collateral_amount_before,
        )?;
        let owned_shares_amount_before = self.alloyx_vault_depository_shares.amount;
        let owned_shares_value_before = compute_value_for_shares_amount_floor(
            owned_shares_amount_before,
            total_shares_supply_before,
            total_shares_value_before,
        )?;

        // Compute withdrawn amount
        let collateral_amount_before_precision_loss = std::cmp::min(
            desired_collateral_amount,
            liquidity_collateral_amount_before,
        );
        let shares_amount = compute_shares_amount_for_value_floor(
            collateral_amount_before_precision_loss,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
        if shares_amount == 0 {
            return Ok(0);
        }
        let collateral_amount_after_precision_loss = compute_value_for_shares_amount_floor(
            shares_amount,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
        if collateral_amount_after_precision_loss == 0 {
            return Ok(0);
        }
        let collateral_amount_delta_precision_loss = checked_sub(
            collateral_amount_before_precision_loss,
            collateral_amount_after_precision_loss,
        )?;

        // Actually runs the CPI
        let alloyx_vault_info = self.alloyx_vault_depository.load()?.alloyx_vault_info;
        let collateral_mint = self.alloyx_vault_depository.load()?.collateral_mint;
        let alloyx_vault_depository_pda_signer: &[&[&[u8]]] = &[&[
            ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
            alloyx_vault_info.as_ref(),
            collateral_mint.as_ref(),
            &[self.alloyx_vault_depository.load()?.bump],
        ]];
        alloyx_cpi::cpi::withdraw(
            self.into_withdraw_from_alloyx_vault_to_alloyx_vault_depository_context()
                .with_signer(alloyx_vault_depository_pda_signer),
            vault_id.to_owned(),
            shares_amount,
        )?;
        token::transfer(
            self.into_transfer_payer_collateral_to_identity_depository_collateral_context(),
            collateral_amount_delta_precision_loss,
        )?;
        token::transfer(
            self.into_transfer_alloyx_vault_depository_collateral_to_identity_depository_collateral_context()
                .with_signer(alloyx_vault_depository_pda_signer),
            collateral_amount_after_precision_loss,
        )?;

        // Reload onchain data
        self.payer_collateral.reload()?;
        self.identity_depository_collateral.reload()?;
        self.alloyx_vault_depository_collateral.reload()?;
        self.alloyx_vault_depository_shares.reload()?;
        self.alloyx_vault_info.reload()?;
        self.alloyx_vault_collateral.reload()?;
        self.alloyx_vault_mint.reload()?;

        // Read onchain state after CPI
        let payer_collateral_amount_after = self.payer_collateral.amount;
        let identity_depository_collateral_amount_after =
            self.identity_depository_collateral.amount;
        let alloyx_vault_depository_collateral_amount_after =
            self.alloyx_vault_depository_collateral.amount;
        let liquidity_collateral_amount_after = self.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount_after = self.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply_after = self.alloyx_vault_mint.supply;
        let total_shares_value_after = checked_add(
            liquidity_collateral_amount_after,
            outstanding_collateral_amount_after,
        )?;
        let owned_shares_amount_after = self.alloyx_vault_depository_shares.amount;
        let owned_shares_value_after = compute_value_for_shares_amount_floor(
            owned_shares_amount_after,
            total_shares_supply_after,
            total_shares_value_after,
        )?;

        // Compute funds movements
        let payer_collateral_amount_decrease = compute_decrease(
            payer_collateral_amount_before,
            payer_collateral_amount_after,
        )?;
        let identity_depository_collateral_amount_increase = compute_increase(
            identity_depository_collateral_amount_before,
            identity_depository_collateral_amount_after,
        )?;
        let total_shares_supply_decrease =
            compute_decrease(total_shares_supply_before, total_shares_supply_after)?;
        let total_shares_value_decrease =
            compute_decrease(total_shares_value_before, total_shares_value_after)?;
        let owned_shares_amount_decrease =
            compute_decrease(owned_shares_amount_before, owned_shares_amount_after)?;
        let owned_shares_value_decrease =
            compute_decrease(owned_shares_value_before, owned_shares_value_after)?;

        // Verify that everything went exactly like according to plan
        require!(
            payer_collateral_amount_decrease == collateral_amount_delta_precision_loss,
            UxdError::CollateralDepositAmountsDoesntMatch,
        );
        require!(
            identity_depository_collateral_amount_increase
                == collateral_amount_before_precision_loss,
            UxdError::CollateralDepositAmountsDoesntMatch,
        );
        require!(
            alloyx_vault_depository_collateral_amount_before
                == alloyx_vault_depository_collateral_amount_after,
            UxdError::CollateralDepositHasRemainingDust
        );
        require!(
            total_shares_supply_decrease == shares_amount,
            UxdError::CollateralDepositAmountsDoesntMatch,
        );
        require!(
            owned_shares_amount_decrease == shares_amount,
            UxdError::CollateralDepositAmountsDoesntMatch,
        );
        require!(
            is_within_range_inclusive(
                total_shares_value_decrease,
                collateral_amount_after_precision_loss,
                collateral_amount_before_precision_loss,
            ),
            UxdError::CollateralDepositDoesntMatchTokenValue,
        );
        require!(
            is_within_range_inclusive(
                owned_shares_value_decrease,
                collateral_amount_after_precision_loss,
                collateral_amount_before_precision_loss,
            ),
            UxdError::CollateralDepositDoesntMatchTokenValue,
        );

        // Return exactly how much was withdrawn
        Ok(collateral_amount_before_precision_loss)
    }
}

// Into functions
impl<'info> RebalanceAlloyxVaultDepository<'info> {
    pub fn into_transfer_identity_depository_collateral_to_profits_beneficiary_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.identity_depository_collateral.to_account_info(),
            to: self.profits_beneficiary_collateral.to_account_info(),
            authority: self.identity_depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_payer_collateral_to_alloyx_vault_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.payer_collateral.to_account_info(),
            to: self.alloyx_vault_depository_collateral.to_account_info(),
            authority: self.payer.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_identity_depository_collateral_to_alloyx_vault_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.identity_depository_collateral.to_account_info(),
            to: self.alloyx_vault_depository_collateral.to_account_info(),
            authority: self.identity_depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_alloyx_vault_depository_to_alloyx_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, alloyx_cpi::cpi::accounts::Deposit<'info>> {
        let cpi_accounts = alloyx_cpi::cpi::accounts::Deposit {
            signer: self.alloyx_vault_depository.to_account_info(),
            investor_pass: self.alloyx_vault_pass.to_account_info(),
            vault_info_account: self.alloyx_vault_info.to_account_info(),
            usdc_vault_account: self.alloyx_vault_collateral.to_account_info(),
            usdc_mint: self.collateral_mint.to_account_info(),
            alloyx_vault_account: self.alloyx_vault_shares.to_account_info(),
            alloyx_mint: self.alloyx_vault_mint.to_account_info(),
            user_usdc_account: self.alloyx_vault_depository_collateral.to_account_info(),
            user_alloyx_account: self.alloyx_vault_depository_shares.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_program = self.alloyx_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_payer_collateral_to_identity_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.payer_collateral.to_account_info(),
            to: self.identity_depository_collateral.to_account_info(),
            authority: self.payer.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_alloyx_vault_depository_collateral_to_identity_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.alloyx_vault_depository_collateral.to_account_info(),
            to: self.identity_depository_collateral.to_account_info(),
            authority: self.alloyx_vault_depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_from_alloyx_vault_to_alloyx_vault_depository_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, alloyx_cpi::cpi::accounts::Withdraw<'info>> {
        let cpi_accounts = alloyx_cpi::cpi::accounts::Withdraw {
            signer: self.alloyx_vault_depository.to_account_info(),
            investor_pass: self.alloyx_vault_pass.to_account_info(),
            vault_info_account: self.alloyx_vault_info.to_account_info(),
            usdc_vault_account: self.alloyx_vault_collateral.to_account_info(),
            usdc_mint: self.collateral_mint.to_account_info(),
            alloyx_vault_account: self.alloyx_vault_shares.to_account_info(),
            alloyx_mint: self.alloyx_vault_mint.to_account_info(),
            user_usdc_account: self.alloyx_vault_depository_collateral.to_account_info(),
            user_alloyx_account: self.alloyx_vault_depository_shares.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_program = self.alloyx_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> RebalanceAlloyxVaultDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
