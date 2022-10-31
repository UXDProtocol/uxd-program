use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use fixed::types::I80F48;

use crate::error::UxdError;
use crate::events::MintWithMaplePoolDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::maple_pool_depository::MaplePoolDepository;
use crate::utils::calculate_amount_less_fees;
use crate::utils::checked_i64_to_u64;
use crate::utils::compute_delta;
use crate::utils::is_equal_with_precision_loss;
use crate::CONTROLLER_NAMESPACE;
use crate::MAPLE_POOL_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
#[instruction(collateral_amount: u64)]
pub struct MintWithMaplePoolDepository<'info> {
    /// #1
    #[account()]
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
    #[account()]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #7
    #[account(
        mut,
        constraint = user_redeemable.owner == user.key()  @UxdError::InvalidOwner,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidRedeemableMint,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #8
    #[account(
        mut,
        constraint = user_collateral.owner == user.key()  @UxdError::InvalidOwner,
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
    #[account()]
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
    pub system_program: Program<'info, System>,
    /// #18
    pub token_program: Program<'info, Token>,
    /// #19
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #20
    pub syrup_program: Program<'info, syrup_cpi::program::Syrup>,
    /// #21
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<MintWithMaplePoolDepository>, collateral_amount: u64) -> Result<()> {
    // Read useful keys
    let maple_pool = ctx.accounts.depository.load()?.maple_pool;
    let collateral_mint = ctx.accounts.depository.load()?.collateral_mint;

    // Make controller signer
    let controller_pda_signer: &[&[&[u8]]] = &[&[
        CONTROLLER_NAMESPACE,
        &[ctx.accounts.controller.load()?.bump],
    ]];

    // Make depository signer
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        MAPLE_POOL_DEPOSITORY_NAMESPACE,
        maple_pool.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.load()?.bump],
    ]];

    // We will try to deposit the exact amount requested by the user
    let collateral_amount_deposited = collateral_amount;

    // Read all state before deposit
    msg!("[mint_with_maple_pool_depository:before_math]");
    let depository_collateral_amount_before: u64 = ctx.accounts.depository_collateral.amount;
    let user_collateral_amount_before: u64 = ctx.accounts.user_collateral.amount;
    let pool_collateral_amount_before: u64 = ctx.accounts.maple_pool_locker.amount;
    let pool_shares_amount_before: u64 = ctx.accounts.maple_pool.shares_outstanding;
    let pool_shares_value_before: u64 = ctx.accounts.maple_pool.total_value;
    let locked_shares_amount_before: u64 = ctx.accounts.maple_locked_shares.amount;
    let lender_shares_amount_before: u64 = ctx.accounts.maple_lender_shares.amount;
    let owned_shares_amount_before: u64 = ctx
        .accounts
        .compute_owned_shares_amount(locked_shares_amount_before, lender_shares_amount_before)?;
    let owned_shares_value_before: u64 = ctx.accounts.compute_shares_value(
        owned_shares_amount_before,
        pool_shares_amount_before,
        pool_shares_value_before,
    )?;

    // Transfer the collateral to an account owned by the depository
    msg!("[mint_with_maple_pool_depository:transfer_to_depository]");
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_depository_collateral_context(),
        collateral_amount_deposited,
    )?;

    // Do the deposit by placing collateral owned by the depository into the pool
    msg!("[mint_with_maple_pool_depository:lender_deposit]");
    syrup_cpi::cpi::lender_deposit(
        ctx.accounts
            .into_deposit_collateral_to_maple_pool_context()
            .with_signer(depository_pda_signer),
        collateral_amount_deposited,
    )?;

    // Refresh account states after deposit
    msg!("[mint_with_maple_pool_depository:reload_after_result]");
    ctx.accounts.depository_collateral.reload()?;
    ctx.accounts.user_collateral.reload()?;
    ctx.accounts.maple_pool_locker.reload()?;
    ctx.accounts.maple_pool.reload()?;
    ctx.accounts.maple_locked_shares.reload()?;
    ctx.accounts.maple_lender_shares.reload()?;

    // Read all states after deposit
    msg!("[mint_with_maple_pool_depository:after_math]");
    let depository_collateral_amount_after: u64 = ctx.accounts.depository_collateral.amount;
    let user_collateral_amount_after: u64 = ctx.accounts.user_collateral.amount;
    let pool_collateral_amount_after: u64 = ctx.accounts.maple_pool_locker.amount;
    let pool_shares_amount_after: u64 = ctx.accounts.maple_pool.shares_outstanding;
    let pool_shares_value_after: u64 = ctx.accounts.maple_pool.total_value;
    let locked_shares_amount_after: u64 = ctx.accounts.maple_locked_shares.amount;
    let lender_shares_amount_after: u64 = ctx.accounts.maple_lender_shares.amount;
    let owned_shares_amount_after: u64 = ctx
        .accounts
        .compute_owned_shares_amount(locked_shares_amount_after, lender_shares_amount_after)?;
    let owned_shares_value_after: u64 = ctx.accounts.compute_shares_value(
        owned_shares_amount_after,
        pool_shares_amount_after,
        pool_shares_value_after,
    )?;

    // Compute changes in states
    msg!("[mint_with_maple_pool_depository:compute_deltas]");
    let depository_collateral_delta: i64 = compute_delta(
        depository_collateral_amount_before,
        depository_collateral_amount_after,
    )?;
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
    msg!("[mint_with_maple_pool_depository:check_dust]");
    require!(
        depository_collateral_delta == 0,
        UxdError::CollateralDepositHasRemainingDust
    );

    // Validate the deposit was successful and meaningful
    msg!("[mint_with_maple_pool_depository:check_changes]");
    require!(
        user_collateral_amount_delta < 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        pool_collateral_amount_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        pool_shares_amount_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        pool_shares_value_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        owned_shares_amount_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        owned_shares_value_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );

    // Because we know the direction of the change, we can use the unsigned values now
    let user_collateral_amount_decrease = checked_i64_to_u64(-user_collateral_amount_delta)?;
    let pool_collateral_amount_increase = checked_i64_to_u64(pool_collateral_amount_delta)?;
    let pool_shares_amount_increase = checked_i64_to_u64(pool_shares_amount_delta)?;
    let pool_shares_value_increase = checked_i64_to_u64(pool_shares_value_delta)?;
    let owned_shares_amount_increase = checked_i64_to_u64(owned_shares_amount_delta)?;
    let owned_shares_value_increase = checked_i64_to_u64(owned_shares_value_delta)?;

    // Validate that the collateral value moved exactly to the correct place
    msg!("[mint_with_maple_pool_depository:check_amounts]");
    require!(
        user_collateral_amount_decrease == collateral_amount_deposited,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        pool_collateral_amount_increase == collateral_amount_deposited,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        pool_shares_value_increase == collateral_amount_deposited,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that we received the correct amount of shares
    require!(
        owned_shares_amount_increase == pool_shares_amount_increase,
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );

    // Check that the shares we received match the collateral value (allowing for precision loss)
    let single_share_value =
        ctx.accounts
            .compute_shares_value(1, pool_shares_amount_after, pool_shares_value_after)?;
    let allowed_precision_loss_amount = single_share_value
        .checked_add(1)
        .ok_or(UxdError::MathError)?;
    msg!(
        "[mint_with_maple_pool_depository:allow_precision_loss:{}]",
        allowed_precision_loss_amount
    );
    require!(
        is_equal_with_precision_loss(
            collateral_amount_deposited,
            owned_shares_value_increase,
            allowed_precision_loss_amount
        )?,
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );

    // Add minting fees on top of the received value we got from the pool
    msg!("[mint_with_maple_pool_depository:compute_redeemable]");
    let depository_minting_fee_in_bps = ctx.accounts.depository.load()?.minting_fee_in_bps;
    let redeemable_amount_before_fees: u64 = owned_shares_value_increase;
    let redeemable_amount_after_fees: u64 =
        calculate_amount_less_fees(redeemable_amount_before_fees, depository_minting_fee_in_bps)?;

    //  Redeemable amount should be positive
    require!(
        redeemable_amount_after_fees > 0,
        UxdError::MinimumMintedRedeemableAmountError
    );

    // Compute how much fees was paid
    msg!("[mint_with_maple_pool_depository:compute_fees]");
    let redeemable_amount_delta =
        compute_delta(redeemable_amount_before_fees, redeemable_amount_after_fees)?;
    let minting_fee_paid = checked_i64_to_u64(-redeemable_amount_delta)?;

    // Mint redeemable to the user
    msg!("[mint_with_maple_pool_depository:mint_redeemable]");
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_amount_after_fees,
    )?;

    // Emit event
    msg!("[mint_with_maple_pool_depository:emit_event]");
    emit!(MintWithMaplePoolDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        collateral_amount: collateral_amount_deposited,
        redeemable_amount: redeemable_amount_after_fees,
        minting_fee_paid: minting_fee_paid,
    });

    // Accouting for depository
    msg!("[mint_with_maple_pool_depository:accounting_depository]");
    let mut depository = ctx.accounts.depository.load_mut()?;
    depository.minting_fee_accrued(minting_fee_paid)?;
    depository.collateral_deposited_and_redeemable_minted(
        collateral_amount_deposited,
        redeemable_amount_after_fees,
    )?;

    // Accouting for controller
    msg!("[mint_with_maple_pool_depository:accounting_controller]");
    ctx.accounts
        .controller
        .load_mut()?
        .update_onchain_accounting_following_mint_or_redeem(redeemable_amount_after_fees.into())?;

    // Done
    Ok(())
}

// Into functions
impl<'info> MintWithMaplePoolDepository<'info> {
    pub fn into_deposit_collateral_to_maple_pool_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, syrup_cpi::cpi::accounts::LenderDeposit<'info>> {
        let cpi_accounts = syrup_cpi::cpi::accounts::LenderDeposit {
            globals: self.maple_globals.to_account_info(),
            pool: self.maple_pool.to_account_info(),
            pool_locker: self.maple_pool_locker.to_account_info(),
            lender: self.maple_lender.to_account_info(),
            lender_user: self.depository.to_account_info(),
            lender_locker: self.depository_collateral.to_account_info(),
            shares_mint: self.maple_shares_mint.to_account_info(),
            locked_shares: self.maple_locked_shares.to_account_info(),
            lender_shares: self.maple_lender_shares.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.syrup_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_user_collateral_to_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self.depository_collateral.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_mint_redeemable_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Compute maths functions
impl<'info> MintWithMaplePoolDepository<'info> {
    pub fn compute_owned_shares_amount(
        &self,
        locked_shares_amount: u64,
        lender_shares_amount: u64,
    ) -> Result<u64> {
        Ok(locked_shares_amount
            .checked_add(lender_shares_amount)
            .ok_or(UxdError::MathError)?)
    }

    // Precision loss may lower the returned owner value amount.
    // Precision loss of 1 native unit may be expected.
    pub fn compute_shares_value(
        &self,
        shares_amount: u64,
        pool_shares_amount: u64,
        pool_shares_value: u64,
    ) -> Result<u64> {
        if pool_shares_value == 0 {
            return Ok(0);
        }
        let shares_amount_fixed =
            I80F48::checked_from_num(shares_amount).ok_or(UxdError::MathError)?;
        let pool_shares_amount_fixed =
            I80F48::checked_from_num(pool_shares_amount).ok_or(UxdError::MathError)?;
        let pool_shares_value_fixed =
            I80F48::checked_from_num(pool_shares_value).ok_or(UxdError::MathError)?;
        let shares_value_fixed = shares_amount_fixed
            .checked_mul(pool_shares_value_fixed)
            .ok_or(UxdError::MathError)?
            .checked_div(pool_shares_amount_fixed)
            .ok_or(UxdError::MathError)?;
        Ok(shares_value_fixed
            .checked_to_num::<u64>()
            .ok_or(UxdError::MathError)?)
    }
}

// Validate
impl<'info> MintWithMaplePoolDepository<'info> {
    pub fn validate(&self, collateral_amount: u64) -> Result<()> {
        require!(collateral_amount > 0, UxdError::InvalidCollateralAmount);
        Ok(())
    }
}
