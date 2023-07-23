use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::error::UxdError;
use crate::state::LsdDepository;
use crate::state::LsdPosition;
use crate::utils::validate_collateral_amount;
use crate::utils::validate_loan_to_value_bps;
use crate::validate_is_program_frozen;
use crate::Controller;
use crate::CONTROLLER_NAMESPACE;
use crate::LSD_DEPOSITORY_NAMESPACE;
use crate::LSD_POSITION_SPACE;
use crate::LSD_PROFITS_TOKEN_ACCOUNT_NAMESPACE;

#[derive(Accounts)]
pub struct BorrowFromLsdDepository<'info> {
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
        has_one = redeemable_mint @UxdError::InvalidRedeemableMint
    )]
    pub controller: AccountLoader<'info, Controller>,

    /// #4
    #[account(
        mut,
        seeds = [LSD_DEPOSITORY_NAMESPACE, collateral_mint.key().as_ref()],
        bump = depository.load()?.bump,
        has_one = collateral_mint @UxdError::InvalidCollateralMint
    )]
    pub depository: AccountLoader<'info, LsdDepository>,

    /// #5
    #[account(
        mut,
        seeds = [LSD_PROFITS_TOKEN_ACCOUNT_NAMESPACE, depository.key().as_ref()],
        token::authority = depository,
        token::mint = liquidation_mint,
        bump = depository.load()?.profits_token_account_bump,
    )]
    pub profits_token_account: Account<'info, TokenAccount>,

    /// #6
    #[account(
        mut,
        constraint = user_collateral.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_collateral.mint == collateral_mint.key() @UxdError::InvalidCollateralMint,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    /// #
    #[account(
        mut,
        constraint = user_redeemable.owner == user.key() @UxdError::InvalidOwner,
        constraint = user_redeemable.mint == redeemable_mint.key() @UxdError::InvalidRedeemableMint,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    /// #
    /// Tracks the user overall position in the LSD Depository (one per user, per LSD Depository).
    #[account(
        init_if_needed,
        seeds = [depository.key().as_ref(), user.key().as_ref()],
        space = LSD_POSITION_SPACE,       
        payer = payer,
        bump,
    )]
    pub lsd_position: AccountLoader<'info, LsdPosition>,

    /// #
    /// PDA holding the deposited collateral on behalf of the lsd_position
    #[account(
        init_if_needed,
        seeds = [lsd_position.key().as_ref()],
        token::authority = lsd_position,
        token::mint = collateral_mint,
        payer = payer,
        bump,
    )]
    pub lsd_position_collateral_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub depository_lp_token_vault: Box<Account<'info, TokenAccount>>,

    /// #
    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// #
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// #
    pub liquidation_mint: Box<Account<'info, Mint>>,

    /// #
    pub system_program: Program<'info, System>,

    /// #
    pub token_program: Program<'info, Token>,

    /// #
    pub uxd_program: Program<'info, crate::program::Uxd>,

    /// #
    pub rent: Sysvar<'info, Rent>,
}

pub(crate) fn handler(ctx: Context<BorrowFromLsdDepository>, collateral_amount: u64, loan_to_value_bps: u8) -> Result<()> {
    // - validate inputs
    // - move user collateral to the lsd_position_collateral_token_account
    // - mint redeemable tokens to the user according to LTV to user_redeemable

    // - find out liquidation price
    // - - setup/update liquidation CRON at price
    // - - setup/update either another cron or in previous cron: liquidate when running out of funding

    // - update lsd_position (to reflect a position growth or a new one)
    // - update lsd_depository
    // - update accounting

    Ok(())
}

// Validate
impl<'info> BorrowFromLsdDepository<'info> {
    pub(crate) fn validate(&self, collateral_amount: u64, loan_to_value_bps: u8) -> Result<()> {
        validate_is_program_frozen(self.controller.load()?)?;
        validate_collateral_amount(&self.user_collateral, collateral_amount)?;
        validate_loan_to_value_bps(loan_to_value_bps, self.depository.load()?.loan_to_value_bps)?;
        Ok(())
    }
}
