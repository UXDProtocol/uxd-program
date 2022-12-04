use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::error::UxdError;
use crate::events::RedeemFromCredixLpDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::utils::calculate_amount_less_fees;
use crate::utils::compute_decrease;
use crate::utils::compute_increase;
use crate::utils::compute_shares_amount_for_value;
use crate::utils::compute_value_for_shares_amount;
use crate::utils::is_within_range_inclusive;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
#[instruction(redeemable_amount: u64)]
pub struct RedeemFromCredixLpDepository<'info> {
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
        has_one = depository_collateral @UxdError::InvalidDepositoryCollateral,
        has_one = depository_shares @UxdError::InvalidDepositoryShares,
        has_one = credix_program_state @UxdError::InvalidCredixProgramState,
        has_one = credix_global_market_state @UxdError::InvalidCredixGlobalMarketState,
        has_one = credix_signing_authority @UxdError::InvalidCredixSigningAuthority,
        has_one = credix_liquidity_collateral @UxdError::InvalidCredixLiquidityCollateral,
        has_one = credix_shares_mint @UxdError::InvalidCredixSharesMint,
    )]
    pub depository: AccountLoader<'info, CredixLpDepository>,

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
    pub depository_shares: Box<Account<'info, TokenAccount>>,

    /// #11
    #[account(
        has_one = credix_multisig_key @UxdError::InvalidCredixMultisig,
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
    pub credix_multisig_key: AccountInfo<'info>,

    /// #19
    #[account(
        mut,
        token::authority = credix_multisig_key,
        token::mint = collateral_mint,
    )]
    pub credix_multisig_collateral: Box<Account<'info, TokenAccount>>,

    /// #20
    pub system_program: Program<'info, System>,
    /// #21
    pub token_program: Program<'info, Token>,
    /// #22
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// #23
    pub credix_program: Program<'info, credix_client::program::Credix>,
    /// #24
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RedeemFromCredixLpDepository>, redeemable_amount: u64) -> Result<()> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Fetch all current onchain state
    // -- and predict all future final state after mutation
    // ---------------------------------------------------------------------

    // Read all states before withdrawal
    let depository_collateral_amount_before: u64 = ctx.accounts.depository_collateral.amount;
    let user_collateral_amount_before: u64 = ctx.accounts.user_collateral.amount;
    let user_redeemable_amount_before: u64 = ctx.accounts.user_redeemable.amount;

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

    // Initial amount
    msg!(
        "[redeem_from_credix_lp_depository:redeemable_amount:{}]",
        redeemable_amount
    );

    // Apply the redeeming fees
    let redeemable_amount_after_fees: u64 = calculate_amount_less_fees(
        redeemable_amount,
        ctx.accounts.depository.load()?.redeeming_fee_in_bps,
    )?;
    msg!(
        "[redeem_from_credix_lp_depository:redeemable_amount_after_fees:{}]",
        redeemable_amount_after_fees
    );

    // Assumes and enforce a collateral/redeemable 1:1 relationship on purpose
    let collateral_amount_before_precision_loss: u64 = redeemable_amount_after_fees;

    // Compute the amount of shares that we need to withdraw based on the amount of wanted collateral
    let shares_amount: u64 = compute_shares_amount_for_value(
        collateral_amount_before_precision_loss,
        total_shares_amount_before,
        total_shares_value_before,
    )?;
    msg!(
        "[redeem_from_credix_lp_depository:shares_amount:{}]",
        shares_amount
    );

    // Compute the amount of collateral that the withdrawn shares are worth (after potential precision loss)
    let collateral_amount_after_precision_loss: u64 = compute_value_for_shares_amount(
        shares_amount,
        total_shares_amount_before,
        total_shares_value_before,
    )?;
    msg!(
        "[redeem_from_credix_lp_depository:collateral_amount_after_precision_loss:{}]",
        collateral_amount_after_precision_loss
    );
    require!(
        collateral_amount_after_precision_loss > 0,
        UxdError::MinimumRedeemedCollateralAmountError
    );

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Actually runs the onchain mutation based on computed parameters
    // ---------------------------------------------------------------------

    // Make depository signer
    let credix_global_market_state = ctx.accounts.depository.load()?.credix_global_market_state;
    let collateral_mint = ctx.accounts.depository.load()?.collateral_mint;
    let depository_pda_signer: &[&[&[u8]]] = &[&[
        CREDIX_LP_DEPOSITORY_NAMESPACE,
        credix_global_market_state.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.depository.load()?.bump],
    ]];

    // Burn the user's redeemable
    msg!("[redeem_from_credix_lp_depository:redeemable_burn]",);
    token::burn(
        ctx.accounts.into_burn_redeemable_context(),
        redeemable_amount,
    )?;

    // Run a withdraw CPI from credix into the depository
    msg!("[redeem_from_credix_lp_depository:withdraw_funds]",);
    credix_client::cpi::withdraw_funds(
        ctx.accounts
            .into_withdraw_funds_from_credix_lp_context()
            .with_signer(depository_pda_signer),
        collateral_amount_before_precision_loss,
    )?;

    // Transfer the received collateral from the depository to the end user
    msg!("[redeem_from_credix_lp_depository:collateral_transfer]",);
    token::transfer(
        ctx.accounts
            .into_transfer_depository_collateral_to_user_collateral_context()
            .with_signer(depository_pda_signer),
        collateral_amount_after_precision_loss,
    )?;

    // Refresh account states after deposit
    ctx.accounts.depository_collateral.reload()?;
    ctx.accounts.depository_shares.reload()?;
    ctx.accounts.user_collateral.reload()?;
    ctx.accounts.user_redeemable.reload()?;
    ctx.accounts.credix_global_market_state.reload()?;
    ctx.accounts.credix_liquidity_collateral.reload()?;
    ctx.accounts.credix_shares_mint.reload()?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Strictly verify that the onchain state
    // -- after mutation exactly match previous predictions
    // ---------------------------------------------------------------------

    // Read all states after withdrawal
    let depository_collateral_amount_after: u64 = ctx.accounts.depository_collateral.amount;
    let user_collateral_amount_after: u64 = ctx.accounts.user_collateral.amount;
    let user_redeemable_amount_after: u64 = ctx.accounts.user_redeemable.amount;

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
    let user_collateral_amount_increase: u64 =
        compute_increase(user_collateral_amount_before, user_collateral_amount_after)?;
    let user_redeemable_amount_decrease: u64 =
        compute_decrease(user_redeemable_amount_before, user_redeemable_amount_after)?;

    let total_shares_amount_decrease: u64 =
        compute_decrease(total_shares_amount_before, total_shares_amount_after)?;
    let total_shares_value_decrease: u64 =
        compute_decrease(total_shares_value_before, total_shares_value_after)?;

    let owned_shares_amount_decrease: u64 =
        compute_decrease(owned_shares_amount_before, owned_shares_amount_after)?;
    let owned_shares_value_decrease: u64 =
        compute_decrease(owned_shares_value_before, owned_shares_value_after)?;

    // Log deltas for debriefing the changes
    msg!(
        "[redeem_from_credix_lp_depository:user_collateral_amount_increase:{}]",
        user_collateral_amount_increase
    );
    msg!(
        "[redeem_from_credix_lp_depository:user_redeemable_amount_decrease:{}]",
        user_redeemable_amount_decrease
    );
    msg!(
        "[redeem_from_credix_lp_depository:total_shares_amount_decrease:{}]",
        total_shares_amount_decrease
    );
    msg!(
        "[redeem_from_credix_lp_depository:total_shares_value_decrease:{}]",
        total_shares_value_decrease
    );
    msg!(
        "[redeem_from_credix_lp_depository:owned_shares_amount_decrease:{}]",
        owned_shares_amount_decrease
    );
    msg!(
        "[redeem_from_credix_lp_depository:owned_shares_value_decrease:{}]",
        owned_shares_value_decrease
    );

    // The depository collateral account should always be empty
    require!(
        depository_collateral_amount_before == depository_collateral_amount_after,
        UxdError::CollateralDepositHasRemainingDust
    );

    // Validate that the locked value moved exactly to the correct place
    require!(
        user_collateral_amount_increase == collateral_amount_after_precision_loss,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        user_redeemable_amount_decrease == redeemable_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that we withdrew the correct amount of shares
    require!(
        total_shares_amount_decrease == shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        owned_shares_amount_decrease == shares_amount,
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // Check that the new state of the pool still makes sense
    require!(
        is_within_range_inclusive(
            total_shares_value_decrease,
            collateral_amount_after_precision_loss,
            collateral_amount_before_precision_loss,
        ),
        UxdError::CollateralDepositAmountsDoesntMatch,
    );
    require!(
        is_within_range_inclusive(
            owned_shares_value_decrease,
            collateral_amount_after_precision_loss,
            collateral_amount_before_precision_loss,
        ),
        UxdError::CollateralDepositAmountsDoesntMatch,
    );

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Emit resulting event, and update onchain accounting
    // ---------------------------------------------------------------------

    // Compute how much fees was paid
    let redeeming_fee_paid: u64 =
        compute_decrease(redeemable_amount, redeemable_amount_after_fees)?;

    // Emit event
    emit!(RedeemFromCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.depository.key(),
        user: ctx.accounts.user.key(),
        redeemable_amount: redeemable_amount,
        collateral_amount_before_fees: collateral_amount_before_precision_loss,
        collateral_amount_after_fees: collateral_amount_after_precision_loss,
        redeeming_fee_paid: redeeming_fee_paid,
    });

    // Accouting for depository
    let mut depository = ctx.accounts.depository.load_mut()?;
    depository.redeeming_fee_accrued(redeeming_fee_paid)?;
    depository.collateral_withdrawn_and_redeemable_burned(
        collateral_amount_before_precision_loss,
        redeemable_amount,
    )?;

    // Accouting for controller
    let redeemable_amount_change: i128 = -i128::from(redeemable_amount);
    let mut controller = ctx.accounts.controller.load_mut()?;
    controller.update_onchain_accounting_following_mint_or_redeem(redeemable_amount_change)?;

    // Done
    Ok(())
}

// Into functions
impl<'info> RedeemFromCredixLpDepository<'info> {
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
            credix_multisig_key: self.credix_multisig_key.to_account_info(),
            credix_multisig_token_account: self.credix_multisig_collateral.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.credix_program.to_account_info();
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

// Validate
impl<'info> RedeemFromCredixLpDepository<'info> {
    pub fn validate(&self, redeemable_amount: u64) -> Result<()> {
        require!(redeemable_amount > 0, UxdError::InvalidRedeemableAmount);
        Ok(())
    }
}
