use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::error::UxdError;
use crate::events::ExchangeLiquidityWithCredixLpDepositoryEvent;
use crate::state::controller::Controller;
use crate::state::credix_lp_depository::CredixLpDepository;
use crate::state::identity_depository::IdentityDepository;
use crate::utils::checked_add;
use crate::utils::checked_as_u64;
use crate::utils::checked_sub;
use crate::utils::compute_decrease;
use crate::utils::compute_increase;
use crate::validate_is_program_frozen;
use crate::CONTROLLER_NAMESPACE;
use crate::CREDIX_LP_DEPOSITORY_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE;
use crate::IDENTITY_DEPOSITORY_NAMESPACE;

#[derive(Accounts)]
pub struct ExchangeLiquidityWithCredixLpDepository<'info> {
    /// #1
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #2
    pub user: Signer<'info>,

    /// #3
    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.load()?.bump,
        constraint = controller.load()?.identity_depository == identity_depository.key() @UxdError::InvalidDepository,
        constraint = controller.load()?.credix_lp_depository == credix_lp_depository.key() @UxdError::InvalidDepository,
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_NAMESPACE],
        bump = identity_depository.load()?.bump,
    )]
    pub identity_depository: AccountLoader<'info, IdentityDepository>,

    /// #5
    #[account(
        mut,
        seeds = [IDENTITY_DEPOSITORY_COLLATERAL_NAMESPACE],
        token::authority = identity_depository,
        token::mint = identity_depository.load()?.collateral_mint,
        bump = identity_depository.load()?.collateral_vault_bump,
    )]
    pub identity_depository_collateral: Box<Account<'info, TokenAccount>>,

    /// #6
    #[account(
        mut,
        seeds = [CREDIX_LP_DEPOSITORY_NAMESPACE, credix_lp_depository.load()?.credix_global_market_state.key().as_ref(), credix_lp_depository.load()?.collateral_mint.as_ref()],
        bump = credix_lp_depository.load()?.bump,
        has_one = controller @UxdError::InvalidController,
        has_one = collateral_mint @UxdError::InvalidCollateralMint,
        constraint = credix_lp_depository.load()?.depository_shares == credix_lp_depository_shares.key() @UxdError::InvalidDepositoryShares,
        constraint = credix_lp_depository.load()?.credix_shares_mint == credix_shares_mint.key() @UxdError::InvalidCredixSharesMint,
    )]
    pub credix_lp_depository: AccountLoader<'info, CredixLpDepository>,

    /// #7
    #[account(mut)]
    pub credix_lp_depository_shares: Box<Account<'info, TokenAccount>>,

    /// #8
    #[account(
        mut,
        token::mint = collateral_mint,
        token::authority = user,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #9
    #[account(
        mut,
        token::mint = credix_shares_mint,
        constraint = receiver_credix_shares.key() != credix_lp_depository_shares.key()
    )]
    pub receiver_credix_shares: Box<Account<'info, TokenAccount>>,

    /// #10
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #11
    pub credix_shares_mint: Box<Account<'info, Mint>>,

    /// #12
    pub system_program: Program<'info, System>,
    /// #13
    pub token_program: Program<'info, Token>,
}

pub(crate) fn handler(
    ctx: Context<ExchangeLiquidityWithCredixLpDepository>,
    collateral_amount: u64,
) -> Result<()> {
    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Fetch all current onchain state
    // -- Compute the amount of shares and collateral exchanged
    // ---------------------------------------------------------------------

    // Read all states before exchange
    let identity_depository_collateral_amount_before =
        ctx.accounts.identity_depository_collateral.amount;
    let credix_lp_depository_shares_amount_before = ctx.accounts.credix_lp_depository_shares.amount;
    let user_collateral_amount_before = ctx.accounts.user_collateral.amount;
    let receiver_credix_shares_amount_before = ctx.accounts.receiver_credix_shares.amount;

    // Compute the amount of shares we will exchange for the provided collateral
    let redeemable_amount_under_management = ctx
        .accounts
        .credix_lp_depository
        .load()?
        .redeemable_amount_under_management;
    let available_shares_amount = ctx.accounts.credix_lp_depository_shares.amount;
    let exchanged_shares_amount = checked_as_u64(
        u128::from(available_shares_amount) * u128::from(collateral_amount)
            / redeemable_amount_under_management,
    )?;

    msg!(
        "[redeemable_amount_under_management:{}]",
        redeemable_amount_under_management
    );
    msg!("[collateral_amount:{}]", collateral_amount);
    msg!("[available_shares_amount:{}]", available_shares_amount);
    msg!("[exchanged_shares_amount:{}]", exchanged_shares_amount);

    // Check the amount swapped is non-zero
    require!(
        exchanged_shares_amount > 0,
        UxdError::InvalidCollateralAmount
    );

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Actually runs the onchain mutation based on computed parameters
    // ---------------------------------------------------------------------

    // Make depository signer
    let credix_global_market_state = ctx
        .accounts
        .credix_lp_depository
        .load()?
        .credix_global_market_state;
    let collateral_mint = ctx.accounts.credix_lp_depository.load()?.collateral_mint;
    let credix_lp_depository_pda_signer: &[&[&[u8]]] = &[&[
        CREDIX_LP_DEPOSITORY_NAMESPACE,
        credix_global_market_state.as_ref(),
        collateral_mint.as_ref(),
        &[ctx.accounts.credix_lp_depository.load()?.bump],
    ]];

    msg!("[collateral_transfer:{}]", collateral_amount);
    token::transfer(
        ctx.accounts
            .into_transfer_user_collateral_to_identity_depository_collateral_context(),
        collateral_amount,
    )?;

    msg!("[shares_transfer:{}]", exchanged_shares_amount);
    token::transfer(
        ctx.accounts
            .into_transfer_credix_lp_depository_shares_to_receiver_credix_shares_context()
            .with_signer(credix_lp_depository_pda_signer),
        exchanged_shares_amount,
    )?;

    // Refresh account states after withdrawal
    ctx.accounts.identity_depository_collateral.reload()?;
    ctx.accounts.credix_lp_depository_shares.reload()?;
    ctx.accounts.user_collateral.reload()?;
    ctx.accounts.receiver_credix_shares.reload()?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Strictly verify that the onchain state
    // -- after mutation exactly match previous predictions
    // ---------------------------------------------------------------------

    // Read all states after exchange
    let identity_depository_collateral_amount_after =
        ctx.accounts.identity_depository_collateral.amount;
    let credix_lp_depository_shares_amount_after = ctx.accounts.credix_lp_depository_shares.amount;
    let user_collateral_amount_after = ctx.accounts.user_collateral.amount;
    let receiver_credix_shares_amount_after = ctx.accounts.receiver_credix_shares.amount;

    // Compute changes in states
    let identity_depository_collateral_amount_increase = compute_increase(
        identity_depository_collateral_amount_before,
        identity_depository_collateral_amount_after,
    )?;
    let credix_lp_depository_shares_amount_decrease = compute_decrease(
        credix_lp_depository_shares_amount_before,
        credix_lp_depository_shares_amount_after,
    )?;
    let user_collateral_amount_decrease =
        compute_decrease(user_collateral_amount_before, user_collateral_amount_after)?;
    let receiver_credix_shares_amount_increase = compute_increase(
        receiver_credix_shares_amount_before,
        receiver_credix_shares_amount_after,
    )?;

    // Log deltas for debriefing the changes
    msg!(
        "[identity_depository_collateral_amount_increase:{}]",
        identity_depository_collateral_amount_increase
    );
    msg!(
        "[credix_lp_depository_shares_amount_decrease:{}]",
        credix_lp_depository_shares_amount_decrease
    );
    msg!(
        "[user_collateral_amount_decrease:{}]",
        user_collateral_amount_decrease
    );
    msg!(
        "[receiver_credix_shares_amount_increase:{}]",
        receiver_credix_shares_amount_increase
    );

    // Fact check
    require!(
        identity_depository_collateral_amount_increase == collateral_amount,
        UxdError::CollateralDepositAmountsDoesntMatch
    );
    require!(
        credix_lp_depository_shares_amount_decrease == exchanged_shares_amount,
        UxdError::CollateralDepositDoesntMatchTokenValue
    );
    require!(
        user_collateral_amount_decrease == collateral_amount,
        UxdError::CollateralDepositAmountsDoesntMatch
    );
    require!(
        receiver_credix_shares_amount_increase == exchanged_shares_amount,
        UxdError::CollateralDepositDoesntMatchTokenValue
    );

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Emit resulting event, and update onchain accounting
    // ---------------------------------------------------------------------

    // Emit event
    emit!(ExchangeLiquidityWithCredixLpDepositoryEvent {
        controller_version: ctx.accounts.controller.load()?.version,
        depository_version: ctx.accounts.credix_lp_depository.load()?.version,
        controller: ctx.accounts.controller.key(),
        depository: ctx.accounts.credix_lp_depository.key(),
        collateral_amount,
        shares_amount: exchanged_shares_amount,
    });

    // Accounting for identity_depository
    let mut identity_depository = ctx.accounts.identity_depository.load_mut()?;
    identity_depository.collateral_amount_deposited = checked_add(
        identity_depository.collateral_amount_deposited,
        collateral_amount.into(),
    )?;
    identity_depository.redeemable_amount_under_management = checked_add(
        identity_depository.redeemable_amount_under_management,
        collateral_amount.into(),
    )?;

    // Accounting for credix_lp_depository
    let mut credix_lp_depository = ctx.accounts.credix_lp_depository.load_mut()?;
    credix_lp_depository.collateral_amount_deposited = checked_sub(
        credix_lp_depository.collateral_amount_deposited,
        collateral_amount.into(),
    )?;
    credix_lp_depository.redeemable_amount_under_management = checked_sub(
        credix_lp_depository.redeemable_amount_under_management,
        collateral_amount.into(),
    )?;

    // Done
    Ok(())
}

// Into functions
impl<'info> ExchangeLiquidityWithCredixLpDepository<'info> {
    pub fn into_transfer_user_collateral_to_identity_depository_collateral_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self.identity_depository_collateral.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
    pub fn into_transfer_credix_lp_depository_shares_to_receiver_credix_shares_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.credix_lp_depository_shares.to_account_info(),
            to: self.receiver_credix_shares.to_account_info(),
            authority: self.credix_lp_depository.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Validate
impl<'info> ExchangeLiquidityWithCredixLpDepository<'info> {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        Ok(())
    }
}
