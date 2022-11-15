use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use syrup_cpi::Nonce;

use crate::error::UxdError;
use crate::events::RedeemFromMaplePoolDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::maple_pool_depository::MaplePoolDepository;
use crate::utils::calculate_amount_less_fees;
use crate::utils::checked_i64_to_u64;
use crate::utils::compute_delta;
use crate::utils::compute_shares_amount_for_value;
use crate::utils::compute_value_for_shares_amount;
use crate::CONTROLLER_NAMESPACE;
use crate::MAPLE_POOL_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
#[instruction(redeemable_amount: u64)]
pub struct RedeemFromMaplePoolDepository<'info> {
    /// #1
    pub user: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.registered_maple_pool_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [MAPLE_POOL_DEPOSITORY_NAMESPACE, depository.load()?.maple_pool.key().as_ref(), depository.load()?.collateral_mint.as_ref()],
        bump = depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        has_one = depository_collateral @UxdError::InvalidCollateralLocker,
        has_one = maple_pool @UxdError::InvalidMaplePool,
        has_one = maple_pool_locker @UxdError::InvalidMaplePoolLocker,
        has_one = maple_globals @UxdError::InvalidMapleGlobals,
        has_one = maple_lender @UxdError::InvalidMapleLender,
        has_one = maple_shares_mint @UxdError::InvalidMapleSharesMint,
        has_one = maple_locked_shares @UxdError::InvalidMapleLockedShares,
        has_one = maple_lender_shares @UxdError::InvalidMapleLenderShares,
    )]
    pub depository: AccountLoader<'info, MaplePoolDepository>,

    /// #5
    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #6
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #7
    #[account(
        mut,
        constraint = user_redeemable.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidRedeemableMint,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #8
    #[account(
        mut,
        constraint = user_collateral.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_collateral.mint == collateral_mint.key() @UxdError::InvalidCollateralMint
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #9
    #[account(mut)]
    pub depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #10
    #[account(mut)]
    pub maple_pool: Box<Account<'info, syrup_cpi::Pool>>,
    /// #11
    #[account(mut)]
    pub maple_pool_locker: Box<Account<'info, TokenAccount>>,
    /// #12
    pub maple_globals: Box<Account<'info, syrup_cpi::Globals>>,
    /// #13
    #[account(mut)]
    pub maple_lender: Box<Account<'info, syrup_cpi::Lender>>,
    /// #14
    #[account(mut)]
    pub maple_shares_mint: Box<Account<'info, Mint>>,
    /// #15
    #[account(mut)]
    pub maple_locked_shares: Box<Account<'info, TokenAccount>>,
    /// #16
    #[account(mut)]
    pub maple_lender_shares: Box<Account<'info, TokenAccount>>,

    /// #17
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub maple_withdrawal_request: AccountInfo<'info>,
    /// #18
    /// CHECK: Does not need an ownership check because it is initialised by syrup and checked by syrup.
    #[account(mut)]
    pub maple_withdrawal_request_locker: AccountInfo<'info>,

    /// #19
    pub system_program: Program<'info, System>,
    /// #20
    pub token_program: Program<'info, Token>,
    /// #21
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #22
    pub syrup_program: Program<'info, syrup_cpi::program::Syrup>,
    /// #23
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RedeemFromMaplePoolDepository>, redeemable_amount: u64) -> Result<()> {
    // Read useful values
    let maple_pool = ctx.accounts.depository.load()?.maple_pool;
    let collateral_mint = ctx.accounts.depository.load()?.collateral_mint;
    let withdrawal_nonce = ctx.accounts.depository.load()?.withdrawal_nonce;

    // Make depository signer
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        MAPLE_POOL_DEPOSITORY_NAMESPACE,
        maple_pool.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.load()?.bump],
    ]];

    // Read all state before deposit
    let depository_collateral_amount_before: u64 = ctx.accounts.depository_collateral.amount;
    let user_redeemable_amount_before: u64 = ctx.accounts.user_redeemable.amount;
    let user_collateral_amount_before: u64 = ctx.accounts.user_collateral.amount;
    let pool_collateral_amount_before: u64 = ctx.accounts.maple_pool_locker.amount;
    let pool_shares_amount_before: u64 = ctx.accounts.maple_pool.shares_outstanding;
    let pool_shares_value_before: u64 = ctx.accounts.maple_pool.total_value;
    let locked_shares_amount_before: u64 = ctx.accounts.maple_locked_shares.amount;
    let lender_shares_amount_before: u64 = ctx.accounts.maple_lender_shares.amount;
    let owned_shares_amount_before: u64 = ctx
        .accounts
        .compute_owned_shares_amount(locked_shares_amount_before, lender_shares_amount_before)?;
    let owned_shares_value_before: u64 = compute_value_for_shares_amount(
        owned_shares_amount_before,
        pool_shares_amount_before,
        pool_shares_value_before,
    )?;

    // Add some pool state log information
    msg!(
        "[redeem_from_maple_pool_depository:depository_collateral_amount_before:{}]",
        depository_collateral_amount_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:user_redeemable_amount_before:{}]",
        user_redeemable_amount_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:user_collateral_amount_before:{}]",
        user_collateral_amount_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:pool_collateral_amount_before:{}]",
        pool_collateral_amount_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:pool_shares_amount_before:{}]",
        pool_shares_amount_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:pool_shares_value_before:{}]",
        pool_shares_value_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:locked_shares_amount_before:{}]",
        locked_shares_amount_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:lender_shares_amount_before:{}]",
        lender_shares_amount_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:owned_shares_amount_before:{}]",
        owned_shares_amount_before
    );
    msg!(
        "[redeem_from_maple_pool_depository:owned_shares_value_before:{}]",
        owned_shares_value_before
    );

    // Calculate the amount of collateral we want to withdraw based on the redeemable amount
    let redeemable_amount_after_fees = calculate_amount_less_fees(
        redeemable_amount,
        ctx.accounts.depository.load()?.redeeming_fee_in_bps,
    )?;
    let collateral_amount_before_precision_loss = redeemable_amount_after_fees; // assume 1:1 on purpose
    require!(
        collateral_amount_before_precision_loss > 0,
        UxdError::MinimumRedeemedCollateralAmountError
    );
    msg!(
        "[redeem_from_maple_pool_depository:collateral_amount_before_precision_loss:{}]",
        collateral_amount_before_precision_loss
    );

    // Compute the amount of shares that we need to withdraw based on the amount of wanted collateral
    let shares_amount = compute_shares_amount_for_value(
        collateral_amount_before_precision_loss,
        pool_shares_amount_before,
        pool_shares_value_before,
    )?;
    msg!(
        "[redeem_from_maple_pool_depository:shares_amount:{}]",
        shares_amount
    );

    // Compute the amount of collateral that the withdrawn shares are worth (after potential precision loss)
    let collateral_amount_after_precision_loss = compute_value_for_shares_amount(
        shares_amount,
        pool_shares_amount_before,
        pool_shares_value_before,
    )?;
    require!(
        collateral_amount_after_precision_loss > 0,
        UxdError::MinimumRedeemedCollateralAmountError
    );
    msg!(
        "[redeem_from_maple_pool_depository:collateral_amount_after_precision_loss:{}]",
        collateral_amount_after_precision_loss
    );

    // Burn the user's redeemable
    msg!(
        "[redeem_from_maple_pool_depository:redeemable_burn:{}]",
        redeemable_amount
    );
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_amount,
    )?;

    // If we don't have enough unlocked shares, we'll need to try to unlock some (may fail if the deposit lockup period is not passed)
    if lender_shares_amount_before < shares_amount {
        msg!("[redeem_from_maple_pool_depository:lender_unlock_deposit]");
        syrup_cpi::cpi::lender_unlock_deposit(
            ctx.accounts
                .into_lender_unlock_deposit_from_maple_pool_context()
                .with_signer(depository_pda_signer),
        )?;
    }

    // Run a full withdrawal request (init/exec/close) through syrup
    msg!("[redeem_from_maple_pool_depository:withdrawal_request_initialize]",);
    syrup_cpi::cpi::withdrawal_request_initialize(
        ctx.accounts
            .into_withdrawal_request_initialize_from_maple_pool_context()
            .with_signer(depository_pda_signer),
        Nonce {
            value: withdrawal_nonce.to_be_bytes().clone(),
        },
        shares_amount,
    )?;
    msg!("[redeem_from_maple_pool_depository:withdrawal_request_execute]",);
    syrup_cpi::cpi::withdrawal_request_execute(
        ctx.accounts
            .into_withdrawal_request_execute_from_maple_pool_context()
            .with_signer(depository_pda_signer),
        shares_amount,
    )?;
    msg!("[redeem_from_maple_pool_depository:withdrawal_request_close]");
    syrup_cpi::cpi::withdrawal_request_close(
        ctx.accounts
            .into_withdrawal_request_close_from_maple_pool_context()
            .with_signer(depository_pda_signer),
    )?;

    // Transfer the received collateral from the depository to the end user
    msg!("[redeem_from_maple_pool_depository:collateral_transfer]",);
    token::transfer(
        ctx.accounts
            .into_transfer_depository_collateral_to_user_collateral_context()
            .with_signer(depository_pda_signer),
        collateral_amount_after_precision_loss,
    )?;

    // Refresh account states after deposit
    ctx.accounts.depository_collateral.reload()?;
    ctx.accounts.user_redeemable.reload()?;
    ctx.accounts.user_collateral.reload()?;
    ctx.accounts.maple_pool_locker.reload()?;
    ctx.accounts.maple_pool.reload()?;
    ctx.accounts.maple_locked_shares.reload()?;
    ctx.accounts.maple_lender_shares.reload()?;

    // Read all states after deposit
    let depository_collateral_amount_after: u64 = ctx.accounts.depository_collateral.amount;
    let user_redeemable_amount_after: u64 = ctx.accounts.user_redeemable.amount;
    let user_collateral_amount_after: u64 = ctx.accounts.user_collateral.amount;
    let pool_collateral_amount_after: u64 = ctx.accounts.maple_pool_locker.amount;
    let pool_shares_amount_after: u64 = ctx.accounts.maple_pool.shares_outstanding;
    let pool_shares_value_after: u64 = ctx.accounts.maple_pool.total_value;
    let locked_shares_amount_after: u64 = ctx.accounts.maple_locked_shares.amount;
    let lender_shares_amount_after: u64 = ctx.accounts.maple_lender_shares.amount;
    let owned_shares_amount_after: u64 = ctx
        .accounts
        .compute_owned_shares_amount(locked_shares_amount_after, lender_shares_amount_after)?;
    let owned_shares_value_after: u64 = compute_value_for_shares_amount(
        owned_shares_amount_after,
        pool_shares_amount_after,
        pool_shares_value_after,
    )?;

    // Add some pool state log information
    msg!(
        "[redeem_from_maple_pool_depository:depository_collateral_amount_after:{}]",
        depository_collateral_amount_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:user_redeemable_amount_after:{}]",
        user_redeemable_amount_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:user_collateral_amount_after:{}]",
        user_collateral_amount_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:pool_collateral_amount_after:{}]",
        pool_collateral_amount_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:pool_shares_amount_after:{}]",
        pool_shares_amount_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:pool_shares_value_after:{}]",
        pool_shares_value_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:locked_shares_amount_after:{}]",
        locked_shares_amount_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:lender_shares_amount_after:{}]",
        lender_shares_amount_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:owned_shares_amount_after:{}]",
        owned_shares_amount_after
    );
    msg!(
        "[redeem_from_maple_pool_depository:owned_shares_value_after:{}]",
        owned_shares_value_after
    );

    // Compute changes in states
    let depository_collateral_delta: i64 = compute_delta(
        depository_collateral_amount_before,
        depository_collateral_amount_after,
    )?;
    let user_redeemable_amount_delta: i64 =
        compute_delta(user_redeemable_amount_before, user_redeemable_amount_after)?;
    let user_collateral_amount_delta: i64 =
        compute_delta(user_collateral_amount_before, user_collateral_amount_after)?;
    let pool_collateral_amount_delta: i64 =
        compute_delta(pool_collateral_amount_before, pool_collateral_amount_after)?;
    let pool_shares_amount_delta: i64 =
        compute_delta(pool_shares_amount_before, pool_shares_amount_after)?;
    let pool_shares_value_delta: i64 =
        compute_delta(pool_shares_value_before, pool_shares_value_after)?;
    let owned_shares_amount_delta: i64 =
        compute_delta(owned_shares_amount_before, owned_shares_amount_after)?;
    let owned_shares_value_delta: i64 =
        compute_delta(owned_shares_value_before, owned_shares_value_after)?;

    // The depository collateral account should always be empty
    require!(
        depository_collateral_delta == 0,
        UxdError::CollateralWithdrawalHasRemainingDust
    );

    // Validate the redeem was successful and meaningful
    require!(
        user_redeemable_amount_delta < 0,
        UxdError::CollateralWithdrawalUnaccountedFor
    );
    require!(
        user_collateral_amount_delta > 0,
        UxdError::CollateralWithdrawalUnaccountedFor
    );
    require!(
        pool_collateral_amount_delta < 0,
        UxdError::CollateralWithdrawalUnaccountedFor
    );
    require!(
        pool_shares_amount_delta < 0,
        UxdError::CollateralWithdrawalUnaccountedFor
    );
    require!(
        pool_shares_value_delta < 0,
        UxdError::CollateralWithdrawalUnaccountedFor
    );
    require!(
        owned_shares_amount_delta < 0,
        UxdError::CollateralWithdrawalUnaccountedFor
    );
    require!(
        owned_shares_value_delta < 0,
        UxdError::CollateralWithdrawalUnaccountedFor
    );

    // Because we know the direction of the change, we can use the unsigned values now
    let user_redeemable_amount_decrease = checked_i64_to_u64(-user_redeemable_amount_delta)?;
    let user_collateral_amount_increase = checked_i64_to_u64(user_collateral_amount_delta)?;
    let pool_collateral_amount_decrease = checked_i64_to_u64(-pool_collateral_amount_delta)?;
    let pool_shares_amount_decrease = checked_i64_to_u64(-pool_shares_amount_delta)?;
    let pool_shares_value_decrease = checked_i64_to_u64(-pool_shares_value_delta)?;
    let owned_shares_amount_decrease = checked_i64_to_u64(-owned_shares_amount_delta)?;
    let owned_shares_value_decrease = checked_i64_to_u64(-owned_shares_value_delta)?;

    // Validate that we didnt return too much collateral
    require!(
        user_redeemable_amount_decrease >= user_collateral_amount_increase,
        UxdError::CollateralWithdrawalAmountsDoesntMatch,
    );

    // Validate that the collateral value moved exactly to the correct place
    require!(
        user_collateral_amount_increase == collateral_amount_after_precision_loss,
        UxdError::CollateralWithdrawalAmountsDoesntMatch,
    );
    require!(
        pool_collateral_amount_decrease == collateral_amount_after_precision_loss,
        UxdError::CollateralWithdrawalAmountsDoesntMatch,
    );
    require!(
        pool_shares_value_decrease == collateral_amount_after_precision_loss,
        UxdError::CollateralWithdrawalAmountsDoesntMatch,
    );

    // Check that we received the correct amount of shares
    require!(
        owned_shares_amount_decrease == pool_shares_amount_decrease,
        UxdError::CollateralWithdrawalDoesntMatchTokenValue,
    );

    // Check that the shares we received match the collateral value
    require!(
        owned_shares_value_decrease == collateral_amount_after_precision_loss,
        UxdError::CollateralWithdrawalDoesntMatchTokenValue
    );

    // Compute how much fees was paid
    let redeemable_amount_delta = compute_delta(redeemable_amount, redeemable_amount_after_fees)?;
    let redeeming_fee_paid = checked_i64_to_u64(-redeemable_amount_delta)?;

    // Emit event
    emit!(RedeemFromMaplePoolDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        redeemable_amount: redeemable_amount,
        collateral_amount: collateral_amount_after_precision_loss,
        redeeming_fee_paid: redeeming_fee_paid,
    });

    // Accouting for depository
    let mut depository = ctx.accounts.depository.load_mut()?;
    depository.redeeming_fee_accrued(redeeming_fee_paid)?;
    depository.collateral_withdrawn_and_redeemable_burned(
        collateral_amount_after_precision_loss,
        redeemable_amount,
    )?;
    depository.increment_withdrawal_nonce()?;

    // Accouting for controller
    ctx.accounts
        .controller
        .load_mut()?
        .update_onchain_accounting_following_mint_or_redeem(redeemable_amount_after_fees.into())?;

    // Done
    Ok(())
}

// Into functions
impl<'info> RedeemFromMaplePoolDepository<'info> {
    pub fn into_lender_unlock_deposit_from_maple_pool_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, syrup_cpi::cpi::accounts::LenderUnlockDeposit<'info>> {
        let cpi_accounts = syrup_cpi::cpi::accounts::LenderUnlockDeposit {
            globals: self.maple_globals.to_account_info(),
            pool: self.maple_pool.to_account_info(),
            lender: self.maple_lender.to_account_info(),
            lender_user: self.depository.to_account_info(),
            locked_shares: self.maple_locked_shares.to_account_info(),
            lender_shares: self.maple_lender_shares.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.syrup_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdrawal_request_initialize_from_maple_pool_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, syrup_cpi::cpi::accounts::WithdrawalRequestInitialize<'info>>
    {
        let cpi_accounts = syrup_cpi::cpi::accounts::WithdrawalRequestInitialize {
            globals: self.maple_globals.to_account_info(),
            pool: self.maple_pool.to_account_info(),
            lender: self.maple_lender.to_account_info(),
            lender_owner: self.depository.to_account_info(),
            lender_share_account: self.maple_lender_shares.to_account_info(),
            shares_mint: self.maple_shares_mint.to_account_info(),
            withdrawal_request: self.maple_withdrawal_request.to_account_info(),
            withdrawal_request_locker: self.maple_withdrawal_request_locker.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.syrup_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdrawal_request_execute_from_maple_pool_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, syrup_cpi::cpi::accounts::WithdrawalRequestExecute<'info>>
    {
        let cpi_accounts = syrup_cpi::cpi::accounts::WithdrawalRequestExecute {
            globals: self.maple_globals.to_account_info(),
            pool: self.maple_pool.to_account_info(),
            pool_locker: self.maple_pool_locker.to_account_info(),
            lender: self.maple_lender.to_account_info(),
            lender_owner: self.depository.to_account_info(),
            lender_locker: self.depository_collateral.to_account_info(),
            lender_share_account: self.maple_lender_shares.to_account_info(),
            shares_mint: self.maple_shares_mint.to_account_info(),
            withdrawal_request: self.maple_withdrawal_request.to_account_info(),
            withdrawal_request_locker: self.maple_withdrawal_request_locker.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.syrup_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_withdrawal_request_close_from_maple_pool_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, syrup_cpi::cpi::accounts::WithdrawalRequestClose<'info>>
    {
        let cpi_accounts = syrup_cpi::cpi::accounts::WithdrawalRequestClose {
            globals: self.maple_globals.to_account_info(),
            pool: self.maple_pool.to_account_info(),
            lender: self.maple_lender.to_account_info(),
            lender_owner: self.depository.to_account_info(),
            lender_share_account: self.maple_lender_shares.to_account_info(),
            withdrawal_request: self.maple_withdrawal_request.to_account_info(),
            withdrawal_request_locker: self.depository_collateral.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.syrup_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_depository_collateral_to_user_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.depository_collateral.to_account_info(),
            to: self.user_collateral.to_account_info(),
            authority: self.depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_burn_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            from: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Compute maths functions
impl<'info> RedeemFromMaplePoolDepository<'info> {
    pub fn compute_owned_shares_amount(
        &self,
        locked_shares_amount: u64,
        lender_shares_amount: u64,
    ) -> Result<u64> {
        Ok(locked_shares_amount
            .checked_add(lender_shares_amount)
            .ok_or(UxdError::MathError)?)
    }
}

// Validate
impl<'info> RedeemFromMaplePoolDepository<'info> {
    pub fn validate(&self, redeemable_amount: u64) -> Result<()> {
        require!(redeemable_amount > 0, UxdError::InvalidRedeemableAmount);
        Ok(())
    }
}
