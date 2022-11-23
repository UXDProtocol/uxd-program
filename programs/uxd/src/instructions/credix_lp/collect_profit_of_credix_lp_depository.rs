use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::error::UxdError;
use crate::events::CollectProfitOfCredixLpDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::utils::checked_i64_to_u64;
use crate::utils::compute_amount_fraction;
use crate::utils::compute_delta;
use crate::utils::compute_shares_amount_for_value;
use crate::utils::compute_value_for_shares_amount;
use crate::utils::is_within_range_inclusive;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
pub struct CollectProfitOfCredixLpDepository<'info> {
    /// #1 Authored call accessible only to the signer matching Controller.authority
    pub authority: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_credix_lp_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = authority @UxdError::InvalidAuthority,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [
            CREDIX_LP_DEPOSITORY_NAMESPACE,
            depository.load()?.credix_global_market_state.key().as_ref(),
            depository.load()?.collateral_mint.as_ref()
        ],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = depository_collateral @UxdError::InvalidCollateralLocker,
        has_one = depository_shares @UxdError::InvalidSharesLocker,
        has_one = credix_program_state @UxdError::InvalidCredixProgramState,
        has_one = credix_global_market_state @UxdError::InvalidCredixGlobalMarketState,
        has_one = credix_signing_authority @UxdError::InvalidCredixSigningAuthority,
        has_one = credix_liquidity_collateral @UxdError::InvalidCredixLiquidityCollateral,
        has_one = credix_shares_mint @UxdError::InvalidCredixSharesMint,
        has_one = profit_treasury_collateral @UxdError::InvalidTreasuryCollateral,
    )]
    pub depository: AccountLoader<'info, CredixLpDepository>,

    /// #6
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #9
    #[account(mut)]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(mut)]
    pub depository_shares: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(
        constraint = credix_program_state.credix_multisig_key == credix_multisig.key() @UxdError::InvalidCredixMultisig,
    )]
    pub credix_program_state: Box<Account<'info, credix_client::ProgramState>>,

    /// #12
    #[account(
        mut,
        constraint = credix_global_market_state.treasury_pool_token_account == credix_treasury_collateral.key() @UxdError::InvalidCredixTreasuryCollateral,
    )]
    pub credix_global_market_state: Box<Account<'info, credix_client::GlobalMarketState>>,

    /// #13
    /// CHECK: unused by us, checked by credix
    pub credix_signing_authority: AccountInfo<'info>,

    /// #14
    #[account(mut)]
    pub credix_liquidity_collateral: Box<Account<'info, TokenAccount>>,

    /// #15
    #[account(mut)]
    pub credix_shares_mint: Box<Account<'info, Mint>>,

    /// #16
    #[account(
        mut,
        constraint = credix_pass.user == depository.key() @UxdError::InvalidCredixPass,
    )]
    pub credix_pass: Box<Account<'info, credix_client::CredixPass>>,

    /// #17
    #[account(
        mut,
        token::mint = collateral_mint,
    )]
    pub credix_treasury_collateral: Box<Account<'info, TokenAccount>>,

    /// #18
    /// CHECK: not used by us, checked by credix program
    pub credix_multisig: AccountInfo<'info>,

    /// #19
    #[account(
        mut,
        token::authority = credix_multisig,
        token::mint = collateral_mint,
    )]
    pub credix_multisig_collateral: Box<Account<'info, TokenAccount>>,

    /// #20
    #[account(mut)]
    pub profit_treasury_collateral: Box<Account<'info, TokenAccount>>,

    /// #21
    pub system_program: Program<'info, System>,
    /// #22
    pub token_program: Program<'info, Token>,
    /// #23
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #24
    pub credix_program: Program<'info, credix_client::program::Credix>,
    /// #25
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CollectProfitOfCredixLpDepository>) -> Result<()> {
    // Read useful values
    let credix_global_market_state = ctx.accounts.depository.load()?.credix_global_market_state;
    let collateral_mint = ctx.accounts.depository.load()?.collateral_mint;

    // Make depository signer
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        CREDIX_LP_DEPOSITORY_NAMESPACE,
        credix_global_market_state.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.load()?.bump],
    ]];

    // Read all states before collect
    let depository_collateral_amount_before: u64 = ctx.accounts.depository_collateral.amount;
    let profit_treasury_collateral_amount_before: u64 =
        ctx.accounts.profit_treasury_collateral.amount;

    let liquidity_collateral_amount_before: u64 = ctx.accounts.credix_liquidity_collateral.amount;
    let outstanding_collateral_amount_before: u64 = ctx
        .accounts
        .credix_global_market_state
        .pool_outstanding_credit;

    let total_shares_amount_before: u64 = ctx.accounts.credix_shares_mint.supply;
    let total_shares_value_before: u64 = liquidity_collateral_amount_before
        .checked_add(outstanding_collateral_amount_before)
        .ok_or(UxdError::MathError)?;

    let owned_shares_amount_before: u64 = ctx.accounts.depository_shares.amount;
    let owned_shares_value_before: u64 = compute_value_for_shares_amount(
        owned_shares_amount_before,
        total_shares_amount_before,
        total_shares_value_before,
    )?;

    // Compute the amount of owed to the users
    let liabilities_redeemable_amount: u128 = ctx
        .accounts
        .depository
        .load()?
        .redeemable_amount_under_management;
    msg!(
        "[collect_profit_of_credix_lp_depository:liabilities_redeemable_amount:{}]",
        liabilities_redeemable_amount
    );

    // Assumes and enforce a collateral/redeemable 1:1 relationship on purpose
    let assets_redeemable_amount: u128 = owned_shares_value_before.into();
    msg!(
        "[collect_profit_of_credix_lp_depository:assets_redeemable_amount:{}]",
        assets_redeemable_amount
    );

    // Compute the amount of profits that we can safely withdraw
    let profits_redeemable_amount: u128 = assets_redeemable_amount
        .checked_sub(liabilities_redeemable_amount)
        .ok_or(UxdError::MathError)?;
    msg!(
        "[collect_profit_of_credix_lp_depository:profits_redeemable_amount:{}]",
        profits_redeemable_amount
    );

    // Assumes and enforce a collateral/redeemable 1:1 relationship on purpose
    let collateral_amount_before_precision_loss: u64 = u64::try_from(profits_redeemable_amount)
        .ok()
        .ok_or(UxdError::MathError)?;
    msg!(
        "[collect_profit_of_credix_lp_depository:collateral_amount_before_precision_loss:{}]",
        collateral_amount_before_precision_loss
    );

    // Compute the amount of shares that we need to withdraw based on the amount of wanted collateral
    let shares_amount: u64 = compute_shares_amount_for_value(
        collateral_amount_before_precision_loss,
        total_shares_amount_before,
        total_shares_value_before,
    )?;
    msg!(
        "[collect_profit_of_credix_lp_depository:shares_amount:{}]",
        shares_amount
    );

    // Compute the amount of collateral that the withdrawn shares are worth (after potential precision loss)
    let collateral_amount_after_precision_loss: u64 = compute_value_for_shares_amount(
        shares_amount,
        total_shares_amount_before,
        total_shares_value_before,
    )?;
    msg!(
        "[collect_profit_of_credix_lp_depository:collateral_amount_after_precision_loss:{}]",
        collateral_amount_after_precision_loss
    );

    // Compute the amount of collateral we will receive after the withdrawal fees
    let withdrawal_fees_fraction = ctx.accounts.credix_global_market_state.withdrawal_fee;
    let withdrawal_fees_amount: u64 = compute_amount_fraction(
        collateral_amount_after_precision_loss,
        withdrawal_fees_fraction.numerator.into(),
        withdrawal_fees_fraction.denominator.into(),
    )?;
    let collateral_amount_after_withdrawal_fees: u64 = collateral_amount_after_precision_loss
        .checked_sub(withdrawal_fees_amount)
        .ok_or(UxdError::MathError)?;
    msg!(
        "[collect_profit_of_credix_lp_depository:collateral_amount_after_withdrawal_fees:{}]",
        collateral_amount_after_withdrawal_fees
    );

    // If nothing to withdraw, no need to continue
    if collateral_amount_after_withdrawal_fees == 0 {
        msg!("[collect_profit_of_credix_lp_depository:no_profit]",);
        return Ok(());
    }

    // Run a withdraw CPI from credix into the depository
    msg!("[collect_profit_of_credix_lp_depository:withdraw_funds]",);
    credix_client::cpi::withdraw_funds(
        ctx.accounts
            .into_withdraw_funds_from_credix_lp_context()
            .with_signer(depository_pda_signer),
        collateral_amount_before_precision_loss,
    )?;

    // Transfer the received collateral from the depository to the end user
    msg!("[collect_profit_of_credix_lp_depository:collateral_transfer]",);
    token::transfer(
        ctx.accounts
            .into_transfer_depository_collateral_to_profit_treasury_collateral_context()
            .with_signer(depository_pda_signer),
        collateral_amount_after_withdrawal_fees,
    )?;

    // Refresh account states after withdrawal
    ctx.accounts.depository_collateral.reload()?;
    ctx.accounts.depository_shares.reload()?;
    ctx.accounts.credix_global_market_state.reload()?;
    ctx.accounts.credix_liquidity_collateral.reload()?;
    ctx.accounts.credix_shares_mint.reload()?;
    ctx.accounts.profit_treasury_collateral.reload()?;

    // Read all states after withdrawal
    let depository_collateral_amount_after: u64 = ctx.accounts.depository_collateral.amount;
    let profit_treasury_collateral_amount_after: u64 =
        ctx.accounts.profit_treasury_collateral.amount;

    let liquidity_collateral_amount_after: u64 = ctx.accounts.credix_liquidity_collateral.amount;
    let outstanding_collateral_amount_after: u64 = ctx
        .accounts
        .credix_global_market_state
        .pool_outstanding_credit;

    let total_shares_amount_after: u64 = ctx.accounts.credix_shares_mint.supply;
    let total_shares_value_after: u64 = liquidity_collateral_amount_after
        .checked_add(outstanding_collateral_amount_after)
        .ok_or(UxdError::MathError)?;

    let owned_shares_amount_after: u64 = ctx.accounts.depository_shares.amount;
    let owned_shares_value_after: u64 = compute_value_for_shares_amount(
        owned_shares_amount_after,
        total_shares_amount_after,
        total_shares_value_after,
    )?;

    // Compute changes in states
    let depository_collateral_delta: i64 = compute_delta(
        depository_collateral_amount_before,
        depository_collateral_amount_after,
    )?;
    let profit_treasury_collateral_amount_delta: i64 = compute_delta(
        profit_treasury_collateral_amount_before,
        profit_treasury_collateral_amount_after,
    )?;

    let total_shares_amount_delta: i64 =
        compute_delta(total_shares_amount_before, total_shares_amount_after)?;
    let total_shares_value_delta: i64 =
        compute_delta(total_shares_value_before, total_shares_value_after)?;

    let owned_shares_amount_delta: i64 =
        compute_delta(owned_shares_amount_before, owned_shares_amount_after)?;
    let owned_shares_value_delta: i64 =
        compute_delta(owned_shares_value_before, owned_shares_value_after)?;

    // The depository collateral account should always be empty
    require!(
        depository_collateral_delta == 0,
        UxdError::CollateralDepositHasRemainingDust
    );

    // Validate the withdrawal was successful and meaningful
    require!(
        profit_treasury_collateral_amount_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        total_shares_amount_delta < 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        total_shares_value_delta < 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        owned_shares_amount_delta < 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        owned_shares_value_delta < 0,
        UxdError::CollateralDepositUnaccountedFor
    );

    // Because we know the direction of the change, we can use the unsigned values now
    let profit_treasury_collateral_amount_increase: u64 =
        checked_i64_to_u64(profit_treasury_collateral_amount_delta)?;
    let total_shares_amount_decrease: u64 = checked_i64_to_u64(-total_shares_amount_delta)?;
    let total_shares_value_decrease: u64 = checked_i64_to_u64(-total_shares_value_delta)?;
    let owned_shares_amount_decrease: u64 = checked_i64_to_u64(-owned_shares_amount_delta)?;
    let owned_shares_value_decrease: u64 = checked_i64_to_u64(-owned_shares_value_delta)?;

    // Validate that the locked value moved exactly to the correct place
    require!(
        profit_treasury_collateral_amount_increase == collateral_amount_after_withdrawal_fees,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that we withdrew the correct amount of shares
    require!(
        owned_shares_amount_decrease == shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        total_shares_amount_decrease == shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that the new state of the pool still makes sense
    require!(
        is_within_range_inclusive(
            owned_shares_value_decrease,
            collateral_amount_after_precision_loss,
            collateral_amount_before_precision_loss,
        ),
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        is_within_range_inclusive(
            total_shares_value_decrease,
            collateral_amount_after_precision_loss,
            collateral_amount_before_precision_loss,
        ),
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Emit event
    emit!(CollectProfitOfCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        profit_treasury_collateral: ctx.accounts.profit_treasury_collateral.key(),
        collateral_amount_before_fees: collateral_amount_before_precision_loss,
        collateral_amount_after_fees: collateral_amount_after_withdrawal_fees,
    });

    // Accouting for depository
    let mut depository = ctx.accounts.depository.load_mut()?;
    depository.profit_treasury_collected(collateral_amount_after_withdrawal_fees)?;

    // Done
    Ok(())
}

// Into functions
impl<'info> CollectProfitOfCredixLpDepository<'info> {
    pub fn into_withdraw_funds_from_credix_lp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, credix_client::cpi::accounts::WithdrawFunds<'info>> {
        let cpi_accounts = credix_client::cpi::accounts::WithdrawFunds {
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
            treasury_pool_token_account: self.credix_treasury_collateral.to_account_info(),
            credix_multisig_key: self.credix_multisig.to_account_info(),
            credix_multisig_token_account: self.credix_multisig_collateral.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_depository_collateral_to_profit_treasury_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.depository_collateral.to_account_info(),
            to: self.profit_treasury_collateral.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> CollectProfitOfCredixLpDepository<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}
