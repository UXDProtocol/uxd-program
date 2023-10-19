use crate::error::UxdError;
use crate::state::alloyx_vault_depository::AlloyxVaultDepository;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::state::mercurial_vault_depository::MercurialVaultDepository;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::ALLOYX_VAULT_DEPOSITORY_NAMESPACE;
use crate::CONTROLLER_NAMESPACE;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RebalanceAlloyxVaultDepository<'info> {
    /// #1
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2
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

    /// #3
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #3
    #[account(
        mut,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = identity_depository.load()?.collateral_vault == identity_depository_collateral.key() @UxdError::InvalidDepositoryCollateral,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #4
    #[account(mut)]
    pub identity_depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #5
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub mercurial_vault_depository: AccountLoader<'info, MercurialVaultDepository>,

    /// #6
    #[account(
        mut,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
    )]
    pub credix_lp_depository: AccountLoader<'info, CredixLpDepository>,

    /// #4
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

    /// #20
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub profits_beneficiary_collateral: Box<Account<'info, TokenAccount>>,

    /// #12
    pub system_program: Program<'info, System>,
    /// #13
    pub token_program: Program<'info, Token>,
    /// #14
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #15
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<RebalanceAlloyxVaultDepository>, vault_id: &str) -> Result<()> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Compute all amounts we need for profits collection and rebalancing
    // ---------------------------------------------------------------------

    let redeemable_amount_under_management = ctx
        .accounts
        .alloyx_vault_depository
        .load()?
        .redeemable_amount_under_management;

    let redeemable_amount_under_management_target_amount =
        calculate_router_depositories_target_redeemable_amount(
            &ctx.accounts.controller,
            &ctx.accounts.identity_depository,
            &ctx.accounts.mercurial_vault_depository,
            &ctx.accounts.credix_lp_depository,
            &ctx.accounts.alloyx_vault_depository,
        )?
        .alloyx_vault_depository_target_redeemable_amount;

    let profits_collateral_amount = {
        let liquidity_collateral_amount: u64 = ctx.accounts.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount: u64 =
            ctx.accounts.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply: u64 = ctx.accounts.alloyx_vault_mint.supply;
        let total_shares_value: u64 =
            checked_add(liquidity_collateral_amount, outstanding_collateral_amount)?;
        let owned_shares_amount: u64 = ctx.accounts.alloyx_vault_depository_shares.amount;
        let owned_shares_value: u64 = compute_value_for_shares_amount_floor(
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

    let overflow_value = {
        if redeemable_amount_under_management < redeemable_amount_under_management_target_amount {
            0
        } else {
            checked_sub(
                redeemable_amount_under_management,
                redeemable_amount_under_management_target_amount,
            )?
        }
    };
    msg!(
        "[rebalance_alloyx_vault_depository:overflow_value:{}]",
        overflow_value
    );

    let underflow_value = {
        if redeemable_amount_under_management_target_amount < redeemable_amount_under_management {
            0
        } else {
            checked_sub(
                redeemable_amount_under_management_target_amount,
                redeemable_amount_under_management,
            )?
        }
    };
    msg!(
        "[rebalance_alloyx_vault_depository:underflow_value:{}]",
        underflow_value
    );

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- If possible, starts with actually withdrawing all profits
    // -- Also updates accounting when needed
    // ---------------------------------------------------------------------

    let withdrawn_profits_collateral_amount = ctx.withdraw_from_alloyx_vault_to(
        profits_collateral_amount,
        ctx.accounts.profits_beneficiary_collateral,
    )?;
    msg!(
        "[rebalance_alloyx_vault_depository:withdrawn_profits_collateral_amount:{}]",
        withdrawn_profits_collateral_amount
    );
    if withdrawn_profits_collateral_amount > 0 {
        // Profit collection accounting updates
        controller.update_onchain_accounting_following_profits_collection(
            withdrawn_profits_collateral_amount,
        )?;
        alloyx_vault_depository.update_onchain_accounting_following_profits_collection(
            withdrawn_profits_collateral_amount,
        )?;
    }

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- If needed and if possible, do some outbound rebalancing
    // -- We withdraw the overflow from alloyx_vault_depository and deposit into identity_depository
    // -- Also updates accounting when needed
    // ---------------------------------------------------------------------

    let withdrawn_overflow_collateral = ctx.withdraw_from_alloyx_vault_to(
        overflow_value,
        ctx.accounts.identity_depository_collateral,
    )?;
    msg!(
        "[rebalance_alloyx_vault_depository:withdrawn_overflow_collateral:{}]",
        withdrawn_overflow_collateral
    );
    if withdrawn_overflow_collateral > 0 {
        // Collateral amount deposited accounting updates
        alloyx_vault_depository.collateral_amount_deposited = checked_sub(
            alloyx_vault_depository.collateral_amount_deposited,
            withdrawn_overflow_collateral.into(),
        )?;
        identity_depository.collateral_amount_deposited = checked_add(
            identity_depository.collateral_amount_deposited,
            withdrawn_overflow_collateral.into(),
        )?;
        // Redeemable under management accounting updates
        alloyx_vault_depository.redeemable_amount_under_management = checked_sub(
            alloyx_vault_depository.redeemable_amount_under_management,
            withdrawn_overflow_collateral.into(),
        )?;
        identity_depository.redeemable_amount_under_management = checked_add(
            identity_depository.redeemable_amount_under_management,
            withdrawn_overflow_collateral.into(),
        )?;
    }

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- If needed and if possible, do some inbound rebalancing
    // -- We withdraw the underflow from identity_depository and deposit into alloyx_vault_depository
    // -- Also updates accounting when needed
    // ---------------------------------------------------------------------

    let deposited_underflow_collateral = ctx.deposit_to_alloyx_vault(underflow_value)?;
    msg!(
        "[rebalance_alloyx_vault_depository:deposited_underflow_collateral:{}]",
        deposited_underflow_collateral
    );
    if deposited_underflow_collateral > 0 {
        // Collateral amount deposited accounting updates
        alloyx_vault_depository.collateral_amount_deposited = checked_add(
            alloyx_vault_depository.collateral_amount_deposited,
            deposited_underflow_collateral.into(),
        )?;
        identity_depository.collateral_amount_deposited = checked_sub(
            identity_depository.collateral_amount_deposited,
            deposited_underflow_collateral.into(),
        )?;
        // Redeemable under management accounting updates
        alloyx_vault_depository.redeemable_amount_under_management = checked_add(
            alloyx_vault_depository.redeemable_amount_under_management,
            deposited_underflow_collateral.into(),
        )?;
        identity_depository.redeemable_amount_under_management = checked_sub(
            identity_depository.redeemable_amount_under_management,
            deposited_underflow_collateral.into(),
        )?;
    }

    // Done
    Ok(())
}

impl<'info> RebalanceAlloyxVaultDepository<'info> {
    // Withdraw from alloyx's vault toward the destination_collateral,
    // on a best effort basis, while double-checking all the output results
    // returns the exact amount withdrawn
    pub(crate) fn withdraw_from_alloyx_vault_to(
        &self,
        desired_collateral_amount: u64,
        destination_collateral: Box<Account<'info, TokenAccount>>,
    ) -> Result<u64> {
        if desired_collateral_amount <= 0 {
            return Ok(0);
        }

        // Read onchain state before CPI
        let destination_collateral_amount_before: u64 = destination_collateral.amount;
        let alloyx_vault_depository_collateral_amount_before: u64 =
            ctx.accounts.alloyx_vault_depository_collateral.amount;
        let liquidity_collateral_amount_before: u64 = ctx.accounts.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount_before: u64 =
            ctx.accounts.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply_before: u64 = ctx.accounts.alloyx_vault_mint.supply;
        let total_shares_value_before: u64 = checked_add(
            liquidity_collateral_amount_before,
            outstanding_collateral_amount_before,
        )?;
        let owned_shares_amount_before: u64 = ctx.accounts.alloyx_vault_depository_shares.amount;
        let owned_shares_value_before: u64 = compute_value_for_shares_amount_floor(
            owned_shares_amount_before,
            total_shares_supply_before,
            total_shares_value_before,
        )?;

        // Compute withdrawn amount
        let collateral_amount_before_precision_loss = std::cmp::min(
            desired_collateral_amount,
            liquidity_collateral_amount_before,
        );
        let shares_amount: u64 = compute_shares_amount_for_value_floor(
            collateral_amount_before_precision_loss,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
        if shares_amount <= 0 {
            return Ok(0);
        }
        let collateral_amount_after_precision_loss: u64 = compute_value_for_shares_amount_floor(
            shares_amount,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
        if collateral_amount_after_precision_loss <= 0 {
            return Ok(0);
        }

        // Actually runs the CPI
        let alloyx_vault_info = ctx
            .accounts
            .alloyx_vault_depository
            .load()?
            .alloyx_vault_info;
        let collateral_mint = ctx.accounts.alloyx_vault_depository.load()?.collateral_mint;
        let alloyx_vault_depository_pda_signer: &[&[&[u8]]] = &[&[
            ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
            alloyx_vault_info.as_ref(),
            collateral_mint.as_ref(),
            &[ctx.accounts.alloyx_vault_depository.load()?.bump],
        ]];
        alloyx_cpi::cpi::withdraw(
            ctx.accounts
                .into_withdraw_from_alloyx_vault_context()
                .with_signer(alloyx_vault_depository_pda_signer),
            shares_amount,
        )?;
        token::transfer(
            ctx.accounts
                .into_transfer_alloyx_vault_depository_collateral_to_destination_collateral_context(
                    destination_collateral,
                )
                .with_signer(alloyx_vault_depository_pda_signer),
            collateral_amount_after_precision_loss,
        )?;

        // Reload onchain data
        destination_collateral.reload()?;
        ctx.accounts.alloyx_vault_depository_collateral.reload()?;
        ctx.accounts.alloyx_vault_depository_shares.reload()?;
        ctx.accounts.alloyx_vault_info.reload()?;
        ctx.accounts.alloyx_vault_collateral.reload()?;
        ctx.accounts.alloyx_vault_mint.reload()?;

        // Read onchain state after CPI
        let destination_collateral_amount_after: u64 = destination_collateral.amount;
        let alloyx_vault_depository_collateral_amount_after: u64 =
            ctx.accounts.alloyx_vault_depository_collateral.amount;
        let liquidity_collateral_amount_after: u64 = ctx.accounts.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount_after: u64 =
            ctx.accounts.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply_after: u64 = ctx.accounts.alloyx_vault_mint.supply;
        let total_shares_value_after: u64 = checked_add(
            liquidity_collateral_amount_after,
            outstanding_collateral_amount_after,
        )?;
        let owned_shares_amount_after: u64 = ctx.accounts.alloyx_vault_depository_shares.amount;
        let owned_shares_value_after: u64 = compute_value_for_shares_amount_floor(
            owned_shares_amount_after,
            total_shares_supply_after,
            total_shares_value_after,
        )?;

        // Compute funds movements
        let destination_collateral_amount_increase: u64 = compute_increase(
            destination_collateral_amount_before,
            destination_collateral_amount_after,
        )?;
        let total_shares_supply_decrease: u64 =
            compute_decrease(total_shares_supply_before, total_shares_supply_after)?;
        let total_shares_value_decrease: u64 =
            compute_decrease(total_shares_value_before, total_shares_value_after)?;
        let owned_shares_amount_decrease: u64 =
            compute_decrease(owned_shares_amount_before, owned_shares_amount_after)?;
        let owned_shares_value_decrease: u64 =
            compute_decrease(owned_shares_value_before, owned_shares_value_after)?;

        // Verify that everything went exactly like according to plan
        require!(
            destination_collateral_amount_increase == collateral_amount_after_precision_loss,
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
        Ok(collateral_amount_after_precision_loss)
    }

    // Deposit to alloyx's vault from the identity_depository_collateral,
    // on a best effort basis, while double-checking all the output results
    // returns the exact amount deposited
    pub(crate) fn deposit_to_alloyx_vault_from_identity_depository(
        &self,
        desired_collateral_amount: u64,
    ) -> Result<u64> {
        if desired_collateral_amount <= 0 {
            return Ok(0);
        }

        // Read onchain state before CPI
        let identity_depository_collateral_amount_before: u64 =
            ctx.accounts.identity_depository_collateral.amount;
        let alloyx_vault_depository_collateral_amount_before: u64 =
            ctx.accounts.alloyx_vault_depository_collateral.amount;
        let liquidity_collateral_amount_before: u64 = ctx.accounts.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount_before: u64 =
            ctx.accounts.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply_before: u64 = ctx.accounts.alloyx_vault_mint.supply;
        let total_shares_value_before: u64 = checked_add(
            liquidity_collateral_amount_before,
            outstanding_collateral_amount_before,
        )?;
        let owned_shares_amount_before: u64 = ctx.accounts.alloyx_vault_depository_shares.amount;
        let owned_shares_value_before: u64 = compute_value_for_shares_amount_floor(
            owned_shares_amount_before,
            total_shares_supply_before,
            total_shares_value_before,
        )?;

        // Compute deposited amount
        let collateral_amount_before_precision_loss = std::cmp::min(
            desired_collateral_amount,
            identity_depository_collateral_amount_before,
        );
        let shares_amount: u64 = compute_shares_amount_for_value_floor(
            collateral_amount_before_precision_loss,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
        if shares_amount <= 0 {
            return Ok(0);
        }
        let collateral_amount_after_precision_loss: u64 = compute_value_for_shares_amount_floor(
            shares_amount,
            total_shares_supply_before,
            total_shares_value_before,
        )?;
        if collateral_amount_after_precision_loss <= 0 {
            return Ok(0);
        }

        // Actually runs the CPI
        let alloyx_vault_info = ctx
            .accounts
            .alloyx_vault_depository
            .load()?
            .alloyx_vault_info;
        let collateral_mint = ctx.accounts.alloyx_vault_depository.load()?.collateral_mint;
        let identity_depository_pda_signer: &[&[&[u8]]] = &[&[
            IDENTITY_DEPOSITORY_NAMESPACE,
            &[ctx.accounts.identity_depository.load()?.bump],
        ]];
        let alloyx_vault_depository_pda_signer: &[&[&[u8]]] = &[&[
            ALLOYX_VAULT_DEPOSITORY_NAMESPACE,
            alloyx_vault_info.as_ref(),
            collateral_mint.as_ref(),
            &[ctx.accounts.alloyx_vault_depository.load()?.bump],
        ]];
        token::transfer(
            ctx.accounts
                .into_transfer_identity_depository_collateral_to_alloyx_vault_depository_collateral_context()
                .with_signer(identity_depository_pda_signer),
            collateral_amount_after_precision_loss,
        )?;
        alloyx_cpi::cpi::deposit(
            ctx.accounts
                .into_redeem_withdraw_request_from_alloyx_vault_context()
                .with_signer(alloyx_vault_depository_pda_signer),
            collateral_amount_after_precision_loss,
        )?;

        // Reload onchain data
        ctx.accounts.identity_depository_collateral.reload()?;
        ctx.accounts.alloyx_vault_depository_collateral.reload()?;
        ctx.accounts.alloyx_vault_depository_shares.reload()?;
        ctx.accounts.alloyx_vault_info.reload()?;
        ctx.accounts.alloyx_vault_collateral.reload()?;
        ctx.accounts.alloyx_vault_mint.reload()?;

        // Read onchain state after CPI
        let identity_depository_collateral_amount_after: u64 =
            ctx.accounts.identity_depository_collateral.amount;
        let alloyx_vault_depository_collateral_amount_after: u64 =
            ctx.accounts.alloyx_vault_depository_collateral.amount;
        let liquidity_collateral_amount_after: u64 = ctx.accounts.alloyx_vault_collateral.amount;
        let outstanding_collateral_amount_after: u64 =
            ctx.accounts.alloyx_vault_info.wallet_desk_usdc_value;
        let total_shares_supply_after: u64 = ctx.accounts.alloyx_vault_mint.supply;
        let total_shares_value_after: u64 = checked_add(
            liquidity_collateral_amount_after,
            outstanding_collateral_amount_after,
        )?;
        let owned_shares_amount_after: u64 = ctx.accounts.alloyx_vault_depository_shares.amount;
        let owned_shares_value_after: u64 = compute_value_for_shares_amount_floor(
            owned_shares_amount_after,
            total_shares_supply_after,
            total_shares_value_after,
        )?;

        // Compute funds movements
        let identity_depository_collateral_amount_decrease: u64 = compute_decrease(
            identity_depository_collateral_amount_before,
            identity_depository_collateral_amount_after,
        )?;
        let total_shares_supply_increase: u64 =
            compute_increase(total_shares_supply_before, total_shares_supply_after)?;
        let total_shares_value_increase: u64 =
            compute_increase(total_shares_value_before, total_shares_value_after)?;
        let owned_shares_amount_increase: u64 =
            compute_increase(owned_shares_amount_before, owned_shares_amount_after)?;
        let owned_shares_value_increase: u64 =
            compute_increase(owned_shares_value_before, owned_shares_value_after)?;

        // Verify that everything went exactly like according to plan
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

        // Return exactly how much was withdrawn
        Ok(collateral_amount_after_precision_loss)
    }
}

// Into functions
impl<'info> RedeemFromCredixLpDepository<'info> {
    pub fn into_transfer_alloyx_vault_depository_collateral_to_destination_collateral_context(
        &self,
        destination_collateral: Box<Account<'info, TokenAccount>>,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.alloyx_vault_depository_collateral.to_account_info(),
            to: destination_collateral.to_account_info(),
            authority: self.alloyx_vault_depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_identity_depository_collateral_to_alloyx_vault_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.identity_depository_collateral.to_account_info(),
            to: self.alloyx_vault_depository_collateral.to_account_info(),
            authority: self.identity_depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdraw_from_alloyx_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, alloyx_cpi::cpi::accounts::Withdraw<'info>> {
        let cpi_accounts = alloyx_cpi::cpi::accounts::Withdraw {
            base_token_mint: self.collateral_mint.to_account_info(),
            investor: self.depository.to_account_info(),
            investor_token_account: self.depository_collateral.to_account_info(),
            investor_lp_token_account: self.depository_shares.to_account_info(),
            program_state: self.credix_program_state.to_account_info(),
            global_market_state: self.credix_global_market_state.to_account_info(),
            signing_authority: self.credix_signing_authority.to_account_info(),
            liquidity_pool_token_account: self.credix_liquidity_collateral.to_account_info(),
            lp_token_mint: self.credix_shares_mint.to_account_info(),
            credix_pass: self.credix_pass.to_account_info(),
            treasury_pool_token_account: self.credix_treasury_pool_collateral.to_account_info(),
            credix_treasury: self.credix_treasury.to_account_info(),
            credix_treasury_token_account: self.credix_treasury_collateral.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_deposit_to_alloyx_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, alloyx_cpi::cpi::accounts::Deposit<'info>> {
        let cpi_accounts = alloyx_cpi::cpi::accounts::Deposit {
            base_token_mint: self.collateral_mint.to_account_info(),
            investor: self.depository.to_account_info(),
            investor_token_account: self.depository_collateral.to_account_info(),
            investor_lp_token_account: self.depository_shares.to_account_info(),
            program_state: self.credix_program_state.to_account_info(),
            global_market_state: self.credix_global_market_state.to_account_info(),
            signing_authority: self.credix_signing_authority.to_account_info(),
            liquidity_pool_token_account: self.credix_liquidity_collateral.to_account_info(),
            lp_token_mint: self.credix_shares_mint.to_account_info(),
            credix_pass: self.credix_pass.to_account_info(),
            treasury_pool_token_account: self.credix_treasury_pool_collateral.to_account_info(),
            credix_treasury: self.credix_treasury.to_account_info(),
            credix_treasury_token_account: self.credix_treasury_collateral.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
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
