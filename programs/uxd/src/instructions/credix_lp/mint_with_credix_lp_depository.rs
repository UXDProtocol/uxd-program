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
use crate::events::MintWithCredixLpDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::utils::calculate_amount_less_fees;
use crate::utils::checked_i64_to_u64;
use crate::utils::compute_delta;
use crate::utils::is_equal_with_precision_loss;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
#[instruction(collateral_amount: u64)]
pub struct MintWithCredixLpDepository<'info> {
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
        constraint = controller.load()?.registered_credix_lp_depositories.contains(&depository.key()) @UxdError::InvalidDepository,
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
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
        has_one = credix_global_market_state @UxdError::InvalidCredixGlobalMarketState,
        has_one = credix_signing_authority @UxdError::InvalidCredixSigningAuthority,
        has_one = credix_treasury_collateral @UxdError::InvalidCredixTreasuryCollateral,
        has_one = credix_liquidity_collateral @UxdError::InvalidCredixLiquidityCollateral,
        has_one = credix_lp_shares_mint @UxdError::InvalidCredixLpSharesMint,
        has_one = credix_locker_lp_shares @UxdError::InvalidCredixLockerLpShares,
    )]
    pub depository: AccountLoader<'info, CredixLpDepository>,

    /// #5
    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #6
    #[account()]
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
    pub credix_global_market_state: Box<Account<'info, syrup_cpi::Pool>>,
    /// #11
    #[account(mut)]
    pub credix_signing_authority: Box<Account<'info, TokenAccount>>,
    /// #12
    #[account(mut)]
    pub credix_treasury_collateral: Box<Account<'info, TokenAccount>>,
    /// #13
    #[account(mut)]
    pub credix_liquidity_collateral: Box<Account<'info, TokenAccount>>,
    /// #14
    #[account(mut)]
    pub credix_lp_shares_mint: Box<Account<'info, Mint>>,
    /// #15
    #[account(mut)]
    pub credix_locker_lp_shares: Box<Account<'info, TokenAccount>>,
    /// #16 // TODO
    #[account(mut)]
    pub credix_pass: Box<Account<'info, TokenAccount>>,

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

pub fn handler(ctx: Context<MintWithCredixLpDepository>, collateral_amount: u64) -> Result<()> {
    // Read useful keys
    let credix_global_market_state = ctx.accounts.depository.load()?.credix_global_market_state;
    let collateral_mint = ctx.accounts.depository.load()?.collateral_mint;

    // Make controller signer
    let controller_pda_signer: &[&[&[u8]]] = &[&[
        CONTROLLER_NAMESPACE,
        &[ctx.accounts.controller.load()?.bump],
    ]];

    // Make depository signer
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        CREDIX_LP_DEPOSITORY_NAMESPACE,
        credix_global_market_state.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.load()?.bump],
    ]];

    // Read all state before deposit
    let depository_collateral_amount_before: u64 = ctx.accounts.depository_collateral.amount;
    let user_collateral_amount_before: u64 = ctx.accounts.user_collateral.amount;
    let liquidity_collateral_amount_before: u64 = ctx.accounts.credix_liquidity_collateral.amount;
    let outstanding_collateral_amount_before: u64 = ctx
        .accounts
        .credix_global_market_state
        .pool_outstanding_credit;
    let lp_shares_supply_before: u64 = ctx.accounts.credix_lp_shares_mint.supply;
    let owned_lp_shares_amount_before: u64 = ctx.accounts.credix_locker_lp_shares.amount;
    let owned_lp_shares_value_before: u64 = ctx.accounts.compute_lp_shares_value(
        owned_lp_shares_amount_before,
        lp_shares_supply_before,
        liquidity_collateral_amount_before,
        outstanding_collateral_amount_before,
    )?;

    // Add some pool state log information
    msg!(
        "[mint_with_credix_lp_depository:before:liquidity_collateral_amount:{}]",
        liquidity_collateral_amount_before
    );
    msg!(
        "[mint_with_credix_lp_depository:before:outstanding_collateral_amount:{}]",
        outstanding_collateral_amount_before
    );
    msg!(
        "[mint_with_credix_lp_depository:before:lp_shares_supply:{}]",
        lp_shares_supply_before
    );
    msg!(
        "[mint_with_credix_lp_depository:before:owned_lp_shares_amount:{}]",
        owned_lp_shares_amount_before
    );
    msg!(
        "[mint_with_credix_lp_depository:before:owned_lp_shares_value:{}]",
        owned_lp_shares_value_before
    );

    msg!(
        "[mint_with_credix_lp_depository:deposit:{}]",
        collateral_amount
    );

    // Transfer the collateral to an account owned by the depository
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_depository_collateral_context(),
        collateral_amount,
    )?;

    // Do the deposit by placing collateral owned by the depository into the pool
    syrup_cpi::cpi::lender_deposit(
        ctx.accounts
            .into_deposit_collateral_to_credix_lp_context()
            .with_signer(depository_pda_signer),
        collateral_amount,
    )?;

    // Refresh account states after deposit
    ctx.accounts.depository_collateral.reload()?;
    ctx.accounts.user_collateral.reload()?;
    ctx.accounts.credix_global_market_state.reload()?;
    ctx.accounts.credix_liquidity_collateral.reload()?;
    ctx.accounts.credix_lp_shares_mint.reload()?;
    ctx.accounts.credix_locker_lp_shares.reload()?;

    // Read all states after deposit
    let depository_collateral_amount_after: u64 = ctx.accounts.depository_collateral.amount;
    let user_collateral_amount_after: u64 = ctx.accounts.user_collateral.amount;
    let liquidity_collateral_amount_after: u64 = ctx.accounts.credix_liquidity_collateral.amount;
    let outstanding_collateral_amount_after: u64 = ctx
        .accounts
        .credix_global_market_state
        .pool_outstanding_credit;
    let lp_shares_supply_after: u64 = ctx.accounts.credix_lp_shares_mint.supply;
    let owned_lp_shares_amount_after: u64 = ctx.accounts.credix_locker_lp_shares.amount;
    let owned_lp_shares_value_after: u64 = ctx.accounts.compute_lp_shares_value(
        owned_lp_shares_amount_after,
        lp_shares_supply_after,
        liquidity_collateral_amount_after,
        outstanding_collateral_amount_after,
    )?;

    // Add some pool state log information
    msg!(
        "[mint_with_credix_lp_depository:after:liquidity_collateral_amount:{}]",
        liquidity_collateral_amount_after
    );
    msg!(
        "[mint_with_credix_lp_depository:after:outstanding_collateral_amount:{}]",
        outstanding_collateral_amount_after
    );
    msg!(
        "[mint_with_credix_lp_depository:after:lp_shares_supply:{}]",
        lp_shares_supply_after
    );
    msg!(
        "[mint_with_credix_lp_depository:after:owned_lp_shares_amount:{}]",
        owned_lp_shares_amount_after
    );
    msg!(
        "[mint_with_credix_lp_depository:after:owned_lp_shares_value:{}]",
        owned_lp_shares_value_after
    );

    // Compute changes in states
    let depository_collateral_delta: i64 = compute_delta(
        depository_collateral_amount_before,
        depository_collateral_amount_after,
    )?;
    let user_collateral_amount_delta: i64 =
        compute_delta(user_collateral_amount_before, user_collateral_amount_after)?;
    let liquidity_collateral_amount_delta: i64 = compute_delta(
        liquidity_collateral_amount_before,
        liquidity_collateral_amount_after,
    )?;
    let outstanding_collateral_amount_delta: i64 = compute_delta(
        outstanding_collateral_amount_before,
        outstanding_collateral_amount_after,
    )?;
    let lp_shares_supply_delta: i64 =
        compute_delta(lp_shares_supply_before, lp_shares_supply_after)?;
    let owned_lp_shares_amount_delta: i64 =
        compute_delta(owned_lp_shares_amount_before, owned_lp_shares_amount_after)?;
    let owned_lp_shares_value_delta: i64 =
        compute_delta(owned_lp_shares_value_before, owned_lp_shares_value_after)?;

    // The depository collateral account should always be empty
    require!(
        depository_collateral_delta == 0,
        UxdError::CollateralDepositHasRemainingDust
    );

    // Validate the deposit was successful and meaningful
    require!(
        user_collateral_amount_delta < 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        liquidity_collateral_amount_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        outstanding_collateral_amount_delta == 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        lp_shares_supply_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        owned_lp_shares_amount_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );
    require!(
        owned_lp_shares_value_delta > 0,
        UxdError::CollateralDepositUnaccountedFor
    );

    // Because we know the direction of the change, we can use the unsigned values now
    let user_collateral_amount_decrease = checked_i64_to_u64(-user_collateral_amount_delta)?;
    let liquidity_collateral_amount_increase =
        checked_i64_to_u64(liquidity_collateral_amount_delta)?;
    let lp_shares_supply_increase = checked_i64_to_u64(lp_shares_supply_delta)?;
    let owned_lp_shares_amount_increase = checked_i64_to_u64(owned_lp_shares_amount_delta)?;
    let owned_lp_shares_value_increase = checked_i64_to_u64(owned_lp_shares_value_delta)?;

    // Validate that the collateral value moved exactly to the correct place
    require!(
        user_collateral_amount_decrease == collateral_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        liquidity_collateral_amount_increase == collateral_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that we received the correct amount of shares
    require!(
        owned_lp_shares_amount_increase == lp_shares_supply_increase,
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );

    // Check that the shares we received match the collateral value (allowing for precision loss)
    let single_share_value = ctx.accounts.compute_lp_shares_value(
        1,
        lp_shares_supply_after,
        liquidity_collateral_amount_after,
        outstanding_collateral_amount_after,
    )?;
    let allowed_precision_loss_amount = single_share_value
        .checked_add(2)
        .ok_or(UxdError::MathError)?;
    msg!(
        "[mint_with_credix_lp_depository:allowed_precision_loss:{}]",
        allowed_precision_loss_amount
    );
    require!(
        is_equal_with_precision_loss(
            collateral_amount,
            owned_lp_shares_value_increase,
            allowed_precision_loss_amount
        )?,
        UxdError::CollateralDepositDoesntMatchTokenValue,
    );

    // Add minting fees on top of the received value we got from the pool
    let depository_minting_fee_in_bps = ctx.accounts.depository.load()?.minting_fee_in_bps;
    let redeemable_amount_before_fees: u64 = owned_lp_shares_value_increase;
    let redeemable_amount_after_fees: u64 =
        calculate_amount_less_fees(redeemable_amount_before_fees, depository_minting_fee_in_bps)?;

    //  Redeemable amount should be positive
    require!(
        redeemable_amount_after_fees > 0,
        UxdError::MinimumMintedRedeemableAmountError
    );

    // Compute how much fees was paid
    let redeemable_amount_delta =
        compute_delta(redeemable_amount_before_fees, redeemable_amount_after_fees)?;
    let minting_fee_paid = checked_i64_to_u64(-redeemable_amount_delta)?;

    // Mint redeemable to the user
    msg!(
        "[mint_with_credix_lp_depository:mint_redeemable:{}]",
        redeemable_amount_after_fees
    );
    token::mint_to(
        ctx.accounts
            .into_mint_redeemable_context()
            .with_signer(controller_pda_signer),
        redeemable_amount_after_fees,
    )?;

    // Emit event
    emit!(MintWithCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        collateral_amount: collateral_amount,
        redeemable_amount: redeemable_amount_after_fees,
        minting_fee_paid: minting_fee_paid,
    });

    // Accouting for depository
    let mut depository = ctx.accounts.depository.load_mut()?;
    depository.minting_fee_accrued(minting_fee_paid)?;
    depository.collateral_deposited_and_redeemable_minted(
        collateral_amount,
        redeemable_amount_after_fees,
    )?;

    // Accouting for controller
    ctx.accounts
        .controller
        .load_mut()?
        .update_onchain_accounting_following_mint_or_redeem(redeemable_amount_after_fees.into())?;

    // Done
    Ok(())
}

// Into functions
impl<'info> MintWithCredixLpDepository<'info> {
    pub fn into_deposit_collateral_to_credix_lp_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, syrup_cpi::cpi::accounts::LenderDeposit<'info>> {
        let cpi_accounts = syrup_cpi::cpi::accounts::LenderDeposit {
            globals: self.credix_globals.to_account_info(),
            pool: self.credix_lp.to_account_info(),
            pool_locker: self.credix_lp_locker.to_account_info(),
            lender: self.credix_lender.to_account_info(),
            lender_user: self.depository.to_account_info(),
            lender_locker: self.depository_collateral.to_account_info(),
            shares_mint: self.credix_shares_mint.to_account_info(),
            locked_shares: self.credix_locked_shares.to_account_info(),
            lender_shares: self.credix_lender_shares.to_account_info(),
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
impl<'info> MintWithCredixLpDepository<'info> {
    // Precision loss may lower the returned owner value amount.
    // Precision loss of 1 native unit may be expected.
    pub fn compute_lp_shares_value(
        &self,
        lp_shares_amount: u64,
        lp_shares_supply: u64,
        liquidity_collateral_amount: u64,
        outstanding_collateral_amount: u64,
    ) -> Result<u64> {
        if lp_shares_amount == 0 {
            return Ok(0);
        }
        let lp_shares_amount_fixed =
            I80F48::checked_from_num(lp_shares_amount).ok_or(UxdError::MathError)?;
        let lp_shares_supply_fixed =
            I80F48::checked_from_num(lp_shares_supply).ok_or(UxdError::MathError)?;
        let liquidity_collateral_amount_fixed =
            I80F48::checked_from_num(liquidity_collateral_amount).ok_or(UxdError::MathError)?;
        let outstanding_collateral_amount_fixed =
            I80F48::checked_from_num(outstanding_collateral_amount).ok_or(UxdError::MathError)?;
        let total_collateral_amount_fixed = liquidity_collateral_amount_fixed
            .checked_add(outstanding_collateral_amount_fixed)
            .ok_or(UxdError::MathError)?;
        let lp_shares_value_fixed = lp_shares_amount_fixed
            .checked_mul(total_collateral_amount_fixed)
            .ok_or(UxdError::MathError)?
            .checked_div(lp_shares_supply_fixed)
            .ok_or(UxdError::MathError)?;
        Ok(lp_shares_value_fixed
            .checked_to_num::<u64>()
            .ok_or(UxdError::MathError)?)
    }
}

// Validate
impl<'info> MintWithCredixLpDepository<'info> {
    pub fn validate(&self, collateral_amount: u64) -> Result<()> {
        require!(collateral_amount > 0, UxdError::InvalidCollateralAmount);
        Ok(())
    }
}
